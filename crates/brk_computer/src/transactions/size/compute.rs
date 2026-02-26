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
        count_vecs: &blocks::CountVecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let block_windows = count_vecs.block_window_starts();

        self.weight
            .derive_from(indexer, indexes, starting_indexes, &block_windows, exit)?;

        self.vsize
            .derive_from(indexer, indexes, starting_indexes, &block_windows, exit)?;

        Ok(())
    }
}
