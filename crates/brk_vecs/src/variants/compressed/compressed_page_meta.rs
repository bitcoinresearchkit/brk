use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Clone, IntoBytes, Immutable, FromBytes, KnownLayout)]
pub struct CompressedPageMetadata {
    pub start: u64,
    pub bytes_len: u32,
    pub values_len: u32,
}

impl CompressedPageMetadata {
    pub fn new(start: u64, bytes_len: u32, values_len: u32) -> Self {
        Self {
            start,
            bytes_len,
            values_len,
        }
    }
}
