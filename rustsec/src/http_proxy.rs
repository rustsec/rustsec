//! Cargo proxy configuration shared by both network clients.

use crate::{Error, ErrorKind};

pub(crate) fn cargo_proxy() -> Result<Option<String>, Error> {
    cargo_config2::Config::load()
        .map_err(|_| {
            Error::new(
                ErrorKind::Parse,
                "failed to read Cargo HTTP proxy configuration",
            )
        })
        .and_then(|config| {
            if config.http.proxy.is_some() {
                return validate_proxy(config.http.proxy);
            }
            // Cargo falls back to the global Git http.proxy setting before the
            // transport's ordinary HTTP(S)_PROXY environment handling.
            let git_config = gix::config::File::from_globals().map_err(|_| {
                Error::new(
                    ErrorKind::Parse,
                    "failed to read global Git proxy configuration",
                )
            })?;
            let proxy = git_config
                .string("http.proxy")
                .map(|value| {
                    String::from_utf8(value.into()).map_err(|_| {
                        Error::new(ErrorKind::Parse, "global Git HTTP proxy is not valid UTF-8")
                    })
                })
                .transpose()?;
            validate_proxy(proxy)
        })
}

fn validate_proxy(proxy: Option<String>) -> Result<Option<String>, Error> {
    if let Some(value) = proxy.as_deref().filter(|value| !value.is_empty()) {
        reqwest::Proxy::all(value)
            .map_err(|_| Error::new(ErrorKind::BadParam, "invalid Cargo HTTP proxy"))?;
    }
    Ok(proxy)
}

pub(crate) fn configure_git_connection<T>(
    connection: &mut gix::remote::Connection<'_, '_, '_, T>,
) -> gix::Result
where
    T: gix::protocol::transport::client::blocking_io::Transport,
{
    use gix::protocol::transport::client::blocking_io::http;
    let Some(proxy) = cargo_proxy().map_err(|_| {
        gix::Error::from(gix::error::message(
            "failed to read Cargo HTTP proxy configuration",
        ))
    })?
    else {
        // No Cargo override: leave the transport's existing Git/environment behavior alone.
        return Ok(());
    };
    let remote = connection.remote();
    let Some(url) = remote.url(gix::remote::Direction::Fetch) else {
        return Ok(());
    };
    let Some(mut options) = remote.repo().transport_options(
        gix::bstr::ByteSlice::as_bstr(url.to_bstring().as_slice()),
        remote.name().map(gix::remote::Name::as_bstr),
    )?
    else {
        return Ok(());
    };
    if let Some(http) = options.downcast_mut::<http::Options>() {
        http.proxy = Some(proxy.clone());
        #[cfg(feature = "gix-reqwest")]
        {
            let proxy = if proxy.is_empty() {
                None
            } else {
                Some(
                    reqwest::Proxy::all(proxy)
                        .map_err(|_| gix::Error::from(gix::error::message("invalid HTTP proxy")))?
                        .no_proxy(reqwest::NoProxy::from_string(
                            http.no_proxy.as_deref().unwrap_or_default(),
                        )),
                )
            };
            http.backend = Some(std::sync::Arc::new(std::sync::Mutex::new(
                http::reqwest::Options {
                    configure_client: Some(Box::new(move |builder| {
                        Ok(match &proxy {
                            Some(proxy) => builder.proxy(proxy.clone()),
                            None => builder.no_proxy(),
                        })
                    })),
                    ..Default::default()
                },
            )));
        }
    }
    connection.set_transport_options(options);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_proxy;
    use cargo_config2::{Config, ResolveOptions};
    use std::path::Path;

    fn load(root: &Path, env: &[(&str, &str)]) -> Option<String> {
        Config::load_with_options(
            root.join("project/nested"),
            ResolveOptions::default()
                .cargo_home(root.join("home"))
                .env(env.iter().copied()),
        )
        .unwrap()
        .http
        .proxy
    }

    fn fixture() -> tempfile::TempDir {
        let root = tempfile::tempdir().unwrap();
        for path in ["project/nested/.cargo", "project/.cargo", "home"] {
            std::fs::create_dir_all(root.path().join(path)).unwrap();
        }
        root
    }

    #[test]
    fn invalid_proxy_does_not_expose_credentials() {
        let error =
            validate_proxy(Some("http://user:secret-sentinel@[invalid".into())).unwrap_err();
        assert!(!format!("{error:?}").contains("secret-sentinel"));
        assert!(!format!("{error}").contains("user"));
        assert_eq!(error.kind(), crate::ErrorKind::BadParam);
    }

    #[test]
    fn http_and_socks_proxies_are_supported() {
        for value in [
            "http://proxy.invalid:8080",
            "https://proxy.invalid",
            "socks5h://proxy.invalid:1080",
        ] {
            assert_eq!(
                validate_proxy(Some(value.into())).unwrap().as_deref(),
                Some(value)
            );
        }
    }

    #[test]
    fn no_cargo_override_keeps_transport_defaults() {
        let root = fixture();
        assert_eq!(load(root.path(), &[]), None);
    }

    #[test]
    fn cargo_proxy_precedence() {
        let root = fixture();
        let write = |path: &str, value: &str| {
            std::fs::write(
                root.path().join(path),
                format!("[http]\nproxy = {value:?}\n"),
            )
            .unwrap()
        };
        write("home/config.toml", "http://home.invalid");
        assert_eq!(
            load(root.path(), &[]).as_deref(),
            Some("http://home.invalid")
        );
        write("project/.cargo/config.toml", "http://parent.invalid");
        assert_eq!(
            load(root.path(), &[]).as_deref(),
            Some("http://parent.invalid")
        );
        write(
            "project/nested/.cargo/config.toml",
            "socks5h://local.invalid",
        );
        assert_eq!(
            load(root.path(), &[]).as_deref(),
            Some("socks5h://local.invalid")
        );
        assert_eq!(
            load(root.path(), &[("CARGO_HTTP_PROXY", "http://env.invalid")]).as_deref(),
            Some("http://env.invalid")
        );
    }

    #[test]
    fn empty_proxy_is_an_explicit_override() {
        let root = fixture();
        std::fs::write(root.path().join("home/config.toml"), "[http]\nproxy = ''\n").unwrap();
        assert_eq!(load(root.path(), &[]).as_deref(), Some(""));
        assert_eq!(
            load(root.path(), &[("CARGO_HTTP_PROXY", "")]).as_deref(),
            Some("")
        );
    }
}
