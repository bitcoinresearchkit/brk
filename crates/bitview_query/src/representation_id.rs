use bitcoin::hashes::{Hash, sha256d};
use brk_types::BlockHash;

/// Identity and provenance of one resolved representation.
#[derive(Debug, Clone, Copy)]
pub enum RepresentationId {
    Content(u64),
    Block(BlockHash),
}

impl RepresentationId {
    /// Identify a representation by its exact serialized bytes.
    pub fn content(bytes: &[u8]) -> Self {
        Self::Content(Self::content_hash(bytes))
    }

    pub fn content_hash(bytes: &[u8]) -> u64 {
        let digest = sha256d::Hash::hash(bytes).to_byte_array();
        u64::from_le_bytes(digest[..8].try_into().unwrap())
    }
}
