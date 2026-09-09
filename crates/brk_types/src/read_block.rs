use derive_more::Deref;

use crate::{BlkMetadata, Block};

#[derive(Debug, Deref)]
pub struct ReadBlock {
    #[deref]
    block: Block,
    metadata: BlkMetadata,
    tx_metadata: Vec<BlkMetadata>,
}

impl From<ReadBlock> for Block {
    #[inline]
    fn from(value: ReadBlock) -> Self {
        value.block
    }
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
}
