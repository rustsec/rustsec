//! Advisory dates

use crate::error::{Error, ErrorKind};

use chrono::{self, NaiveDate, Utc};
use serde::{de, Deserialize, Serialize};
use std::str::FromStr;

/// Minimum allowed year on advisory dates
pub(crate) const YEAR_MIN: u32 = 2000;

/// Maximum allowed year on advisory dates
pub(crate) const YEAR_MAX: u32 = YEAR_MIN + 100;

/// Dates on advisories (RFC 3339)
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Date(String);

impl Date {
    /// Convert an advisory RFC 3339 date into a `chrono::Date`
    pub fn to_chrono_date(&self) -> Result<chrono::Date<Utc>, Error> {
        let date = NaiveDate::parse_from_str(&self.0, "%Y-%m-%d")?;
        Ok(chrono::Date::from_utc(date, Utc))
    }

    /// Borrow this date as a string reference
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}

impl AsRef<str> for Date {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for Date {
    type Err = Error;

    /// Create a `Date` from the given RFC 3339 date string
    fn from_str(rfc3339_date: &str) -> Result<Self, Error> {
        validate_date(rfc3339_date)?;
        Ok(Date(rfc3339_date.into()))
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
            .ok_or_else(|| format_err!(ErrorKind::Parse, "invalid date: {}", $string))?;

        if part.len() != $len {
            fail!(ErrorKind::Parse, "malformed {}: {}", $name, $string);
        }

        match part.parse::<u32>() {
            Ok($min..=$max) => (),
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
    use std::str::FromStr;

    #[test]
    fn from_str_test() {
        // Valid dates
        assert!(Date::from_str("2000-01-01").is_ok());
        assert!(Date::from_str("2017-01-01").is_ok());
        assert!(Date::from_str("2099-12-31").is_ok());

        // Invalid dates
        assert!(Date::from_str("derp").is_err());
        assert!(Date::from_str("1999-12-31").is_err());
        assert!(Date::from_str("02017-01-01").is_err());
        assert!(Date::from_str("2017-00-01").is_err());
        assert!(Date::from_str("2017-01-00").is_err());
        assert!(Date::from_str("2017-13-01").is_err());
        assert!(Date::from_str("2017-01-32").is_err());
        assert!(Date::from_str("2017-01-").is_err());
        assert!(Date::from_str("2017-01-01-01").is_err());
    }
}
