use std::borrow::Cow;

use derive_more::Deref;

use crate::BlkMetadata;

use super::{BlockHash, Height};

#[derive(Debug, Deref)]
pub struct Block {
    height: Height,
    hash: BlockHash,
    #[deref]
    block: bitcoin::Block,
}

impl Block {
    pub fn height(&self) -> Height {
        self.height
    }

    pub fn hash(&self) -> &BlockHash {
        &self.hash
    }

    pub fn coinbase_tag(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(
            self.txdata
                .first()
                .and_then(|tx| tx.input.first())
                .unwrap()
                .script_sig
                .as_bytes(),
        )
    }
}

impl From<(Height, bitcoin::Block)> for Block {
    #[inline]
    fn from((height, block): (Height, bitcoin::Block)) -> Self {
        Self::from((height, block.block_hash(), block))
    }
}

impl From<(Height, bitcoin::BlockHash, bitcoin::Block)> for Block {
    #[inline]
    fn from((height, hash, block): (Height, bitcoin::BlockHash, bitcoin::Block)) -> Self {
        Self::from((height, BlockHash::from(hash), block))
    }
}

impl From<(Height, BlockHash, bitcoin::Block)> for Block {
    #[inline]
    fn from((height, hash, block): (Height, BlockHash, bitcoin::Block)) -> Self {
        Self {
            height,
            hash,
            block,
        }
    }
}

impl From<ReadBlock> for Block {
    #[inline]
    fn from(value: ReadBlock) -> Self {
        value.block
    }
}

#[derive(Debug, Deref)]
pub struct ReadBlock {
    #[deref]
    block: Block,
    metadata: BlkMetadata,
    tx_metadata: Vec<BlkMetadata>,
}

impl From<(Block, BlkMetadata, Vec<BlkMetadata>)> for ReadBlock {
    #[inline]
    fn from((block, metadata, tx_metadata): (Block, BlkMetadata, Vec<BlkMetadata>)) -> Self {
        Self {
            block,
            metadata,
            tx_metadata,
        }
    }
}

impl ReadBlock {
    pub fn metadata(&self) -> &BlkMetadata {
        &self.metadata
    }

    pub fn tx_metadata(&self) -> &Vec<BlkMetadata> {
        &self.tx_metadata
    }

    pub fn inner(self) -> Block {
        self.block
    }
}

