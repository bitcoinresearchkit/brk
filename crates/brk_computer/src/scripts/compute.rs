use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use crate::ComputeIndexes;

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.count
            .compute(indexer, starting_indexes, exit)?;

        self.value
            .compute(indexer, starting_indexes, exit)?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
