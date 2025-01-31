use bindex::{Addressindex, Txoutindex};
use fjall::Slice;
use unsafe_slice_serde::UnsafeSliceSerde;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddressindexTxoutindex {
    addressindex: Addressindex,
    txoutindex: Txoutindex,
}

impl TryFrom<Slice> for AddressindexTxoutindex {
    type Error = unsafe_slice_serde::Error;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        Ok(*Self::unsafe_try_from_slice(&value)?)
    }
}
impl From<AddressindexTxoutindex> for Slice {
    fn from(value: AddressindexTxoutindex) -> Self {
        Self::new(value.unsafe_as_slice())
    }
}
