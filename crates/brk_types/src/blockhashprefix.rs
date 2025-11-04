use std::{cmp::Ordering, mem};

use byteview::ByteView;
use derive_deref::Deref;
use redb::{Key, TypeName, Value};
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
    Hash,
)]
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
        Self(u64::from_ne_bytes(copy_first_8bytes(&value[..]).unwrap()))
    }
}

impl From<ByteView> for BlockHashPrefix {
    #[inline]
    fn from(value: ByteView) -> Self {
        Self::read_from_bytes(&value).unwrap()
    }
}

impl From<&BlockHashPrefix> for ByteView {
    #[inline]
    fn from(value: &BlockHashPrefix) -> Self {
        Self::new(value.as_bytes())
    }
}

impl From<BlockHashPrefix> for ByteView {
    #[inline]
    fn from(value: BlockHashPrefix) -> Self {
        Self::from(&value)
    }
}

impl Value for BlockHashPrefix {
    type SelfType<'a> = BlockHashPrefix;
    type AsBytes<'a>
        = [u8; mem::size_of::<u64>()]
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        Some(mem::size_of::<u64>())
    }

    fn from_bytes<'a>(data: &'a [u8]) -> BlockHashPrefix
    where
        Self: 'a,
    {
        BlockHashPrefix(u64::from_le_bytes(data.try_into().unwrap()))
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> [u8; mem::size_of::<u64>()]
    where
        Self: 'a,
        Self: 'b,
    {
        value.0.to_le_bytes()
    }

    fn type_name() -> TypeName {
        TypeName::new("BlockHashPrefix")
    }
}

impl Key for BlockHashPrefix {
    fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
        Self::from_bytes(data1).cmp(&Self::from_bytes(data2))
    }
}
