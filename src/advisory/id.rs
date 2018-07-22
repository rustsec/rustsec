use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

use super::date::{YEAR_MAX, YEAR_MIN};
use error::{Error, ErrorKind};

/// An identifier for an individual advisory
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct AdvisoryId {
    /// An autodetected identifier kind
    kind: AdvisoryIdKind,

    /// Year this vulnerability was published
    year: Option<u32>,

    /// The actual string representing the identifier
    string: String,
}

impl AdvisoryId {
    /// Create an `AdvisoryId` from the given string
    pub fn new<S: Into<String>>(into_string: S) -> Result<Self, Error> {
        let string = into_string.into();
        let kind = AdvisoryIdKind::detect(&string);

        // Ensure known advisory types are well-formed
        let year = match kind {
            AdvisoryIdKind::RUSTSEC | AdvisoryIdKind::CVE | AdvisoryIdKind::TALOS => {
                Some(parse_year(&string)?)
            }
            _ => None,
        };

        Ok(Self { kind, year, string })
    }

    /// Get a string reference to this advisory ID
    pub fn as_str(&self) -> &str {
        self.string.as_ref()
    }

    /// Get the advisory kind for this advisory
    pub fn kind(&self) -> AdvisoryIdKind {
        self.kind
    }

    /// Is this advisory ID a RUSTSEC advisory?
    pub fn is_rustsec(&self) -> bool {
        match self.kind {
            AdvisoryIdKind::RUSTSEC => true,
            _ => false,
        }
    }

    /// Is this advisory ID a CVE?
    pub fn is_cve(&self) -> bool {
        match self.kind {
            AdvisoryIdKind::CVE => true,
            _ => false,
        }
    }

    /// Is this an unknown kind of advisory ID?
    pub fn is_unknown(&self) -> bool {
        match self.kind {
            AdvisoryIdKind::Unknown => true,
            _ => false,
        }
    }

    /// Get the year this vulnerability was published (if known)
    pub fn year(&self) -> Option<u32> {
        self.year
    }

    /// Get a URL to a web page with more information on this advisory
    pub fn url(&self) -> Option<String> {
        match self.kind {
            AdvisoryIdKind::RUSTSEC => {
                Some(format!("https://rustsec.org/advisories/{}", &self.string))
            }
            AdvisoryIdKind::CVE => Some(format!(
                "https://cve.mitre.org/cgi-bin/cvename.cgi?name={}",
                &self.string
            )),
            AdvisoryIdKind::TALOS => Some(format!(
                "https://www.talosintelligence.com/reports/{}",
                &self.string
            )),
            AdvisoryIdKind::Unknown => None,
        }
    }
}

impl AsRef<AdvisoryId> for AdvisoryId {
    fn as_ref(&self) -> &AdvisoryId {
        self
    }
}

impl fmt::Display for AdvisoryId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.string.fmt(f)
    }
}

impl Into<String> for AdvisoryId {
    fn into(self) -> String {
        self.string
    }
}

impl Serialize for AdvisoryId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.string)
    }
}

impl<'de> Deserialize<'de> for AdvisoryId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::new(String::deserialize(deserializer)?)
            .map_err(|e| D::Error::custom(format!("{}", e)))
    }
}

/// Known kinds of advisory IDs
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum AdvisoryIdKind {
    /// Our advisory namespace
    RUSTSEC,

    /// Common Vulnerabilities and Exposures
    CVE,

    /// Cisco Talos identifiers
    TALOS,

    /// Other types of advisory identifiers we don't know about
    Unknown,
}

impl AdvisoryIdKind {
    /// Detect the identifier kind for the given string
    pub fn detect(string: &str) -> Self {
        if string.starts_with("RUSTSEC-") {
            AdvisoryIdKind::RUSTSEC
        } else if string.starts_with("CVE-") {
            AdvisoryIdKind::CVE
        } else if string.starts_with("TALOS-") {
            AdvisoryIdKind::TALOS
        } else {
            AdvisoryIdKind::Unknown
        }
    }
}

/// Parse the year from an advisory identifier
fn parse_year(advisory_id: &str) -> Result<u32, Error> {
    let mut parts = advisory_id.split('-');
    parts.next().unwrap();

    let year = match parts.next().unwrap().parse::<u32>() {
        Ok(n) => match n {
            YEAR_MIN...YEAR_MAX => n,
            _ => fail!(
                ErrorKind::Parse,
                "out-of-range year in advisory ID: {}",
                advisory_id
            ),
        },
        _ => fail!(
            ErrorKind::Parse,
            "malformed year in advisory ID: {}",
            advisory_id
        ),
    };

    if let Some(num) = parts.next() {
        if num.parse::<u32>().is_err() {
            fail!(ErrorKind::Parse, "malformed advisory ID: {}", advisory_id);
        }
    } else {
        fail!(ErrorKind::Parse, "incomplete advisory ID: {}", advisory_id);
    }

    if parts.next().is_some() {
        fail!(ErrorKind::Parse, "malformed advisory ID: {}", advisory_id);
    }

    Ok(year)
}

#[cfg(test)]
mod tests {
    use super::{AdvisoryId, AdvisoryIdKind};

    const EXAMPLE_RUSTSEC_ID: &str = "RUSTSEC-2018-0001";
    const EXAMPLE_CVE_ID: &str = "CVE-2017-1000168";
    const EXAMPLE_TALOS_ID: &str = "TALOS-2017-0468";
    const EXAMPLE_UNKNOWN_ID: &str = "Anonymous-42";

    #[test]
    fn rustsec_id_test() {
        let rustsec_id = AdvisoryId::new(EXAMPLE_RUSTSEC_ID).unwrap();
        assert!(rustsec_id.is_rustsec());
        assert_eq!(rustsec_id.year().unwrap(), 2018);
        assert_eq!(
            rustsec_id.url().unwrap(),
            "https://rustsec.org/advisories/RUSTSEC-2018-0001"
        );
    }

    #[test]
    fn cve_id_test() {
        let cve_id = AdvisoryId::new(EXAMPLE_CVE_ID).unwrap();
        assert!(cve_id.is_cve());
        assert_eq!(cve_id.year().unwrap(), 2017);
        assert_eq!(
            cve_id.url().unwrap(),
            "https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2017-1000168"
        );
    }

    #[test]
    fn talos_id_test() {
        let talos_id = AdvisoryId::new(EXAMPLE_TALOS_ID).unwrap();
        assert_eq!(talos_id.kind(), AdvisoryIdKind::TALOS);
        assert_eq!(talos_id.year().unwrap(), 2017);
        assert_eq!(
            talos_id.url().unwrap(),
            "https://www.talosintelligence.com/reports/TALOS-2017-0468"
        );
    }

    #[test]
    fn unknown_id_test() {
        let unknown_id = AdvisoryId::new(EXAMPLE_UNKNOWN_ID).unwrap();
        assert!(unknown_id.is_unknown());
        assert!(unknown_id.year().is_none());
        assert!(unknown_id.url().is_none());
    }
}
