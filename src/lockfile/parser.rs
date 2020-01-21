//! serde-based lockfile parser
//!
//! This uses a custom visitor impl (extracted from the proc macro one)
//! which allows us to do postprocessing to detect the V1 vs V2 formats
//! and ensure the end-user is supplied a consistent representation.

use super::{version::ResolveVersion, Lockfile};
use crate::{metadata::Metadata, package::Package};
use serde::{de, Deserialize};
use std::{fmt, marker::PhantomData};

impl<'de> Deserialize<'de> for Lockfile {
    fn deserialize<D>(deserialize: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        /// Field names in `Cargo.lock`
        const FIELDS: &[&str] = &["package", "metadata"];

        /// Fields in `Cargo.lock`
        enum Field {
            Package,
            Metadata,
        }

        /// Serde visitor for fields in `Cargo.lock`
        struct FieldVisitor;

        impl<'de> de::Visitor<'de> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("field identifier")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                match value {
                    "package" => Ok(Field::Package),
                    "metadata" => Ok(Field::Metadata),
                    _ => Err(de::Error::unknown_field(value, FIELDS)),
                }
            }
        }

        impl<'de> Deserialize<'de> for Field {
            #[inline]
            fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                de::Deserializer::deserialize_identifier(deserializer, FieldVisitor)
            }
        }

        /// Lockfile visitor
        struct Visitor<'de> {
            marker: PhantomData<Lockfile>,
            lifetime: PhantomData<&'de ()>,
        }

        impl<'de> de::Visitor<'de> for Visitor<'de> {
            type Value = Lockfile;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("struct Lockfile")
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut packages: Option<Vec<Package>> = None;
                let mut metadata: Option<Metadata> = None;

                while let Some(key) = de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Package => {
                            if packages.is_some() {
                                return Err(<A::Error as de::Error>::duplicate_field("package"));
                            }

                            packages = Some(de::MapAccess::next_value::<Vec<Package>>(&mut map)?);
                        }
                        Field::Metadata => {
                            if metadata.is_some() {
                                return Err(<A::Error as de::Error>::duplicate_field("metadata"));
                            }

                            metadata = Some(de::MapAccess::next_value::<Metadata>(&mut map)?);
                        }
                    }
                }

                let packages =
                    packages.ok_or_else(|| <A::Error as de::Error>::missing_field("package"))?;

                let metadata = metadata.unwrap_or_default();

                // Autodetect Cargo.lock resolve version based on its contents
                let version =
                    ResolveVersion::detect(&packages, &metadata).map_err(de::Error::custom)?;

                Ok(Lockfile {
                    version,
                    packages,
                    metadata,
                })
            }
        }

        de::Deserializer::deserialize_struct(
            deserialize,
            "Lockfile",
            FIELDS,
            Visitor {
                marker: PhantomData,
                lifetime: PhantomData,
            },
        )
    }
}
