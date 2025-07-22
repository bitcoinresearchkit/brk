use bincode::{Decode, Encode};

use crate::PAGE_SIZE;

#[derive(Debug, Clone, Encode, Decode)]
pub struct Region {
    /// Must be multiple of 4096
    start: u64,
    length: u64,
    /// Must be multiple of 4096
    reserved: u64,
}

impl Region {
    pub fn new(start: u64, length: u64, reserved: u64) -> Self {
        assert!(reserved > 0);
        assert!(start % PAGE_SIZE == 0);
        assert!(reserved % PAGE_SIZE == 0);
        assert!(length <= reserved);

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
        assert!(start % PAGE_SIZE == 0);
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
