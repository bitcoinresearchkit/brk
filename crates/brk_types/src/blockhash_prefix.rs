use byteview::ByteView;
use derive_more::Deref;

use super::BlockHash;

/// First-8-bytes prefix of a block hash, packed as a `u64`. Both
/// `From<&BlockHash>` (via `from_le_bytes`) and `From<ByteView>` (via
/// `from_be_bytes`, inverse of the `to_be_bytes` writer) are
/// host-independent so on-disk keys are portable across architectures.
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
        Self(u64::from_le_bytes(
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

impl From<u64> for BlockHashPrefix {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
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
