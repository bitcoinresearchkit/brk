use byteview::ByteView;
use derive_deref::Deref;
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
