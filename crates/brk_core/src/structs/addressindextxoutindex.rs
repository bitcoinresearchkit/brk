use byteview::ByteView;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::Error;

use super::{Addressindex, Txoutindex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Immutable, IntoBytes, KnownLayout, FromBytes)]
#[repr(C)]
pub struct AddressindexTxoutindex {
    addressindex: Addressindex,
    _padding: u32,
    txoutindex: Txoutindex,
}

impl TryFrom<ByteView> for AddressindexTxoutindex {
    type Error = Error;
    fn try_from(value: ByteView) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
    }
}
impl From<AddressindexTxoutindex> for ByteView {
    fn from(value: AddressindexTxoutindex) -> Self {
        Self::new(value.as_bytes())
    }
}
