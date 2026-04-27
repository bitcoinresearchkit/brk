use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, indexes, inputs, prices};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        inputs: &inputs::Vecs,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        self.count
            .compute(indexer, indexes, blocks, starting_indexes, exit)?;
        self.per_sec.compute(&self.count, starting_indexes, exit)?;
        self.value
            .compute(indexer, prices, starting_indexes, exit)?;
        self.by_type.compute(indexer, starting_indexes, exit)?;
        self.unspent.compute(
            &self.count,
            &inputs.count,
            &self.by_type,
            starting_indexes,
            exit,
        )?;
        let lock = self
            .spent
            .compute(indexer, inputs, starting_indexes, exit)?;
        self.db.run_bg(move |db| {
            let _lock = lock;
            db.compact_deferred_default()
        });
        Ok(())
    }
}
