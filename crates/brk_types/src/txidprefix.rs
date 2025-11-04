use std::{cmp::Ordering, mem};

use byteview::ByteView;
use derive_deref::Deref;
use redb::{Key, TypeName, Value};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::copy_first_8bytes;

use super::Txid;

#[derive(
    Debug,
    Deref,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Hash,
)]
pub struct TxidPrefix(u64);

impl From<Txid> for TxidPrefix {
    #[inline]
    fn from(value: Txid) -> Self {
        Self::from(&value)
    }
}

impl From<&Txid> for TxidPrefix {
    #[inline]
    fn from(value: &Txid) -> Self {
        Self(u64::from_ne_bytes(copy_first_8bytes(&value[..]).unwrap()))
    }
}

impl From<ByteView> for TxidPrefix {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self::read_from_bytes(&value).unwrap()
    }
}

impl From<&TxidPrefix> for ByteView {
    #[inline]
    fn from(value: &TxidPrefix) -> Self {
        Self::new(value.as_bytes())
    }
}

impl From<TxidPrefix> for ByteView {
    #[inline]
    fn from(value: TxidPrefix) -> Self {
        Self::from(&value)
    }
}

impl From<[u8; 8]> for TxidPrefix {
    #[inline]
    fn from(value: [u8; 8]) -> Self {
        Self(u64::from_ne_bytes(value))
    }
}

impl Value for TxidPrefix {
    type SelfType<'a> = TxidPrefix;
    type AsBytes<'a>
        = [u8; mem::size_of::<u64>()]
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        Some(mem::size_of::<u64>())
    }

    fn from_bytes<'a>(data: &'a [u8]) -> TxidPrefix
    where
        Self: 'a,
    {
        TxidPrefix(u64::from_le_bytes(data.try_into().unwrap()))
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> [u8; mem::size_of::<u64>()]
    where
        Self: 'a,
        Self: 'b,
    {
        value.0.to_le_bytes()
    }

    fn type_name() -> TypeName {
        TypeName::new("TxidPrefix")
    }
}

impl Key for TxidPrefix {
    fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
        Self::from_bytes(data1).cmp(&Self::from_bytes(data2))
    }
}
