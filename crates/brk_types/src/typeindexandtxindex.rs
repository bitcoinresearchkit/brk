use std::{cmp::Ordering, mem};

use byteview::ByteView;
use redb::{Key, TypeName, Value};
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

impl Value for TypeIndexAndTxIndex {
    type SelfType<'a> = TypeIndexAndTxIndex;
    type AsBytes<'a>
        = [u8; mem::size_of::<u64>()]
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        Some(mem::size_of::<u64>())
    }

    fn from_bytes<'a>(data: &'a [u8]) -> TypeIndexAndTxIndex
    where
        Self: 'a,
    {
        TypeIndexAndTxIndex(u64::from_le_bytes(data.try_into().unwrap()))
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> [u8; mem::size_of::<u64>()]
    where
        Self: 'a,
        Self: 'b,
    {
        value.0.to_le_bytes()
    }

    fn type_name() -> TypeName {
        TypeName::new("TypeIndexAndTxIndex")
    }
}

impl Key for TypeIndexAndTxIndex {
    fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
        Self::from_bytes(data1).cmp(&Self::from_bytes(data2))
    }
}
