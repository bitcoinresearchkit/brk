use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use crate::{indexes, ComputeIndexes};

use super::Vecs;

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.count
            .compute(indexer, indexes, starting_indexes, exit)?;

        self.value
            .compute(indexer, indexes, starting_indexes, exit)?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
