use tame_index::external::gix;

/// ID (i.e. SHA-1 hash) of a git commit
/// 
/// This is a wrapper around [gix::ObjectId] to prevent gix semver changes
/// also breaking semver for `rustsec` crate.
#[cfg_attr(docsrs, doc(cfg(feature = "git")))]
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
pub struct CommitHash {
    hash: gix::ObjectId,
}

impl CommitHash {
    // Conversions to/from `gix` are only pub(crate)
    // to avoid leaking `gix` types and semver into the external API
    pub(crate) fn to_gix(self) -> gix::ObjectId {
        self.hash
    }

    pub(crate) fn from_gix(hash: gix::ObjectId) -> Self {
        CommitHash { hash }
    }

    /// Interpret this object id as raw byte slice.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.hash.as_bytes()
    }

    /// Display the hash as a hexadecimal string.
    #[inline]
    pub fn to_hex(&self) -> String {
        self.hash.to_hex().to_string()
    }
}