use std::hash::Hash;

use byteview::ByteView;
use serde::Serialize;
use vecdb::Bytes;

use super::{TxIndex, TypeIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Bytes, Hash)]
pub struct AddressIndexTxIndex(u64);

impl AddressIndexTxIndex {
    pub fn addressindex(&self) -> u32 {
        (self.0 >> 32) as u32
    }

    pub fn txindex(&self) -> u32 {
        self.0 as u32
    }
}

impl From<(TypeIndex, TxIndex)> for AddressIndexTxIndex {
    #[inline]
    fn from((addressindex, txindex): (TypeIndex, TxIndex)) -> Self {
        Self((u64::from(addressindex) << 32) | u64::from(txindex))
    }
}

impl From<ByteView> for AddressIndexTxIndex {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self(u64::from_be_bytes((&*value).try_into().unwrap()))
    }
}

impl From<AddressIndexTxIndex> for ByteView {
    #[inline]
    fn from(value: AddressIndexTxIndex) -> Self {
        ByteView::from(&value)
    }
}
impl From<&AddressIndexTxIndex> for ByteView {
    #[inline]
    fn from(value: &AddressIndexTxIndex) -> Self {
        ByteView::from(value.0.to_be_bytes())
    }
}
