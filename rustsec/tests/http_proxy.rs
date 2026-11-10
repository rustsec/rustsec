//! Exercise Cargo proxy settings without changing the test runner's environment.
// These HTTPS fixtures use the default TLS-enabled reqwest transport. The
// bare gix-curl feature does not enable a TLS backend.
#![cfg(feature = "gix-reqwest")]

use std::{
    io::{Read, Write},
    net::TcpListener,
    process::Command,
    thread,
    time::{Duration, Instant},
};

#[test]
#[ignore = "subprocess entry point for isolated environment"]
fn proxy_child() {
    let Ok(mode) = std::env::var("RUSTSEC_PROXY_TEST") else {
        return;
    };
    if mode == "registry" {
        let lockfile = rustsec::Lockfile::load("Cargo.lock").unwrap();
        let mut index =
            rustsec::registry::CachedIndex::fetch_as(Duration::ZERO, "proxy-test".into()).unwrap();
        // The proxy deliberately refuses the request. We only need to prove routing.
        let _ = index.find_yanked(lockfile.packages.iter());
    } else {
        let path = std::env::current_dir().unwrap().join("db");
        let url = if mode.starts_with("empty-") {
            "https://127.0.0.1:1/fixture.git"
        } else {
            "https://proxy-target.invalid/fixture.git"
        };
        if mode.ends_with("fetch") {
            let repo = gix::init(&path).unwrap();
            let mut config = std::fs::OpenOptions::new()
                .append(true)
                .open(repo.git_dir().join("config"))
                .unwrap();
            writeln!(
                config,
                "[remote \"origin\"]\nurl = {url}\nproxy = http://127.0.0.1:1"
            )
            .unwrap();
        }
        let error = rustsec::repository::git::Repository::fetch(url, path, false, Duration::ZERO)
            .err()
            .expect("proxy fixture must refuse the request");
        eprintln!("{error:?}");
    }
}

fn check_routing(mode: &str, socks: bool, source: &str) {
    let temp = tempfile::tempdir().unwrap();
    let cargo_home = temp.path().join("cargo");
    std::fs::create_dir(&cargo_home).unwrap();
    std::fs::write(
        temp.path().join("Cargo.lock"),
        include_str!("support/cratesio_cargo.lock"),
    )
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let scheme = if socks { "socks5h" } else { "http" };
    let proxy = format!("{scheme}://{}", listener.local_addr().unwrap());
    let server = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(15);
        let (mut socket, _) = loop {
            match listener.accept() {
                Ok(connection) => break connection,
                Err(error)
                    if error.kind() == std::io::ErrorKind::WouldBlock
                        && Instant::now() < deadline =>
                {
                    thread::sleep(Duration::from_millis(10))
                }
                Err(error) => panic!("proxy connection missing: {error}"),
            }
        };
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut buf = [0; 4096];
        if socks {
            socket.read_exact(&mut buf[..3]).unwrap();
            assert_eq!(&buf[..3], &[5, 1, 0]);
            socket.write_all(&[5, 0]).unwrap();
            socket.read_exact(&mut buf[..5]).unwrap();
            assert_eq!(&buf[..4], &[5, 1, 0, 3]); // Remote DNS resolution, not local DNS.
            let length = usize::from(buf[4]);
            socket.read_exact(&mut buf[5..7 + length]).unwrap();
            let target = std::str::from_utf8(&buf[5..5 + length]).unwrap();
            assert!(
                matches!(target, "proxy-target.invalid" | "index.crates.io"),
                "{target}"
            );
            socket.write_all(&[5, 5, 0, 1, 0, 0, 0, 0, 0, 0]).unwrap();
        } else {
            let mut n = 0;
            while !buf[..n].ends_with(b"\r\n\r\n") {
                let read = socket.read(&mut buf[n..]).unwrap();
                assert!(read > 0, "incomplete CONNECT request");
                n += read;
                assert!(n < buf.len(), "CONNECT request exceeds fixture buffer");
            }
            let request = std::str::from_utf8(&buf[..n]).unwrap();
            assert!(
                request.starts_with("CONNECT proxy-target.invalid:443 ")
                    || request.starts_with("CONNECT index.crates.io:443 "),
                "{request}"
            );
            socket
                .write_all(b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\n\r\n")
                .unwrap();
        }
    });
    let mut child = Command::new(std::env::current_exe().unwrap());
    child
        .args(["--ignored", "--exact", "proxy_child", "--nocapture"])
        .current_dir(temp.path())
        .env("HOME", temp.path())
        .env("USERPROFILE", temp.path())
        .env("CARGO_HOME", &cargo_home)
        .env("RUSTSEC_PROXY_TEST", mode)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", temp.path().join("no-git-config"));
    for key in [
        "HTTP_PROXY",
        "HTTPS_PROXY",
        "ALL_PROXY",
        "NO_PROXY",
        "http_proxy",
        "https_proxy",
        "all_proxy",
        "no_proxy",
        "CARGO_HTTP_PROXY",
    ] {
        child.env_remove(key);
    }
    if source == "env" {
        child.env("CARGO_HTTP_PROXY", &proxy);
    } else if source == "git" {
        let path = temp.path().join("gitconfig");
        std::fs::write(&path, format!("[http]\nproxy = {proxy}\n")).unwrap();
        child.env("GIT_CONFIG_GLOBAL", path);
    } else {
        std::fs::write(
            cargo_home.join("config.toml"),
            format!("[http]\nproxy = {proxy:?}\n"),
        )
        .unwrap();
    }
    let mut child = child.spawn().unwrap();
    let routed = server.join();
    // tame-index retries connection errors. Stop the isolated registry worker once
    // the proxy has observed the request, rather than waiting on those retries.
    if mode == "registry" || routed.is_err() {
        let _ = child.kill();
    }
    let status = child.wait().unwrap();
    routed.unwrap();
    if mode != "registry" {
        assert!(status.success());
    }
}

