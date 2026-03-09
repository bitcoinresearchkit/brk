use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Indexes, StoredU32};
use vecdb::Exit;

use super::Vecs;

use crate::blocks::lookback;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        lookback: &lookback::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // Block count height + cumulative
        self.block_count.height.compute_range(
            starting_indexes.height,
            &indexer.vecs.blocks.weight,
            |h| (h, StoredU32::from(1_u32)),
            exit,
        )?;
        self.block_count.cumulative.height.compute_cumulative(
            starting_indexes.height,
            &self.block_count.height,
            exit,
        )?;

        // Rolling window block counts
        let ws = lookback.window_starts();
        self.block_count.sum.compute_rolling_sum(
            starting_indexes.height,
            &ws,
            &self.block_count.height,
            exit,
        )?;

        Ok(())
    }
}
