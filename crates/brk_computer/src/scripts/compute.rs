use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::Exit;

use crate::prices;

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        self.count.compute(indexer, starting_indexes, exit)?;

        self.value
            .compute(indexer, prices, starting_indexes, exit)?;

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact_deferred_default()
        });
        Ok(())
    }
}
