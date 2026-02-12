use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{indexes, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.usd.compute(starting_indexes, &self.cents, exit)?;

        self.sats.compute(starting_indexes, &self.usd, exit)?;

        self.oracle
            .compute(indexer, indexes, starting_indexes, exit)?;

        let _lock = exit.lock();
        self.db().compact()?;
        Ok(())
    }
}
