use brk_error::Result;
use brk_types::{BlockHash, BlockStatus, Height};
use vecdb::AnyVec;

use crate::Query;

impl Query {
    pub fn block_status(&self, hash: &BlockHash) -> Result<BlockStatus> {
        let height = self.height_by_hash(hash)?;
        self.block_status_by_height(height)
    }

    fn block_status_by_height(&self, height: Height) -> Result<BlockStatus> {
        let indexer = self.indexer();

        let max_height = Height::from(
            indexer
                .vecs
                .blocks
                .blockhash
                .len()
                .saturating_sub(1),
        );

        if height > max_height {
            return Ok(BlockStatus::not_in_best_chain());
        }

        let next_best = if height < max_height {
            Some(
                indexer
                    .vecs
                    .blocks
                    .blockhash
                    .read_once(height.incremented())?,
            )
        } else {
            None
        };

        Ok(BlockStatus::in_best_chain(height, next_best))
    }
}
