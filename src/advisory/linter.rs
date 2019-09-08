//! Advisory linter: ensure advisories are well-formed according to the
//! currently valid set of fields.
//!
//! This is run in CI at the time advisories are submitted.

use super::{Advisory, Category};
use std::{fmt, fs, path::Path};

/// Lint information about a particular advisory
#[derive(Debug)]
pub struct Linter {
    /// Advisory being linted
    advisory: Advisory,

    /// Errors detected during linting
    errors: Vec<Error>,
}

impl Linter {
    /// Lint the advisory TOML file located at the given path
    pub fn lint_file<P: AsRef<Path>>(path: P) -> Result<Self, crate::Error> {
        let path = path.as_ref();
        let toml = fs::read_to_string(path).map_err(|e| {
            format_err!(
                crate::ErrorKind::Io,
                "couldn't open {}: {}",
                path.display(),
                e
            )
        })?;

        Self::lint_string(&toml)
    }

    /// Lint the given advisory TOML string
    pub fn lint_string(s: &str) -> Result<Self, crate::Error> {
        // Ensure the advisory parses according to the normal parser first
        let advisory = s.parse::<Advisory>()?;

        // Get a raw TOML value representing the document for linting
        let toml_value = s.parse::<toml::Value>()?;

        let mut linter = Self {
            advisory,
            errors: vec![],
        };

        linter.lint_advisory(&toml_value);
        Ok(linter)
    }

    /// Get the parsed advisory
    pub fn advisory(&self) -> &Advisory {
        &self.advisory
    }

    /// Get the errors that occurred during linting
    pub fn errors(&self) -> &[Error] {
        self.errors.as_slice()
    }

    /// Lint the provided TOML value as the toplevel table of an advisory
    fn lint_advisory(&mut self, advisory: &toml::Value) {
        if let Some(table) = advisory.as_table() {
            for (key, value) in table {
                match key.as_str() {
                    "advisory" => self.lint_metadata(value),
                    "versions" => self.lint_versions(value),
                    "affected" => self.lint_affected(value),
                    _ => self.errors.push(Error {
                        kind: ErrorKind::key(key),
                        section: None,
                        msg: None,
                    }),
                }
            }
        } else {
            self.errors.push(Error {
                kind: ErrorKind::Malformed,
                section: None,
                msg: Some("expected table"),
            });
        }
    }

    /// Lint the `[advisory]` metadata section
    fn lint_metadata(&mut self, metadata: &toml::Value) {
        if let Some(table) = metadata.as_table() {
            for (key, value) in table {
                match key.as_str() {
                    "id" => {
                        if self.advisory.metadata.id.is_other() {
                            self.errors.push(Error {
                                kind: ErrorKind::value("id", value.to_string()),
                                section: Some("advisory"),
                                msg: Some("unknown advisory ID type"),
                            });
                        }
                    }
                    "categories" => {
                        for category in &self.advisory.metadata.categories {
                            if let Category::Other(other) = category {
                                self.errors.push(Error {
                                    kind: ErrorKind::value("category", other.to_string()),
                                    section: Some("advisory"),
                                    msg: Some("unknown category"),
                                });
                            }
                        }
                    }
                    "collection" => self.errors.push(Error {
                        kind: ErrorKind::Malformed,
                        section: Some("advisory"),
                        msg: Some("collection shouldn't be explicit; inferred by location"),
                    }),
                    "url" => {
                        if let Some(url) = value.as_str() {
                            if !url.starts_with("https://") {
                                self.errors.push(Error {
                                    kind: ErrorKind::value("url", value.to_string()),
                                    section: Some("advisory"),
                                    msg: Some("URL must start with https://"),
                                });
                            }
                        }
                    }
                    "patched_versions" | "unaffected_versions" => (), // TODO(tarcieri): deprecate
                    "aliases" | "cvss" | "date" | "keywords" | "obsolete" | "package"
                    | "references" | "title" | "description" => (),
                    _ => self.errors.push(Error {
                        kind: ErrorKind::key(key),
                        section: Some("advisory"),
                        msg: None,
                    }),
                }
            }
        } else {
            self.errors.push(Error {
                kind: ErrorKind::Malformed,
                section: Some("advisory"),
                msg: Some("expected table"),
            });
        }
    }

    /// Lint the `[versions]` section of an advisory
    fn lint_versions(&mut self, versions: &toml::Value) {
        if let Some(table) = versions.as_table() {
            for (key, _) in table {
                match key.as_str() {
                    "patched" | "unaffected" => (),
                    _ => self.errors.push(Error {
                        kind: ErrorKind::key(key),
                        section: Some("versions"),
                        msg: None,
                    }),
                }
            }
        }
    }

    /// Lint the `[affected]` section of an advisory
    fn lint_affected(&mut self, affected: &toml::Value) {
        if let Some(table) = affected.as_table() {
            for (key, _) in table {
                match key.as_str() {
                    "functions" => {
                        for function in self.advisory.affected.as_ref().unwrap().functions.keys() {
                            if function.segments()[0].as_str()
                                != self.advisory.metadata.package.as_str()
                            {
                                self.errors.push(Error {
                                    kind: ErrorKind::value("functions", function.to_string()),
                                    section: Some("affected"),
                                    msg: Some("function path must start with crate name"),
                                });
                            }
                        }
                    }
                    "arch" | "os" => (),
                    _ => self.errors.push(Error {
                        kind: ErrorKind::key(key),
                        section: Some("affected"),
                        msg: None,
                    }),
                }
            }
        }
    }
}

/// Lint errors
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// Kind of error
    kind: ErrorKind,

    /// Section of the advisory where the error occurred
    section: Option<&'static str>,

    /// Message about why it's invalid
    msg: Option<&'static str>,
}

impl Error {
    /// Get the kind of error
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Get the section of the advisory where the error occurred
    pub fn section(&self) -> Option<&str> {
        self.section.as_ref().map(AsRef::as_ref)
    }

    /// Get an optional message about the lint failure
    pub fn msg(&self) -> Option<&str> {
        self.msg.as_ref().map(AsRef::as_ref)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.kind)?;

        if let Some(section) = &self.section {
            write!(f, " in [{}]", section)?;
        } else {
            write!(f, " in toplevel")?;
        }

        if let Some(msg) = &self.msg {
            write!(f, ": {}", msg)?
        }

        Ok(())
    }
}

/// Lint errors
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// Advisory is structurally malformed
    Malformed,

    /// Unknown key
    InvalidKey {
        /// Name of the key
        name: String,
    },

    /// Unknown value
    InvalidValue {
        /// Name of the key
        name: String,

        /// Invalid value
        value: String,
    },
}

impl ErrorKind {
    /// Invalid key
    pub fn key(name: &str) -> Self {
        ErrorKind::InvalidKey {
            name: name.to_owned(),
        }
    }

    /// Invalid value
    pub fn value(name: &str, value: impl Into<String>) -> Self {
        ErrorKind::InvalidValue {
            name: name.to_owned(),
            value: value.into(),
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Malformed => write!(f, "malformed content"),
            ErrorKind::InvalidKey { name } => write!(f, "invalid key `{}`", name),
            ErrorKind::InvalidValue { name, value } => {
                write!(f, "invalid value `{}` for key `{}`", value, name)
            }
        }
    }
}
