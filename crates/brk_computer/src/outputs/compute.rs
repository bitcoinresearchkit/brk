use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, indexes, inputs, price};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        inputs: &inputs::Vecs,
        blocks: &blocks::Vecs,
        prices: &price::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        let starting_lengths = indexer.safe_lengths();

        self.count.compute(indexer, indexes, blocks, exit)?;
        self.per_sec.compute(&self.count, &starting_lengths, exit)?;
        self.value.compute(indexer, prices, exit)?;
        self.by_type.compute(indexer, exit)?;
        self.unspent.compute(
            &self.count,
            &inputs.count,
            &self.by_type,
            &starting_lengths,
            exit,
        )?;
        let lock = self.spent.compute(indexer, inputs, exit)?;
        self.db.run_bg(move |db| {
            let _lock = lock;
            db.compact_deferred_default()
        });
        Ok(())
    }
}
