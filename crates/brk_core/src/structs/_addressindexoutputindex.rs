use byteview::ByteView;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

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

impl From<ByteView> for AddressIndexOutputIndex {
    fn from(value: ByteView) -> Self {
        Ok(Self::read_from_bytes(&value)?)
    }
}
impl From<AddressIndexOutputIndex> for ByteView {
    fn from(value: AddressIndexOutputIndex) -> Self {
        Self::new(value.as_bytes())
    }
}
