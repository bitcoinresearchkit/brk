use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use crate::{blocks, outputs, prices, ComputeIndexes};

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        blocks: &blocks::Vecs,
        outputs: &outputs::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.count
            .compute(indexer, &blocks.count, starting_indexes, exit)?;

        self.value
            .compute(indexer, &blocks.count, prices, starting_indexes, exit)?;

        self.adoption
            .compute(&self.count, &outputs.count, starting_indexes, exit)?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
