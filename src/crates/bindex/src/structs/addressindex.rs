use derive_deref::{Deref, DerefMut};
use fjall::Slice;

use super::SliceExtended;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deref, DerefMut, Default)]
pub struct Addressindex(u32);

impl Addressindex {
    pub const BYTES: usize = size_of::<Self>();

    pub fn decremented(self) -> Self {
        Self(*self - 1)
    }

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

impl From<usize> for Addressindex {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<Addressindex> for usize {
    fn from(value: Addressindex) -> Self {
        value.0 as usize
    }
}

impl TryFrom<Slice> for Addressindex {
    type Error = color_eyre::Report;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        Self::try_from(&value[..])
    }
}
impl TryFrom<&[u8]> for Addressindex {
    type Error = color_eyre::Report;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from(value.read_be_u32()?))
    }
}
impl From<Addressindex> for Slice {
    fn from(value: Addressindex) -> Self {
        value.to_be_bytes().into()
    }
}
