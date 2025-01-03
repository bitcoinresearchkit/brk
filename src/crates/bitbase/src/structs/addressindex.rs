use derive_deref::{Deref, DerefMut};
use fjall::Slice;

use super::SliceExtended;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, DerefMut, Default)]
pub struct Addressindex(u32);

impl Addressindex {
    pub const BYTES: usize = size_of::<Self>();

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }
}

impl From<u32> for Addressindex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u64> for Addressindex {
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}
impl From<Addressindex> for u64 {
    fn from(value: Addressindex) -> Self {
        value.0 as u64
    }
}

impl From<Slice> for Addressindex {
    fn from(slice: Slice) -> Self {
        Self(slice.read_u32())
    }
}
impl From<Addressindex> for Slice {
    fn from(value: Addressindex) -> Self {
        value.to_be_bytes().into()
    }
}
