#[cfg(feature = "chrono")]
use chrono::{self, DateTime, Utc};

use error::Error;

/// Dates on advisories
// TODO: better validate how these are formed
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Date(String);

impl Date {
    /// Convert an advisory RFC 3339 date into a `chrono::Date`
    #[cfg(feature = "chrono")]
    pub fn to_chrono_date(&self) -> Result<chrono::Date<Utc>, Error> {
        let datetime = DateTime::parse_from_rfc3339(self.0.as_ref())?;
        Ok(chrono::Date::from_utc(datetime.naive_utc().date(), Utc))
    }
}

impl Into<String> for Date {
    fn into(self) -> String {
        self.0
    }
}

impl<'a> From<&'a str> for Date {
    // TODO: validate inputs
    fn from(string: &'a str) -> Date {
        Date(string.into())
    }
}
