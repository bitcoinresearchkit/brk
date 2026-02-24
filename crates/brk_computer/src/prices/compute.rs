use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, indexes};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.cents
            .compute(indexer, indexes, starting_indexes, exit)?;

        let _lock = exit.lock();
        self.db().compact()?;
        Ok(())
    }
}
