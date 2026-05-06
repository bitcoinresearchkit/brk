use std::thread;

use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use crate::indexes;

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        // lookback depends on indexes.timestamp.monotonic
        self.lookback.compute(indexer, indexes, exit)?;

        // Parallel: remaining sub-modules are independent of each other.
        // size depends on lookback (already computed above).
        let Vecs {
            lookback,
            count,
            interval,
            size,
            weight,
            difficulty,
            halving,
            ..
        } = self;
        thread::scope(|s| -> Result<()> {
            let r1 = s.spawn(|| count.compute(indexer, exit));
            let r2 = s.spawn(|| interval.compute(indexer, exit));
            let r3 = s.spawn(|| weight.compute(indexer, exit));
            let r4 = s.spawn(|| difficulty.compute(indexer, indexes, exit));
            let r5 = s.spawn(|| halving.compute(indexer, indexes, exit));
            size.compute(indexer, &*lookback, exit)?;
            r1.join().unwrap()?;
            r2.join().unwrap()?;
            r3.join().unwrap()?;
            r4.join().unwrap()?;
            r5.join().unwrap()?;
            Ok(())
        })?;

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact_deferred_default()
        });
        Ok(())
    }
}
