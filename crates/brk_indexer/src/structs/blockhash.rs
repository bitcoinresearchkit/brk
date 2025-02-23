use std::mem;

use brk_parser::{
    Height,
    rpc::{Client, RpcApi},
};
use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Deref, Clone, PartialEq, Eq, Immutable, IntoBytes, KnownLayout, FromBytes, Serialize)]
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

impl TryFrom<(&Client, Height)> for BlockHash {
    type Error = brk_parser::rpc::Error;
    fn try_from((rpc, height): (&Client, Height)) -> Result<Self, Self::Error> {
        Ok(Self::from(rpc.get_block_hash(u64::from(height))?))
    }
}
