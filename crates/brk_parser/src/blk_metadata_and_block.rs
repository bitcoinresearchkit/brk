use bitcoin::Block;

use crate::BlkMetadata;

#[derive(Debug)]
pub struct BlkIndexAndBlock {
    pub blk_metadata: BlkMetadata,
    pub block: Block,
}

impl BlkIndexAndBlock {
    pub fn new(blk_metadata: BlkMetadata, block: Block) -> Self {
        Self { blk_metadata, block }
    }
}
