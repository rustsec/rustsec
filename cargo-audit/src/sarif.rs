//! SARIF (Static Analysis Results Interchange Format) output support
//!
//! This module provides functionality to convert cargo-audit reports to SARIF format,
//! which can be uploaded to GitHub Security tab and other security analysis platforms.
//!
//! SARIF is an OASIS Standard that defines a common format for static analysis tools
//! to report their findings. This implementation follows SARIF 2.1.0 specification
//! and is compatible with GitHub's code scanning requirements.

use rustsec::{Report, Vulnerability, Warning, WarningKind, advisory};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// SARIF schema URI
const SARIF_SCHEMA: &str = "https://json.schemastore.org/sarif-2.1.0.json";
/// SARIF version
const SARIF_VERSION: &str = "2.1.0";
/// Tool name
const TOOL_NAME: &str = "cargo-audit";
/// Tool version
const TOOL_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Security tags for vulnerabilities
const VULNERABILITY_TAGS: &[&str] = &["security", "vulnerability"];
/// Security tags for warnings
const WARNING_TAGS: &[&str] = &["security", "warning"];

/// SARIF log root object
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SarifLog {
    /// URI of the SARIF schema
    #[serde(rename = "$schema")]
    pub schema: String,
    /// SARIF format version
    pub version: String,
    /// Array of analysis runs
    pub runs: Vec<Run>,
}

impl SarifLog {
    /// Convert a cargo-audit report to SARIF format
    pub fn from_report(report: &Report, cargo_lock_path: &str) -> Self {
        let mut rules = Vec::new();
        let mut seen_rules = HashSet::new();
        let mut results = Vec::new();

        for vuln in &report.vulnerabilities.list {
            let rule_id = vuln.advisory.id.to_string();

            if seen_rules.insert(rule_id.clone()) {
                rules.push(ReportingDescriptor::from_advisory(&vuln.advisory, true));
            }

            results.push(Result::from_vulnerability(vuln, cargo_lock_path));
        }

        for (warning_kind, warnings) in &report.warnings {
            for warning in warnings {
                let rule_id = if let Some(advisory) = &warning.advisory {
                    advisory.id.to_string()
                } else {
                    format!("{warning_kind:?}").to_lowercase()
                };

                if seen_rules.insert(rule_id) {
                    let descriptor = if let Some(advisory) = &warning.advisory {
                        ReportingDescriptor::from_advisory(advisory, false)
                    } else {
                        ReportingDescriptor::from_warning_kind(*warning_kind)
                    };
                    rules.push(descriptor);
                }

                results.push(Result::from_warning(warning, cargo_lock_path));
            }
        }

        SarifLog {
            schema: SARIF_SCHEMA.to_string(),
            version: SARIF_VERSION.to_string(),
            runs: vec![Run {
                tool: Tool {
                    driver: ToolComponent {
                        name: TOOL_NAME.to_string(),
                        version: Some(TOOL_VERSION.to_string()),
                        semantic_version: Some(TOOL_VERSION.to_string()),
                        rules,
                    },
                },
                results,
                automation_details: None,
            }],
        }
    }
}

/// A run represents a single invocation of an analysis tool
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    /// Tool information for this run
    pub tool: Tool,
    /// Array of results (findings) from the analysis
    pub results: Vec<Result>,
    /// Automation details to distinguish between runs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_details: Option<RunAutomationDetails>,
}

/// Tool information
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    /// The analysis tool that was run
    pub driver: ToolComponent,
}

/// Tool component (driver) information
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolComponent {
    /// Name of the tool component
    pub name: String,
    /// Tool version string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Semantic version of the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_version: Option<String>,
    /// Rules defined by this tool
    pub rules: Vec<ReportingDescriptor>,
}

