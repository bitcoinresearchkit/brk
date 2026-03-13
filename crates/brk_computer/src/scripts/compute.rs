use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::Exit;

use crate::{outputs, prices};

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        outputs: &outputs::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.count
            .compute(indexer, starting_indexes, exit)?;

        self.value
            .compute(indexer, prices, starting_indexes, exit)?;

        self.adoption
            .compute(&self.count, &outputs.count, starting_indexes, exit)?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
