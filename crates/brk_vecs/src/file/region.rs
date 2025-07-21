// use std::sync::Arc;

use bincode::{Decode, Encode};
// use parking_lot::{RwLock, RwLockReadGuard};

use crate::PAGE_SIZE;

// #[derive(Debug, Encode, Decode)]
#[derive(Debug, Encode, Decode)]
pub struct Region {
    // Bad name
    /// Must be multiple of 4096
    start: usize,
    length: usize,
    /// Must be multiple of 4096
    reserved: usize,
    // lock: Arc<RwLock<()>>,
    // variant: usize, // Raw or Compressed or something else ? to know if there is a header ? Since blocks 4096, storing headers individually would be dumb
}

impl Region {
    pub fn new(start: usize, length: usize, reserved: usize) -> Self {
        assert!(reserved > 0);
        assert!(start % PAGE_SIZE == 0);
        assert!(reserved % PAGE_SIZE == 0);
        assert!(length <= reserved);

        Self {
            start,
            length,
            reserved,
            // lock: Arc::new(RwLock::new(())),
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn reserved(&self) -> usize {
        self.reserved
    }

    // pub fn lock(&self) -> RwLockReadGuard<'_, ()> {
    //     self.lock.read()
    // }
}

// #[derive(Debug, Encode, Decode)]
// pub struct RegionInner {
//     start: usize,
//     length: usize,
//     reserved: usize,
// }

// impl From<Region> for RegionInner {
//     fn from(value: Region) -> Self {
//         Self {
//             start: value.start,
//             length: value.length,
//             reserved: value.reserved,
//         }
//     }
// }

// impl From<RegionInner> for Region {
//     fn from(value: RegionInner) -> Self {
//         Self {
//             start: value.start,
//             length: value.length,
//             reserved: value.reserved,
//             lock: Arc::new(RwLock::new(())),
//         }
//     }
// }
