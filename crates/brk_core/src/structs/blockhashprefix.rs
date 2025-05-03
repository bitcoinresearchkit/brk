use byteview::ByteView;
use derive_deref::Deref;
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{Error, copy_first_8bytes};

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

impl TryFrom<ByteView> for BlockHashPrefix {
    type Error = Error;
    fn try_from(value: ByteView) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
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
