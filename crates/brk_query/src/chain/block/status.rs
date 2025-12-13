use brk_error::Result;
use brk_types::{BlockStatus, Height};
use vecdb::{AnyVec, GenericStoredVec};

use crate::Query;

/// Get block status by height
pub fn get_block_status_by_height(height: Height, query: &Query) -> Result<BlockStatus> {
    let indexer = query.indexer();

    let max_height = Height::from(
        indexer
            .vecs
            .block
            .height_to_blockhash
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
                .block
                .height_to_blockhash
                .read_once(height.incremented())?,
        )
    } else {
        None
    };

    Ok(BlockStatus::in_best_chain(height, next_best))
}
