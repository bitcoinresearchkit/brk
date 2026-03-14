use std::thread;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::Exit;

use crate::indexes;

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // Sequential: time → lookback (dependency chain)
        self.time
            .timestamp
            .compute(indexer, indexes, starting_indexes, exit)?;
        self.lookback
            .compute(&self.time, starting_indexes, exit)?;

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
            let r1 = s.spawn(|| count.compute(indexer, starting_indexes, exit));
            let r2 = s.spawn(|| interval.compute(indexer, starting_indexes, exit));
            let r3 = s.spawn(|| weight.compute(indexer, starting_indexes, exit));
            let r4 =
                s.spawn(|| difficulty.compute(indexer, indexes, starting_indexes, exit));
            let r5 = s.spawn(|| halving.compute(indexes, starting_indexes, exit));
            size.compute(indexer, &*lookback, starting_indexes, exit)?;
            r1.join().unwrap()?;
            r2.join().unwrap()?;
            r3.join().unwrap()?;
            r4.join().unwrap()?;
            r5.join().unwrap()?;
            Ok(())
        })?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
