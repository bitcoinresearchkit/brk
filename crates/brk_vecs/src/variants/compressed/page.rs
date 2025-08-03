use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Clone, IntoBytes, Immutable, FromBytes, KnownLayout)]
#[repr(C)]
pub struct Page {
    pub start: u64,
    pub bytes: u32,
    pub values: u32,
}

impl Page {
    pub fn new(start: u64, bytes: u32, values: u32) -> Self {
        Self {
            start,
            bytes,
            values,
        }
    }
}
