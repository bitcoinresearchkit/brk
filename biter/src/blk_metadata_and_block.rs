use bitcoin::Block;

use crate::BlkMetadata;

pub struct BlkMetadataAndBlock {
    pub blk_metadata: BlkMetadata,
    pub block: Block,
}

impl BlkMetadataAndBlock {
    pub fn new(blk_metadata: BlkMetadata, block: Block) -> Self {
        Self {
            blk_metadata,
            block,
        }
    }
}
