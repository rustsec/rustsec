use error::{Error, Result};
use semver::VersionReq;
use toml;

/// An individual security advisory pertaining to a single vulnerability
#[derive(Debug, PartialEq)]
pub struct Advisory {
    pub id: String,
    pub crate_name: String,
    pub patched_versions: Vec<VersionReq>,
    pub date: Option<String>,
    pub url: Option<String>,
    pub title: String,
    pub description: String,
}

impl Advisory {
    /// Parse an Advisory from a TOML value object
    pub fn from_toml_value(value: &toml::Value) -> Result<Advisory> {
        Ok(Advisory {
            id: String::from(try!(value["id"].as_str().ok_or(Error::MissingAttribute))),
            crate_name: String::from(try!(value["crate_name"].as_str().ok_or(Error::MissingAttribute))),
            patched_versions: try!(parse_versions(&value["patched_versions"])),
            date: value["date"].as_str().map(String::from),
            url: value["url"].as_str().map(String::from),
            title: String::from(try!(value["title"].as_str().ok_or(Error::MissingAttribute))),
            description: String::from(try!(value["description"]
                .as_str()
                .ok_or(Error::MissingAttribute))),
        })
    }
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
