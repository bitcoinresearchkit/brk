use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use crate::{ComputeIndexes, indexes};

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.interval.compute(indexer, starting_indexes, exit)?;
        self.count
            .compute(indexer, &self.time, starting_indexes, exit)?;
        self.size.compute(indexer, starting_indexes, exit)?;
        self.weight.compute(indexer, starting_indexes, exit)?;
        self.difficulty
            .compute(indexer, indexes, starting_indexes, exit)?;
        self.halving.compute(indexes, starting_indexes, exit)?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
