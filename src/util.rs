//! Utility functions for parsing TOML files

use error::{Error, ErrorKind};
use semver::{Version, VersionReq};
use toml::value::Table;
use toml::Value;

pub fn parse_optional_string(table: &Table, attribute: &str) -> Result<Option<String>, Error> {
    match table.get(attribute) {
        Some(v) => Ok(Some(String::from(v.as_str().ok_or_else(|| {
            err!(
                ErrorKind::InvalidAttribute,
                "couldn't parse string: {}",
                attribute
            )
        })?))),
        None => Ok(None),
    }
}

pub fn parse_mandatory_string(table: &Table, attribute: &str) -> Result<String, Error> {
    let str = parse_optional_string(table, attribute)?;
    str.ok_or_else(|| {
        err!(
            ErrorKind::MissingAttribute,
            "missing mandatory string: {}",
            attribute
        )
    })
}

pub fn parse_version(table: &Table, attribute: &str) -> Result<Version, Error> {
    let version = parse_mandatory_string(table, attribute)?;
    Version::parse(&version).or_else(|_| {
        Err(err!(
            ErrorKind::MalformedVersion,
            "bad version: {}",
            attribute
        ))
    })
}

pub fn parse_version_reqs(table: &Table, attribute: &str) -> Result<Vec<VersionReq>, Error> {
    match table.get(attribute) {
        Some(&Value::Array(ref arr)) => {
            let mut result = Vec::new();

            for version in arr {
                let version_str = version.as_str().ok_or_else(|| {
                    err!(
                        ErrorKind::MissingAttribute,
                        "missing version requirement attribute"
                    )
                })?;
                let version_req = VersionReq::parse(version_str).or_else(|_| {
                    Err(err!(
                        ErrorKind::MalformedVersion,
                        "bad version requirement: {}",
                        attribute
                    ))
                })?;

                result.push(version_req)
            }

            Ok(result)
        }
        Some(_) => fail!(
            ErrorKind::InvalidAttribute,
            "expected version requirement array for attribute: {}",
            attribute
        ),
        None => fail!(
            ErrorKind::MissingAttribute,
            "missing attribute: {}",
            attribute
        ),
    }
}
