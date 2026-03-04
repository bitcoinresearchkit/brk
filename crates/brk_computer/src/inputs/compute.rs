use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, indexes};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.spent
            .compute(&self.db, indexer, starting_indexes, exit)?;
        self.count
            .compute(indexer, indexes, blocks, starting_indexes, exit)?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