#[test]
fn cargo_proxy_routes_clone() {
    for socks in [false, true] {
        for source in ["env", "cargo", "git"] {
            check_routing("clone", socks, source);
        }
    }
}

#[test]
fn cargo_proxy_routes_fetch() {
    for socks in [false, true] {
        for source in ["env", "cargo", "git"] {
            check_routing("fetch", socks, source);
        }
    }
}

#[test]
fn cargo_proxy_routes_registry() {
    for socks in [false, true] {
        for source in ["env", "cargo", "git"] {
            check_routing("registry", socks, source);
        }
    }
}

#[test]
fn empty_cargo_proxy_disables_inherited_git_and_environment_proxies() {
    let temp = tempfile::tempdir().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let proxy = format!("http://{}", listener.local_addr().unwrap());
    let cargo_home = temp.path().join("cargo");
    std::fs::create_dir(&cargo_home).unwrap();
    std::fs::write(cargo_home.join("config.toml"), "[http]\nproxy = ''\n").unwrap();
    let git_config = temp.path().join("gitconfig");
    std::fs::write(&git_config, format!("[http]\nproxy = {proxy}\n")).unwrap();
    for mode in ["empty-clone", "empty-fetch"] {
        let output = Command::new(std::env::current_exe().unwrap())
            .args(["--ignored", "--exact", "proxy_child"])
            .current_dir(temp.path())
            .env("HOME", temp.path())
            .env("USERPROFILE", temp.path())
            .env("CARGO_HOME", &cargo_home)
            .env("RUSTSEC_PROXY_TEST", mode)
            .env_remove("CARGO_HTTP_PROXY")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", &git_config)
            .env("HTTP_PROXY", &proxy)
            .env("HTTPS_PROXY", &proxy)
            .env("ALL_PROXY", &proxy)
            .env("http_proxy", &proxy)
            .env("https_proxy", &proxy)
            .env("all_proxy", &proxy)
            .env_remove("NO_PROXY")
            .env_remove("no_proxy")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            listener.accept().unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
    }
}
