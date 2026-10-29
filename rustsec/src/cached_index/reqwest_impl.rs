use crate::{error::Error, package::Name};
use tame_index::{external::http, flock::FileLock};

macro_rules! req_err {
    ($err:expr) => {
        $crate::error::Error::from_source($crate::error::ErrorKind::Registry, $err)
    };
}

macro_rules! try_reqwest {
    ($e:expr) => {
        match $e {
            Ok(r) => r,
            Err(err) => {
                return Err(req_err!(err));
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
    client: reqwest::Client,
}

impl RemoteSparseIndex {
    /// Crates a new [`Self`] for the provided index, using the specified user agent when making remote requests
    pub fn new(index: tame_index::SparseIndex, user_agent: String) -> Result<Self, Error> {
        let client = try_reqwest!(
            reqwest::ClientBuilder::new()
                // This would need to be removed if rustsec needed to query HTTP indices that don't support HTTP/2, but
                // for now where crates.io is the primary (really, only) remote, this is absolutely critical, cutting
                // down the time for a bulk fetch for this repo's lockfile from ~10s to ~700ms
                .http2_prior_knowledge()
                // Note that it is is _required_ for us to explicitly call use_rustls_tls here as otherwise http2_prior_knowledge
                // WILL NOT WORK https://github.com/seanmonstar/reqwest/issues/857. Sadly, we CANNOT configure tls and call
                // use_preconfigured_tls even when we use rustls.
                .use_rustls_tls()
                .user_agent(user_agent)
                //.tls_backend_preconfigured(tls)
                .build()
        );

        Ok(Self { index, client })
    }

    async fn exec_request(
        client: &reqwest::Client,
        req: reqwest::Request,
    ) -> Result<http::Response<Vec<u8>>, Error> {
        // This is unfortunate, but we always make a copy in case we need to retry
        let mut res = loop {
            let reqc = req.try_clone().unwrap();
            let res = client.execute(reqc).await;

            match res {
                Err(err) if err.is_connect() || err.is_timeout() || err.is_request() => {}
                Err(err) => return Err(req_err!(err)),
                Ok(res) => break res,
            }
        };

        // reqwest implements From for http::Response, however the body still needs to be read, but reqwest uses frankly
        // ridiculuous levels of abstractions to do this but doesn't re-export the crates it uses so rather than do
        // that implementation here, just be lazy and take the headers before reading the body, it's annoying, but here
        // we are
        let status = res.status();
        let mut headers = std::mem::take(res.headers_mut());
        let body = try_reqwest!(res.bytes().await);

        let mut res = http::Response::builder().status(status);
        std::mem::swap(res.headers_mut().unwrap(), &mut headers);

        Ok(res.body(body.into()).unwrap())
    }

    /// Attempts to fetch the metadata from the index for all of the specified crate names
    ///
    /// The timeout is on a per-crate basis
    pub fn multi_fetch(
        &self,
        mut krates: std::collections::BTreeSet<Name>,
        timeout: Option<std::time::Duration>,
        lock: &FileLock,
    ) -> Result<super::MultiFetchResult, Error> {
        if krates.is_empty() {
            return Ok(Default::default());
        }

        let rt = tokio::runtime::Runtime::new().map_err(|err| {
            Error::with_source(
                crate::error::ErrorKind::Registry,
                "unable to start a tokio runtime".to_owned(),
                err,
            )
        })?;

        let (tx, rx) = crossbeam_channel::bounded(krates.len());

        let mut results = Vec::with_capacity(krates.len());

        std::thread::scope(|s| {
            s.spawn(move || {
                rt.block_on(async move {
                    let prep_krate = krates.pop_last().unwrap();

                    macro_rules! send {
                        ($n:expr, $res:expr) => {
                            drop(tx.send(($n, $res)));
                        };
                    }

                    let create_req = |kname: &str| -> Result<reqwest::Request, Error> {
                        let name = try_tame!(kname.try_into());
                        let request = try_tame!(self.index.make_remote_request(name, None, lock));

                        let mut req = self.client.request(http::Method::GET, request.uri);
                        req = req.version(http::Version::HTTP_2);

                        for (k, v) in request.headers {
                            req = req.header(k, v);
                        }

                        Ok(try_reqwest!(req.build()))
                    };

                    {
                        let res = match create_req(prep_krate.as_str()) {
                            Ok(req) => Self::exec_request(&self.client, req).await,
                            Err(err) => Err(err),
                        };

                        send!(prep_krate, res);
                    }

                    let mut tasks = tokio::task::JoinSet::new();

                    for kname in krates {
                        match create_req(kname.as_str()) {
                            Ok(req) => {
                                let client = self.client.clone();
                                let ttx = tx.clone();
                                tasks.spawn(async move {
                                    let res = if let Some(to) = timeout {
                                        match tokio::time::timeout(
                                            to,
                                            Self::exec_request(&client, req),
                                        )
                                        .await
                                        {
                                            Ok(res) => res,
                                            Err(_) => Err(Error::from_tame(
                                                tame_index::error::HttpError::StatusCode {
                                                    code: http::StatusCode::REQUEST_TIMEOUT,
                                                    msg: "request timed out",
                                                }
                                                .into(),
                                            )),
                                        }
                                    } else {
                                        Self::exec_request(&client, req).await
                                    };

                                    drop(ttx.send((kname, res)));
                                });
                            }
                            Err(err) => {
                                send!(kname, Err(err));
                            }
                        }
                    }

                    while let Some(_res) = tasks.join_next().await {}
                });
            });

            s.spawn(|| {
                while let Ok((kname, res)) = rx.recv() {
                    let res = res.and_then(|res| {
                        let name = kname
                            .as_str()
                            .try_into()
                            .expect("this was already validated");
                        super::process_entry(
                            self.index.parse_remote_response(name, &res, true, lock),
                        )
                    });

                    results.push((kname, res));
                }
            });
        });

        Ok(results)
    }
}
