use std::path::PathBuf;

use bitcoin::{hashes::Hash, BlockHash};
use serde::{Deserialize, Serialize};

use crate::{path_to_modified_time, BlkMetadataAndBlock};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlkRecap {
    min_continuous_height: usize,
    min_continuous_prev_hash: BlockHash,
    modified_time: u64,
}

impl BlkRecap {
    pub fn first(blk_metadata_and_block: &BlkMetadataAndBlock) -> Self {
        Self {
            min_continuous_height: 0,
            min_continuous_prev_hash: BlockHash::all_zeros(),
            modified_time: blk_metadata_and_block.blk_metadata.modified_time,
        }
    }

    pub fn from(height: usize, blk_metadata_and_block: &BlkMetadataAndBlock) -> Self {
        Self {
            min_continuous_height: height,
            min_continuous_prev_hash: blk_metadata_and_block.block.header.prev_blockhash,
            modified_time: blk_metadata_and_block.blk_metadata.modified_time,
        }
    }

    pub fn has_different_modified_time(&self, blk_path: &PathBuf) -> bool {
        self.modified_time != path_to_modified_time(blk_path)
    }

    pub fn is_younger_than(&self, height: usize) -> bool {
        self.min_continuous_height > height
    }

    pub fn height(&self) -> usize {
        self.min_continuous_height
    }

    pub fn prev_hash(&self) -> &BlockHash {
        &self.min_continuous_prev_hash
    }
}
