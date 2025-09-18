use std::{borrow::Cow, ops::Deref};

use crate::BlkPosition;

use super::{BlockHash, Height};

#[derive(Debug)]
pub struct Block {
    height: Height,
    hash: BlockHash,
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
    fn from((height, block): (Height, bitcoin::Block)) -> Self {
        Self::from((height, block.block_hash(), block))
    }
}

impl From<(Height, bitcoin::BlockHash, bitcoin::Block)> for Block {
    fn from((height, hash, block): (Height, bitcoin::BlockHash, bitcoin::Block)) -> Self {
        Self {
            height,
            hash: hash.into(),
            block,
        }
    }
}

impl Deref for Block {
    type Target = bitcoin::Block;
    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

#[derive(Debug)]
pub struct ParsedBlock {
    block: Block,
    position: BlkPosition,
    tx_positions: Vec<BlkPosition>,
}

impl From<(Block, BlkPosition, Vec<BlkPosition>)> for ParsedBlock {
    fn from((block, position, tx_positions): (Block, BlkPosition, Vec<BlkPosition>)) -> Self {
        Self {
            block,
            position,
            tx_positions,
        }
    }
}

impl ParsedBlock {
    pub fn position(&self) -> &BlkPosition {
        &self.position
    }

    pub fn tx_positions(&self) -> &Vec<BlkPosition> {
        &self.tx_positions
    }
}

impl Deref for ParsedBlock {
    type Target = Block;
    fn deref(&self) -> &Self::Target {
        &self.block
    }
}
