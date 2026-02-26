use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, indexes, ComputeIndexes};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.height.compute_with_skip(
            starting_indexes.height,
            &indexes.txindex.input_count,
            &indexer.vecs.transactions.first_txindex,
            &indexes.height.txindex_count,
            exit,
            0,
        )?;

        let window_starts = blocks.count.window_starts();
        self.rolling.compute(
            starting_indexes.height,
            &window_starts,
            self.height.sum_cumulative.sum.inner(),
            exit,
        )?;

        Ok(())
    }
}
