//! Report Confidence (RC)

use crate::{Error, Metric, MetricType, Result};
use alloc::borrow::ToOwned;
use core::{fmt, str::FromStr};

/// Report Confidence (RC) - CVSS v3.1 Temporal Metric Group
/// > This metric measures the degree of confidence in the existence of the
/// > vulnerability and the credibility of the known technical details.
/// > Sometimes only the existence of vulnerabilities is publicized, but without
/// > specific details.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ReportConfidence {
    /// Not Defined (X)
    /// > Assigning this value indicates there is insufficient information to
    /// > choose one of the other values, and has no impact on the overall
    /// > Temporal Score, i.e., it has the same effect on scoring as assigning
    /// > Confirmed.
    NotDefined,

    /// Confirmed (C)
    /// > Detailed reports exist, or functional reproduction is possible
    /// > (functional exploits may provide this). Source code is available to
    /// > independently verify the assertions of the research, or the author or
    /// > vendor of the affected code has confirmed the presence of the
    /// > vulnerability.
    Confirmed,

    /// Reasonable (R)
    /// > Significant details are published, but researchers either do not have
    /// > full confidence in the root cause, or do not have access to source
    /// > code to fully confirm all of the interactions that may lead to the
    /// > result. Reasonable confidence exists, however, that the bug is
    /// > reproducible and at least one impact is able to be verified
    /// > (proof-of-concept exploits may provide this). An example is a detailed
    /// > write-up of research into a vulnerability with an explanation
    /// > (possibly obfuscated or “left as an exercise to the reader”) that
    /// > gives assurances on how to reproduce the results.
    Reasonable,

    /// Unknown (U)
    /// > There are reports of impacts that indicate a vulnerability is present.
    /// > The reports indicate that the cause of the vulnerability is unknown,
    /// > or reports may differ on the cause or impacts of the vulnerability.
    /// > Reporters are uncertain of the true nature of the vulnerability, and
    /// > there is little confidence in the validity of the reports or whether a
    /// > static Base Score can be applied given the differences described. An
    /// > example is a bug report which notes that an intermittent but
    /// > non-reproducible crash occurs, with evidence of memory corruption
    /// > suggesting that denial of service, or possible more serious impacts,
    /// > may result.
    Unknown,
}

impl Metric for ReportConfidence {
    const TYPE: MetricType = MetricType::RC;

    fn score(self) -> f64 {
        match self {
            ReportConfidence::NotDefined => 1.0,
            ReportConfidence::Confirmed => 1.0,
            ReportConfidence::Reasonable => 0.96,
            ReportConfidence::Unknown => 0.92,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            ReportConfidence::NotDefined => "X",
            ReportConfidence::Confirmed => "C",
            ReportConfidence::Reasonable => "R",
            ReportConfidence::Unknown => "U",
        }
    }
}

impl fmt::Display for ReportConfidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", Self::name(), self.as_str())
    }
}

impl FromStr for ReportConfidence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "X" => Ok(ReportConfidence::NotDefined),
            "C" => Ok(ReportConfidence::Confirmed),
            "R" => Ok(ReportConfidence::Reasonable),
            "U" => Ok(ReportConfidence::Unknown),
            _ => Err(Error::InvalidMetric {
                metric_type: Self::TYPE,
                value: s.to_owned(),
            }),
        }
    }
}
