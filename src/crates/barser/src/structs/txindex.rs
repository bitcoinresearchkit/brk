use derive_deref::{Deref, DerefMut};

use super::SliceExtended;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, DerefMut, Default)]
pub struct Txindex(u32);

impl Txindex {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }
}

impl From<u32> for Txindex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u64> for Txindex {
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}
impl From<Txindex> for u64 {
    fn from(value: Txindex) -> Self {
        value.0 as u64
    }
}

impl From<fjall::Slice> for Txindex {
    fn from(slice: fjall::Slice) -> Self {
        Self(slice.read_u32())
    }
}
impl From<Txindex> for fjall::Slice {
    fn from(value: Txindex) -> Self {
        value.to_be_bytes().into()
    }
}
