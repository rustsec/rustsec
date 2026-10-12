use crate::{error::Error, package::Name};
use curl::{easy, multi};
use tame_index::{
    external::http::{self, header},
    utils::flock::FileLock,
};

macro_rules! curl_err {
    ($err:expr) => {
        $crate::error::Error::from_source($crate::error::ErrorKind::Registry, $err)
    };
}

macro_rules! try_curl {
    ($e:expr) => {
        match $e {
            Ok(r) => r,
            Err(err) => {
                return Err(curl_err!(err));
            }
        }
    };
}

macro_rules! try_tame {
    ($e:expr) => {
        match $e {
            Ok(r) => r,
            Err(err) => return Err($crate::Error::from_tame(err)),
        }
    };
}

/// Combines a [`tame_index::SparseIndex`] with an HTTP client for updating the local cache with the latest from the remote index
pub struct RemoteSparseIndex {
    pub(super) index: tame_index::SparseIndex,
    user_agent: String,
}

impl RemoteSparseIndex {
    /// Crates a new [`Self`] for the provided index, using the specified user agent when making remote requests
    pub fn new(index: tame_index::SparseIndex, user_agent: String) -> Result<Self, Error> {
        Ok(Self { index, user_agent })
    }

    /// Attempts to fetch the metadata from the index for all of the specified crate names
    ///
    /// The timeout is on a per-crate basis
    pub fn multi_fetch(
        &self,
        packages: std::collections::BTreeSet<Name>,
        timeout: Option<std::time::Duration>,
        lock: &FileLock,
    ) -> Result<super::MultiFetchResult, Error> {
        let mut results = Vec::with_capacity(packages.len());

        let (tx, rx) = crossbeam_channel::bounded::<(Name, Result<http::Response<Vec<u8>>, Error>)>(
            results.len(),
        );

        std::thread::scope(|s| {
            s.spawn(move || {
                let mut multi = multi::Multi::new();

                // We _could_ set the error for all packages and just return here, but not being able to enable pipelining doesn't
                // necessarily mean that the actual I/O will fail, just slower than it could be
                let _might_care = multi.pipelining(true, true);

                macro_rules! send {
                    ($n:expr, $e:expr) => {
                        drop(tx.send(($n, $e)));
                    };
                }

                fn add(
                    h: easy::Easy2<Response>,
                    i: usize,
                    multi: &mut multi::Multi,
                ) -> Result<multi::Easy2Handle<Response>, Error> {
                    let mut handle = try_curl!(multi.add2(h));

                    match handle.set_token(i) {
                        Ok(()) => Ok(handle),
                        Err(e) => {
                            // Unsure what scenario setting token would fail after adding the handle, but just in case, attempt
                            // to remove the handle, ignore the possible error from remove as the failure to set the token is more
                            // relevant than the follow on error
                            let _dont_care = multi.remove2(handle);
                            Err(curl_err!(e))
                        }
                    }
                }

                struct Item {
                    name: Option<Name>,
                    handle: Option<multi::Easy2Handle<Response>>,
                }

                let mut handles: Vec<_> = packages
                    .into_iter()
                    .enumerate()
                    .map(|(i, pn)| match self.build_request(&pn, lock, timeout) {
                        Ok(h) => match add(h, i, &mut multi) {
                            Ok(h) => Item {
                                name: Some(pn),
                                handle: Some(h),
                            },
                            Err(err) => {
                                send!(pn, Err(err));
                                Item {
                                    name: None,
                                    handle: None,
                                }
                            }
                        },
                        Err(err) => {
                            send!(pn, Err(err));
                            Item {
                                name: None,
                                handle: None,
                            }
                        }
                    })
                    .collect();

                let mut still_alive = true;
                while still_alive {
                    match multi.perform() {
                        Err(err) => {
                            // Mark any of the remaining items with the same error, this would be lying in the case of any crates
                            // that were requested, but were actually NOT_FOUND, but that's probably not a big issue
                            let err = curl_err!(err);
                            for item in &mut handles {
                                if let Some(pn) = item.name.take() {
                                    send!(pn, Err(err.clone()));
                                }
                            }
                            break;
                        }
                        Ok(tx) => {
                            // we still need to process any remaining messages
                            still_alive = tx != 0;
                        }
                    }

                    multi.messages(|message| {
                        let Ok(token) = message.token() else {
                            // We always set the token, so its absence is really a terminal failure, but since we want this
                            // method to be resilient, just ignore it
                            return;
                        };

                        let Some((name, handle)) =
                            handles[token].name.take().zip(handles[token].handle.take())
                        else {
                            return;
                        };

                        let Some(res) = message.result_for2(&handle) else {
                            return;
                        };

                        let res = match multi.remove2(handle) {
                            Ok(h) => match res {
                                Ok(()) => {
                                    let res = handle_to_http_response(h);
                                    Ok(res)
                                }
                                Err(err) => Err(curl_err!(err)),
                            },
                            Err(err) => Err(curl_err!(err)),
                        };

                        send!(name, res);
                    });

                    if still_alive {
                        // The sleeping time could be reduced to allow other processing.
                        // For instance, a thread could check a condition signalling the
                        // thread shutdown.
                        if let Err(err) = multi.wait(&mut [], std::time::Duration::from_secs(1)) {
                            let err = curl_err!(err);
                            for item in &mut handles {
                                if let Some(name) = item.name.take() {
                                    send!(name, Err(err.clone()));
                                }
                            }
                        }
                    }
                }

                for mut item in handles {
                    if let Some(name) = item.name.take() {
                        send!(
                            name,
                            Err(Error::from_tame(tame_index::Error::Http(
                                tame_index::error::HttpError::Timeout,
                            )))
                        );
                    }

                    if let Some(handle) = item.handle.take() {
                        let _dont_care = multi.remove2(handle);
                    }
                }

                // We don't move
                drop(tx);
            });

            s.spawn(|| {
                while let Ok((name, res)) = rx.recv() {
                    let res = match res {
                        Ok(response) => {
                            // This would have failed during request creation if it was invalid
                            let pn = name.as_str().try_into().unwrap();
                            let (parts, body) = response.into_parts();

                            self.index
                                .parse_remote_response(
                                    pn,
                                    http::response::Response::from_parts(parts, &body),
                                    true,
                                    lock,
                                )
                                .map_err(Error::from_tame)
                        }
                        Err(err) => Err(err),
                    };

                    results.push((name, res));
                }
            });
        });

        Ok(results)
    }

