use error::{Error, Result};
use semver::VersionReq;
use toml::value::Table;
use toml::Value;

pub fn parse_optional_string(table: &Table, attribute: &str) -> Result<Option<String>> {
    match table.get(attribute) {
        Some(v) => Ok(Some(String::from(v.as_str().ok_or(Error::InvalidAttribute)?))),
        None => Ok(None),
    }
}

pub fn parse_mandatory_string(table: &Table, attribute: &str) -> Result<String> {
    let str = parse_optional_string(table, attribute)?;
    str.ok_or(Error::MissingAttribute)
}

pub fn parse_versions(table: &Table, attribute: &str) -> Result<Vec<VersionReq>> {
    match table.get(attribute) {
        Some(&Value::Array(ref arr)) => {
            let mut result = Vec::new();

            for version in arr {
                let version_str = version.as_str().ok_or(Error::MissingAttribute)?;
                let version_req = VersionReq::parse(version_str).or(Err(Error::MalformedVersion))?;

                result.push(version_req)
            }

            Ok(result)
        }
        Some(_) => Err(Error::InvalidAttribute),
        None => Err(Error::MissingAttribute),
    }
}
