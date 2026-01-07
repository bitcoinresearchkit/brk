use byteview::ByteView;
use derive_more::Deref;

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
        Self(u64::from_ne_bytes(
            value.as_slice()[0..8].try_into().unwrap(),
        ))
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
