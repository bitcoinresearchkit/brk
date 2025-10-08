use std::{fmt, mem};

use bitcoin::hashes::Hash;
use bitcoincore_rpc::{Client, RpcApi};
use derive_deref::Deref;
use serde::{Serialize, Serializer};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use super::Height;

#[derive(Debug, Deref, Clone, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes)]
pub struct BlockHash([u8; 32]);

impl From<bitcoin::BlockHash> for BlockHash {
    fn from(value: bitcoin::BlockHash) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<BlockHash> for bitcoin::BlockHash {
    fn from(value: BlockHash) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<&BlockHash> for bitcoin::BlockHash {
    fn from(value: &BlockHash) -> Self {
        bitcoin::BlockHash::from_slice(&value.0).unwrap()
    }
}

impl TryFrom<(&Client, Height)> for BlockHash {
    type Error = bitcoincore_rpc::Error;
    fn try_from((rpc, height): (&Client, Height)) -> Result<Self, Self::Error> {
        Ok(Self::from(rpc.get_block_hash(u64::from(height))?))
    }
}

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&bitcoin::BlockHash::from(self).to_string())
    }
}

impl Serialize for BlockHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
