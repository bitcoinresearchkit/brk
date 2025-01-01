use derive_deref::{Deref, DerefMut};

use super::SliceExtended;

#[derive(Debug, Clone, Copy, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u8);

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<fjall::Slice> for Version {
    fn from(slice: fjall::Slice) -> Self {
        Self(slice.read_u8())
    }
}
