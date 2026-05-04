use byteview::ByteView;
use derive_more::Deref;

use super::Txid;

/// First-8-bytes prefix of a txid, packed as a `u64`. Both `From<&Txid>`
/// (via `from_le_bytes`) and `From<ByteView>` (via `from_be_bytes`,
/// inverse of the `to_be_bytes` writer) are host-independent so on-disk
/// keys are portable across architectures.
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
        Self(u64::from_le_bytes(
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
