use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::PAGE_SIZE;

#[derive(Debug, Clone, FromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct Region {
    /// Must be multiple of 4096
    start: u64,
    length: u64,
    /// Must be multiple of 4096
    reserved: u64,
}

pub const SIZE_OF_REGION: usize = size_of::<Region>();

impl Region {
    pub fn new(start: u64, length: u64, reserved: u64) -> Self {
        debug_assert!(reserved > 0);
        debug_assert!(start % PAGE_SIZE == 0);
        debug_assert!(reserved % PAGE_SIZE == 0);
        debug_assert!(length <= reserved);

        Self {
            start,
            length,
            reserved,
        }
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn set_start(&mut self, start: u64) {
        debug_assert!(start % PAGE_SIZE == 0);
        self.start = start
    }

    pub fn len(&self) -> u64 {
        self.length
    }

    pub fn set_len(&mut self, len: u64) {
        self.length = len
    }

    pub fn reserved(&self) -> u64 {
        self.reserved
    }

    pub fn set_reserved(&mut self, reserved: u64) {
        self.reserved = reserved;
    }

    pub fn left(&self) -> u64 {
        self.reserved - self.length
    }
}
