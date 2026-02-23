use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::ComputeIndexes;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.size
            .compute_cumulative(starting_indexes, &indexer.vecs.blocks.total_size, exit)?;

        self.vbytes.compute_cumulative(starting_indexes, exit)?;

        Ok(())
    }
}
