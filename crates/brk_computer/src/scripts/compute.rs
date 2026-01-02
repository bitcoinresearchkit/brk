use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use crate::{indexes, price, ComputeIndexes};

use super::Vecs;

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.count
            .compute(indexer, indexes, starting_indexes, exit)?;

        self.value
            .compute(indexer, indexes, price, starting_indexes, exit)?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