/// Rule/reporting descriptor
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingDescriptor {
    /// Unique identifier for the rule
    pub id: String,
    /// Human-readable name of the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Brief description of the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<MultiformatMessageString>,
    /// Detailed description of the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_description: Option<MultiformatMessageString>,
    /// Default severity and enablement for the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_configuration: Option<ReportingConfiguration>,
    /// Help text or URI for the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<MultiformatMessageString>,
    /// Additional properties including tags and severity scores
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RuleProperties>,
}

impl ReportingDescriptor {
    /// Create a ReportingDescriptor from an advisory
    pub fn from_advisory(metadata: &advisory::Metadata, is_vulnerability: bool) -> Self {
        let tags = if is_vulnerability {
            VULNERABILITY_TAGS.iter().map(|&s| s.to_string()).collect()
        } else {
            WARNING_TAGS.iter().map(|&s| s.to_string()).collect()
        };

        let security_severity = metadata
            .cvss
            .as_ref()
            .map(|cvss| format!("{:.1}", cvss.score().value()));

        let default_level = if is_vulnerability { "error" } else { "warning" };

        ReportingDescriptor {
            id: metadata.id.to_string(),
            name: Some(metadata.id.to_string()),
            short_description: Some(MultiformatMessageString {
                text: metadata.title.clone(),
                markdown: None,
            }),
            full_description: if metadata.description.is_empty() {
                None
            } else {
                Some(MultiformatMessageString {
                    text: metadata.description.clone(),
                    markdown: None,
                })
            },
            default_configuration: Some(ReportingConfiguration {
                level: default_level.to_string(),
            }),
            help: metadata.url.as_ref().map(|url| MultiformatMessageString {
                text: format!("For more information, see: {url}"),
                markdown: Some(format!(
                    "For more information, see: [{}]({})",
                    metadata.id, url
                )),
            }),
            properties: Some(RuleProperties {
                tags: Some(tags),
                precision: Some("very-high".to_string()),
                problem_severity: if !is_vulnerability {
                    Some("warning".to_string())
                } else {
                    None
                },
                security_severity,
            }),
        }
    }

    /// Create a ReportingDescriptor from a warning kind
    pub fn from_warning_kind(kind: WarningKind) -> Self {
        let (name, description) = match kind {
            WarningKind::Unmaintained => (
                "unmaintained",
                "Package is unmaintained and may have unaddressed security vulnerabilities",
            ),
            WarningKind::Unsound => (
                "unsound",
                "Package has known soundness issues that may lead to memory safety problems",
            ),
            WarningKind::Yanked => (
                "yanked",
                "Package version has been yanked from the registry",
            ),
            _ => ("unknown", "Unknown warning type"),
        };

        ReportingDescriptor {
            id: name.to_string(),
            name: Some(name.to_string()),
            short_description: Some(MultiformatMessageString {
                text: description.to_string(),
                markdown: None,
            }),
            full_description: None,
            default_configuration: Some(ReportingConfiguration {
                level: "warning".to_string(),
            }),
            help: None,
            properties: Some(RuleProperties {
                tags: Some(WARNING_TAGS.iter().map(|&s| s.to_string()).collect()),
                precision: Some("high".to_string()),
                problem_severity: Some("warning".to_string()),
                security_severity: None,
            }),
        }
    }
}

/// Rule properties
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleProperties {
    /// Tags associated with the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Precision of the rule (e.g., "very-high", "high")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    /// Problem severity for non-security issues
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "problem.severity")]
    pub problem_severity: Option<String>,
    /// CVSS score as a string (0.0-10.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "security-severity")]
    pub security_severity: Option<String>,
}

/// Reporting configuration for a rule
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingConfiguration {
    /// Default level for the rule ("error", "warning", "note")
    pub level: String,
}

/// Message with optional markdown
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiformatMessageString {
    /// Plain text message
    pub text: String,
    /// Optional markdown-formatted message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,
}

