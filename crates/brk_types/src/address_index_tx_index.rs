use std::hash::Hash;

use byteview::ByteView;
use serde::Serialize;
use vecdb::Bytes;

use super::{TxIndex, TypeIndex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Bytes, Hash)]
pub struct AddressIndexTxIndex(u64);

impl AddressIndexTxIndex {
    pub fn address_index(&self) -> u32 {
        (self.0 >> 32) as u32
    }

    pub fn tx_index(&self) -> TxIndex {
        TxIndex::from(self.0 as u32)
    }

    pub fn min_for_address(address_index: TypeIndex) -> Self {
        Self(u64::from(address_index) << 32)
    }

    pub fn max_for_address(address_index: TypeIndex) -> Self {
        Self((u64::from(address_index) << 32) | u64::MAX >> 32)
    }
}

impl From<(TypeIndex, TxIndex)> for AddressIndexTxIndex {
    #[inline]
    fn from((address_index, tx_index): (TypeIndex, TxIndex)) -> Self {
        Self((u64::from(address_index) << 32) | u64::from(tx_index))
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
