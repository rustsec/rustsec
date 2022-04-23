
use core::convert::TryFrom;

impl TryFrom<u8> for PointerWidth {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            64 => Ok(PointerWidth::U64),
            32 => Ok(PointerWidth::U32),
            16 => Ok(PointerWidth::U16),
            _ => Err("Invalid pointer width!"),
        }
    }
}

impl From<PointerWidth> for u8 {
    fn from(value: PointerWidth) -> Self {
        match value {
            PointerWidth::U64 => 64,
            PointerWidth::U32 => 32,
            PointerWidth::U16 => 16,
        }
    }
}

impl fmt::Display for PointerWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl Serialize for PointerWidth {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(all(feature = "serde", feature = "std"))]
impl<'de> Deserialize<'de> for PointerWidth {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = std::string::String::deserialize(deserializer)?;
        string.parse().map_err(|_| D::Error::custom(std::format!("Unrecognized value '{}' for target_pointer_width", string)))
    }
}