/// A result (finding/alert)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    /// ID of the rule that was violated
    pub rule_id: String,
    /// Message describing the result
    pub message: Message,
    /// Severity level of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// Locations where the issue was detected
    pub locations: Vec<Location>,
    /// Fingerprints for result matching
    pub partial_fingerprints: HashMap<String, String>,
}

impl Result {
    /// Create a Result from a vulnerability
    pub fn from_vulnerability(vuln: &Vulnerability, cargo_lock_path: &str) -> Self {
        let fingerprint = format!(
            "{}:{}:{}",
            vuln.advisory.id, vuln.package.name, vuln.package.version
        );

        Result {
            rule_id: vuln.advisory.id.to_string(),
            message: Message {
                text: format!(
                    "{} {} is vulnerable to {} ({})",
                    vuln.package.name, vuln.package.version, vuln.advisory.id, vuln.advisory.title
                ),
            },
            level: Some("error".to_string()),
            locations: vec![Location {
                physical_location: PhysicalLocation {
                    artifact_location: ArtifactLocation {
                        uri: cargo_lock_path.to_string(),
                    },
                    region: Region {
                        start_line: 1,
                        start_column: None,
                        end_line: None,
                        end_column: None,
                    },
                },
            }],
            partial_fingerprints: {
                let mut fingerprints = HashMap::new();
                // Use a custom fingerprint key instead of primaryLocationLineHash
                // to avoid conflicts with GitHub's calculated fingerprints
                fingerprints.insert("cargo-audit/advisory-fingerprint".to_string(), fingerprint);
                fingerprints
            },
        }
    }

    /// Create a Result from a warning
    pub fn from_warning(warning: &Warning, cargo_lock_path: &str) -> Self {
        let rule_id = if let Some(advisory) = &warning.advisory {
            advisory.id.to_string()
        } else {
            format!("{:?}", warning.kind).to_lowercase()
        };

        let message_text = if let Some(advisory) = &warning.advisory {
            format!(
                "{} {} has a {} warning: {}",
                warning.package.name,
                warning.package.version,
                warning.kind.as_str(),
                advisory.title
            )
        } else {
            format!(
                "{} {} has a {} warning",
                warning.package.name,
                warning.package.version,
                warning.kind.as_str()
            )
        };

        let fingerprint = format!(
            "{}:{}:{}",
            rule_id, warning.package.name, warning.package.version
        );

        Result {
            rule_id,
            message: Message { text: message_text },
            level: Some("warning".to_string()),
            locations: vec![Location {
                physical_location: PhysicalLocation {
                    artifact_location: ArtifactLocation {
                        uri: cargo_lock_path.to_string(),
                    },
                    region: Region {
                        start_line: 1,
                        start_column: None,
                        end_line: None,
                        end_column: None,
                    },
                },
            }],
            partial_fingerprints: {
                let mut fingerprints = HashMap::new();
                // Use a custom fingerprint key instead of primaryLocationLineHash
                // to avoid conflicts with GitHub's calculated fingerprints
                fingerprints.insert("cargo-audit/advisory-fingerprint".to_string(), fingerprint);
                fingerprints
            },
        }
    }
}

/// Simple message
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// The message text
    pub text: String,
}

/// Location of a finding
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// Physical location of the finding
    pub physical_location: PhysicalLocation,
}

/// Physical location in a file
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalLocation {
    /// The artifact (file) containing the issue
    pub artifact_location: ArtifactLocation,
    /// Region within the artifact
    pub region: Region,
}

/// Artifact (file) location
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactLocation {
    /// URI of the artifact
    pub uri: String,
}

/// Region within a file
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    /// Starting line number (1-based)
    pub start_line: u32,
    /// Starting column number (1-based)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_column: Option<u32>,
    /// Ending line number (1-based)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<u32>,
    /// Ending column number (1-based)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_column: Option<u32>,
}

/// Run automation details for distinguishing multiple runs
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunAutomationDetails {
    /// Unique identifier for the run
    pub id: String,
}
