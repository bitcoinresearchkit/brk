use byteview::ByteView;
use derive_deref::Deref;
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

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
)]
pub struct TxidPrefix([u8; 8]);

impl From<Txid> for TxidPrefix {
    fn from(value: Txid) -> Self {
        Self::from(&value)
    }
}

impl From<&Txid> for TxidPrefix {
    fn from(value: &Txid) -> Self {
        Self(copy_first_8bytes(&value[..]).unwrap())
    }
}

impl From<ByteView> for TxidPrefix {
    fn from(value: ByteView) -> Self {
        Self::read_from_bytes(&value).unwrap()
    }
}

impl From<&TxidPrefix> for ByteView {
    fn from(value: &TxidPrefix) -> Self {
        Self::new(value.as_bytes())
    }
}

impl From<TxidPrefix> for ByteView {
    fn from(value: TxidPrefix) -> Self {
        Self::from(&value)
    }
}

impl From<[u8; 8]> for TxidPrefix {
    fn from(value: [u8; 8]) -> Self {
        Self(value)
    }
}
