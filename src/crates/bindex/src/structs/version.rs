use derive_deref::{Deref, DerefMut};

use super::SliceExtended;

#[derive(Debug, Clone, Copy, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u8);

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl TryFrom<Slice> for Version {
    type Error = color_eyre::Report;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        Self::try_from(&value[..])
    }
}
impl TryFrom<&[u8]> for Version {
    type Error = color_eyre::Report;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from(value.read_be_u8()?))
    }
}
impl From<Version> for Slice {
    fn from(value: Version) -> Self {
        value.to_be_bytes().into()
    }
}
