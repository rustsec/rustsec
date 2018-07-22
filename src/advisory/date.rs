#[cfg(feature = "chrono")]
use chrono::{self, DateTime, Utc};
use serde::{de::Error as DeError, Deserialize, Deserializer};

use error::{Error, ErrorKind};

/// Minimum allowed year on advisory dates
pub(crate) const YEAR_MIN: u32 = 2000;

/// Maximum allowed year on advisory dates
pub(crate) const YEAR_MAX: u32 = YEAR_MIN + 100;

/// Dates on advisories (RFC 3339)
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Date(String);

impl Date {
    /// Create a `Date` from the given string (RFC 3339 date)
    pub fn new<S: Into<String>>(into_string: S) -> Result<Self, Error> {
        let string = into_string.into();
        validate_date(&string)?;
        Ok(Date(string))
    }

    /// Convert an advisory RFC 3339 date into a `chrono::Date`
    #[cfg(feature = "chrono")]
    pub fn to_chrono_date(&self) -> chrono::Date<Utc> {
        let datetime = DateTime::parse_from_rfc3339(self.0.as_ref()).unwrap();
        chrono::Date::from_utc(datetime.naive_utc().date(), Utc)
    }

    /// Borrow this date as a string reference
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::new(String::deserialize(deserializer)?)
            .map_err(|e| D::Error::custom(format!("{}", e)))
    }
}

impl Into<String> for Date {
    fn into(self) -> String {
        self.0
    }
}

macro_rules! check_date_part {
    ($name:expr, $string:expr, $parts:expr, $len:expr, $min:expr, $max:expr) => {
        let part = $parts
            .next()
            .ok_or_else(|| err!(ErrorKind::Parse, "invalid date: {}", $string))?;

        if part.len() != $len {
            fail!(ErrorKind::Parse, "malformed {}: {}", $name, $string);
        }

        match part.parse::<u32>() {
            Ok($min...$max) => (),
            _ => fail!(ErrorKind::Parse, "malformed {}: {}", $name, $string),
        }
    };
}

/// Validate that a date is well-formed
fn validate_date(string: &str) -> Result<(), Error> {
    let mut parts = string.split('-');

    check_date_part!("year", string, parts, 4, YEAR_MIN, YEAR_MAX);
    check_date_part!("month", string, parts, 2, 1, 12);
    // TODO: ensure day is in range for month
    check_date_part!("day", string, parts, 2, 1, 31);

    if parts.next().is_some() {
        fail!(ErrorKind::Parse, "invalid date: {}", string)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Date;

    #[test]
    fn valid_date_test() {
        assert!(Date::new("2000-01-01").is_ok());
        assert!(Date::new("2017-01-01").is_ok());
        assert!(Date::new("2099-12-31").is_ok());
    }

    #[test]
    fn invalid_date_test() {
        assert!(Date::new("derp").is_err());
        assert!(Date::new("1999-12-31").is_err());
        assert!(Date::new("02017-01-01").is_err());
        assert!(Date::new("2017-00-01").is_err());
        assert!(Date::new("2017-01-00").is_err());
        assert!(Date::new("2017-13-01").is_err());
        assert!(Date::new("2017-01-32").is_err());
        assert!(Date::new("2017-01-").is_err());
        assert!(Date::new("2017-01-01-01").is_err());
    }
}
