use std::ops::Add;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Formattable, Pco};

/// Position within a .blk file, encoding file index and byte offset
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pco, JsonSchema)]
pub struct BlkPosition(u64);

impl BlkPosition {
    pub fn new(blk_index: u16, offset: u32) -> Self {
        Self(((blk_index as u64) << 32) | offset as u64)
    }

    pub fn blk_index(&self) -> u16 {
        (self.0 >> 32) as u16
    }

    pub fn offset(&self) -> u32 {
        self.0 as u32
    }
}

impl Add<u32> for BlkPosition {
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        Self(self.0 + rhs as u64)
    }
}

impl std::fmt::Display for BlkPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

impl Formattable for BlkPosition {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        false
    }
}
