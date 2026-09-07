use brk_error::{OptionData, Result};
use brk_types::{BlockHash, BlockStatus, Height};
use vecdb::ReadableVec;

use crate::Query;

impl Query {
    pub fn block_status(&self, hash: &BlockHash) -> Result<BlockStatus> {
        let _guard = self.indexer().pin_safe_lengths();
        let height = self.height_by_hash(hash)?;
        self.block_status_at_height(height)
    }

    fn block_status_at_height(&self, height: Height) -> Result<BlockStatus> {
        let tip = self.height();
        let next_best = if height < tip {
            Some(
                self.indexer()
                    .vecs()
                    .blocks
                    .blockhash
                    .inner
                    .collect_one(height.incremented())
                    .data()?,
            )
        } else {
            None
        };

        Ok(BlockStatus::in_best_chain(height, next_best))
    }
}
