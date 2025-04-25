use byteview::ByteView;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::Error;

use super::{AddressIndex, Outputindex};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Immutable, IntoBytes, KnownLayout, FromBytes,
)]
#[repr(C)]
pub struct AddressIndexOutputIndex {
    addressindex: AddressIndex,
    _padding: u32,
    outputindex: Outputindex,
}

impl TryFrom<ByteView> for AddressIndexOutputIndex {
    type Error = Error;
    fn try_from(value: ByteView) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
    }
}
impl From<AddressIndexOutputIndex> for ByteView {
    fn from(value: AddressIndexOutputIndex) -> Self {
        Self::new(value.as_bytes())
    }
}
