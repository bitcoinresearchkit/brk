use brk_indexer::{Addressindex, Txoutindex};
use fjall::Slice;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct AddressindexTxoutindex {
    addressindex: Addressindex,
    _padding: u32,
    txoutindex: Txoutindex,
}

impl TryFrom<Slice> for AddressindexTxoutindex {
    type Error = storable_vec::Error;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
    }
}
impl From<AddressindexTxoutindex> for Slice {
    fn from(value: AddressindexTxoutindex) -> Self {
        Self::new(value.as_bytes())
    }
}