    #[inline]
    fn build_request(
        &self,
        name: &Name,
        lock: &FileLock,
        timeout: Option<std::time::Duration>,
    ) -> Result<easy::Easy2<Response>, Error> {
        let name = try_tame!(name.as_str().try_into());
        let req = try_tame!(self.index.make_remote_request(name, None, lock));

        let (
            http::request::Parts {
                uri,
                headers,
                // method, as of now always GET
                ..
            },
            _, // as of now, there will never be a body
        ) = req.into_parts();

        let mut req = easy::Easy2::new(Response::new());
        // http Uri doesn't have a as_str() since it's stored in separate parts, so we're doing a heap allocation
        // for the String which is then being heap allocated again to a CString...and passing that by reference
        // presumably to be heap allocated by curl :-/
        try_curl!(req.url(&format!("{uri}")));
        try_curl!(req.get(true));
        try_curl!(req.useragent(&self.user_agent));
        try_curl!(req.http_version(easy::HttpVersion::V2));

        if let Some(timeout) = timeout {
            try_curl!(req.timeout(timeout));
        }

        // This is barely an optimization since append internally creates a CString :-/
        let mut buf = String::with_capacity(128);
        let mut list = easy::List::new();

        let mut append = |h: (Option<header::HeaderName>, header::HeaderValue),
                          list: &mut easy::List| {
            if let Some(hn) = h.0 {
                buf.clear();
                buf.push_str(hn.as_str());
                buf.push_str(": ");
            }

            let Ok(val) = h.1.to_str() else {
                // the curl API only allows utf-8 strings, this is fine as all headers as of now are utf-8
                return;
            };

            let mark = buf.len();
            buf.push_str(val);

            // This would only fail if we were giving strings with null terminators in them, we are not
            list.append(&buf).unwrap();
            buf.truncate(mark);
        };

        for h in headers {
            append(h, &mut list);
        }

        try_curl!(req.http_headers(list));
        Ok(req)
    }
}

struct Response {
    etag: Option<header::HeaderValue>,
    last_modified: Option<header::HeaderValue>,
    gzipped: bool,
    body: Vec<u8>,
}

impl Response {
    fn new() -> Self {
        Self {
            etag: None,
            last_modified: None,
            gzipped: false,
            body: Vec::with_capacity(1024),
        }
    }
}

impl easy::Handler for Response {
    fn write(&mut self, data: &[u8]) -> Result<usize, easy::WriteError> {
        // TODO: as of now crates.io doesn't respond with gzipped contents even when we ask for them, but if that changes
        // we'd want to create a gzip decoder when we parsed that header and progressively decompress as data was written
        // rather than allocate for the compressed data & the eventual decompression
        self.body.extend_from_slice(data);
        Ok(data.len())
    }

    fn header(&mut self, data: &[u8]) -> bool {
        // the index only cares about a limited number of headers, so filter them out here to avoid allocating memory
        // that we'll just throw away later
        let Ok(hdr) = std::str::from_utf8(data) else {
            return true;
        };
        // Remove the trailing newlines
        let Some((name, value)) = hdr.trim_end().split_once(": ") else {
            return true;
        };

        let Ok(name) = header::HeaderName::from_bytes(name.as_bytes()) else {
            return true;
        };

        match name {
            header::ETAG => {
                self.etag = header::HeaderValue::from_str(value).ok();
            }
            header::LAST_MODIFIED => {
                self.last_modified = header::HeaderValue::from_str(value).ok();
            }
            header::CONTENT_ENCODING => {
                self.gzipped = value == "gzip";
            }
            _ => {}
        }

        true
    }
}

fn handle_to_http_response(mut h: easy::Easy2<Response>) -> http::response::Response<Vec<u8>> {
    let mut b = http::response::Builder::new().status(
        http::StatusCode::from_u16(h.response_code().unwrap_or(418) as u16)
            .unwrap_or(http::StatusCode::IM_A_TEAPOT),
    );

    let res = h.get_mut();

    // this will always be Some
    if let Some(hm) = b.headers_mut() {
        hm.extend(res.etag.take().map(|v| (header::ETAG, v)));
        hm.extend(res.last_modified.take().map(|v| (header::LAST_MODIFIED, v)));
    }

    let body = std::mem::take(&mut res.body);

    // This can't fail, we don't set extensions, and we manually add headers only if they are valid
    b.body(body).expect("unreachable")
}
