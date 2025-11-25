use byteview::ByteView;
use derive_deref::Deref;

use super::Txid;

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        Self(u64::from_ne_bytes(value.as_slice().try_into().unwrap()))
    }
}

impl From<ByteView> for TxidPrefix {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self(u64::from_be_bytes((&*value).try_into().unwrap()))
    }
}

impl From<TxidPrefix> for ByteView {
    #[inline]
    fn from(value: TxidPrefix) -> Self {
        Self::from(&value)
    }
}

impl From<&TxidPrefix> for ByteView {
    #[inline]
    fn from(value: &TxidPrefix) -> Self {
        Self::from(value.to_be_bytes())
    }
}

impl From<[u8; 8]> for TxidPrefix {
    #[inline]
    fn from(value: [u8; 8]) -> Self {
        Self(u64::from_ne_bytes(value))
    }
}
