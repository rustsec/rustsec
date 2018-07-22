use error::Error;

/// Digital signatures (in OpenPGP format) on commits to the repository
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature(Vec<u8>);

impl Signature {
    /// Parse a signature from a Git commit
    // TODO: actually verify the signature is well-structured
    pub fn new<T: Into<Vec<u8>>>(into_vec: T) -> Result<Self, Error> {
        Ok(Signature(into_vec.into()))
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
