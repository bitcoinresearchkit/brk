use byteview::ByteView;
use derive_deref::Deref;

use super::BlockHash;

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockHashPrefix(u64);

impl From<BlockHash> for BlockHashPrefix {
    #[inline]
    fn from(value: BlockHash) -> Self {
        Self::from(&value)
    }
}

impl From<&BlockHash> for BlockHashPrefix {
    #[inline]
    fn from(value: &BlockHash) -> Self {
        Self(u64::from_ne_bytes(
            value.as_slice()[0..8].try_into().unwrap(),
        ))
    }
}

impl From<ByteView> for BlockHashPrefix {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self(u64::from_be_bytes((&*value).try_into().unwrap()))
    }
}

impl From<BlockHashPrefix> for ByteView {
    #[inline]
    fn from(value: BlockHashPrefix) -> Self {
        Self::from(&value)
    }
}

impl From<&BlockHashPrefix> for ByteView {
    #[inline]
    fn from(value: &BlockHashPrefix) -> Self {
        Self::from(value.to_be_bytes())
    }
}
