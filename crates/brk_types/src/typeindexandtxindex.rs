use byteview::ByteView;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::{TxIndex, TypeIndex};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Default,
    Serialize,
    Hash,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
)]
pub struct TypeIndexAndTxIndex(u64);

impl TypeIndexAndTxIndex {
    pub fn typeindex(&self) -> u32 {
        (self.0 >> 32) as u32
    }

    pub fn txindex(&self) -> u32 {
        self.0 as u32
    }

    pub fn to_be_bytes(&self) -> [u8; 8] {
        self.0.to_be_bytes()
    }
}

impl From<(TypeIndex, TxIndex)> for TypeIndexAndTxIndex {
    #[inline]
    fn from((typeindex, txindex): (TypeIndex, TxIndex)) -> Self {
        Self((u64::from(typeindex) << 32) | u64::from(txindex))
    }
}

impl From<ByteView> for TypeIndexAndTxIndex {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self::from(&*value)
    }
}

impl From<&[u8]> for TypeIndexAndTxIndex {
    #[inline]
    fn from(value: &[u8]) -> Self {
        let typeindex = TypeIndex::from(&value[0..4]);
        let txindex = TxIndex::from(&value[4..8]);
        Self::from((typeindex, txindex))
    }
}

impl From<TypeIndexAndTxIndex> for ByteView {
    #[inline]
    fn from(value: TypeIndexAndTxIndex) -> Self {
        ByteView::from(&value)
    }
}
impl From<&TypeIndexAndTxIndex> for ByteView {
    #[inline]
    fn from(value: &TypeIndexAndTxIndex) -> Self {
        ByteView::from(value.0.to_be_bytes().as_slice())
    }
}

impl From<TypeIndexAndTxIndex> for u64 {
    #[inline]
    fn from(value: TypeIndexAndTxIndex) -> Self {
        value.0
    }
}
