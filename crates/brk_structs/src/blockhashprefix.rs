use byteview::ByteView;
use derive_deref::Deref;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::copy_first_8bytes;

use super::BlockHash;

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
pub struct BlockHashPrefix([u8; 8]);

impl From<BlockHash> for BlockHashPrefix {
    fn from(value: BlockHash) -> Self {
        Self::from(&value)
    }
}

impl From<&BlockHash> for BlockHashPrefix {
    fn from(value: &BlockHash) -> Self {
        Self(copy_first_8bytes(&value[..]).unwrap())
    }
}

impl From<ByteView> for BlockHashPrefix {
    fn from(value: ByteView) -> Self {
        Self::read_from_bytes(&value).unwrap()
    }
}

impl From<&BlockHashPrefix> for ByteView {
    fn from(value: &BlockHashPrefix) -> Self {
        Self::new(value.as_bytes())
    }
}

impl From<BlockHashPrefix> for ByteView {
    fn from(value: BlockHashPrefix) -> Self {
        Self::from(&value)
    }
}
