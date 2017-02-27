//! Advisory type and related parsing code

use error::{Error, Result};
use semver::VersionReq;
use toml;

/// An individual security advisory pertaining to a single vulnerability
#[derive(Debug, PartialEq)]
pub struct Advisory {
    /// Security advisory ID (e.g. RUSTSEC-YYYY-NNNN)
    pub id: String,

    /// Name of affected crate
    pub package: String,

    /// Versions which are patched and not vulnerable (expressed as semantic version requirements)
    pub patched_versions: Vec<VersionReq>,

    /// Date vulnerability was originally disclosed (optional)
    pub date: Option<String>,

    /// URL with an announcement (e.g. blog post, PR, disclosure issue, CVE)
    pub url: Option<String>,

    /// One-liner description of a vulnerability
    pub title: String,

    /// Extended description of a vulnerability
    pub description: String,
}

impl Advisory {
    /// Parse an Advisory from a TOML table object
    pub fn from_toml_table(value: &toml::value::Table) -> Result<Advisory> {
        Ok(Advisory {
            id: try!(parse_mandatory_string(value, "id")),
            package: try!(parse_mandatory_string(value, "package")),
            patched_versions: try!(parse_versions(&value["patched_versions"])),
            date: try!(parse_optional_string(value, "date")),
            url: try!(parse_optional_string(value, "url")),
            title: try!(parse_mandatory_string(value, "title")),
            description: try!(parse_mandatory_string(value, "description")),
        })
    }
}

fn parse_optional_string(table: &toml::value::Table, attribute: &str) -> Result<Option<String>> {
    match table.get(attribute) {
        Some(v) => Ok(Some(String::from(try!(v.as_str().ok_or(Error::InvalidAttribute))))),
        None => Ok(None),
    }
}

fn parse_mandatory_string(table: &toml::value::Table, attribute: &str) -> Result<String> {
    let str = try!(parse_optional_string(table, attribute));
    str.ok_or(Error::MissingAttribute)
}

fn parse_versions(value: &toml::Value) -> Result<Vec<VersionReq>> {
    match *value {
        toml::Value::Array(ref arr) => {
            let mut result = Vec::new();
            for version in arr {
                let version_str = try!(version.as_str().ok_or(Error::MissingAttribute));
                let version_req = try!(VersionReq::parse(version_str)
                    .map_err(|_| Error::MalformedVersion));

                result.push(version_req)
            }
            Ok(result)
        }
        _ => Err(Error::MissingAttribute),
    }
}
