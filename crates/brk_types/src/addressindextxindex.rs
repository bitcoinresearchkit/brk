use std::hash::Hash;

use byteview::ByteView;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::copy_first_8bytes;

use super::{TxIndex, TypeIndex};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Serialize,
    FromBytes,
    IntoBytes,
    Immutable,
    KnownLayout,
    Hash,
)]
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
        Self::from(&*value)
    }
}

impl From<&[u8]> for AddressIndexTxIndex {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self(u64::from_be_bytes(copy_first_8bytes(value).unwrap()))
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
