//! rustsec: Client library for the RustSec security advisory database

#![crate_name = "rustsec"]
#![crate_type = "lib"]

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(unsafe_code, unstable_features, unused_import_braces, unused_qualifications)]

extern crate reqwest;
extern crate semver;
extern crate toml;

mod advisory;
mod error;

use advisory::Advisory;
use error::{Error, Result};
use std::io::Read;
use std::str;

/// URL where the TOML file containing the advisory database is located
pub const ADVISORY_DB_URL: &'static str = "https://raw.githubusercontent.\
                                           com/RustSec/advisory-db/master/Advisories.toml";

/// Fetch the advisory database from the server where it is stored
pub fn fetch() -> Result<Vec<Advisory>> {
    let mut response = try!(reqwest::get(ADVISORY_DB_URL).map_err(|_| Error::Request));

    if !response.status().is_success() {
        return Err(Error::Response);
    }

    let mut body = Vec::new();
    try!(response.read_to_end(&mut body).map_err(|_| Error::Response));
    let response_str = try!(str::from_utf8(&body).map_err(|_| Error::Parse));

    from_toml(response_str)
}

/// Parse the advisory database from a TOML serialization of it
pub fn from_toml(data: &str) -> Result<Vec<Advisory>> {
    let db_toml = try!(data.parse::<toml::Value>().map_err(|_| Error::Parse));

    match db_toml["advisory"] {
        toml::Value::Array(ref arr) => {
            let mut result = Vec::new();
            for advisory in arr {
                result.push(try!(Advisory::from_toml_value(advisory)));
            }
            Ok(result)
        }
        _ => Err(Error::MissingAttribute),
    }
}

#[cfg(test)]
mod tests {
    use semver::VersionReq;

    #[test]
    fn fetch() {
        let advisories = super::fetch();
        let ref example_advisory = advisories.unwrap()[0];

        assert_eq!(example_advisory.id, "RUSTSEC-2017-0001");
        assert_eq!(example_advisory.package, "sodiumoxide");
        assert_eq!(example_advisory.patched_versions[0],
                   VersionReq::parse(">= 0.0.14").unwrap());
        assert_eq!(example_advisory.date, Some(String::from("2017-01-26")));
        assert_eq!(example_advisory.url,
                   Some(String::from("https://github.com/dnaq/sodiumoxide/issues/154")));
        assert_eq!(example_advisory.title,
                   "scalarmult() vulnerable to degenerate public keys");
        assert_eq!(&example_advisory.description[0..30],
                   "The `scalarmult()` function in")
    }
}
