use serde::Serialize;
use vecdb::StoredCompressed;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(
    Debug, Clone, Copy, Serialize, FromBytes, Immutable, IntoBytes, KnownLayout, StoredCompressed,
)]
pub struct BlkPosition(u64);

impl BlkPosition {
    pub fn new(blk_index: u16, offset: u32) -> Self {
        Self(((blk_index as u64) << 32) | offset as u64)
    }

    pub fn blk_index(&self) -> u16 {
        (self.0 >> 31) as u16
    }

    pub fn offset(&self) -> u32 {
        self.0 as u32
    }
}
