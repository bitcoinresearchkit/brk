use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use crate::{blocks, indexes, price};

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        prices: &price::Vecs,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        // Phase 1: Independent sub-modules in parallel
        let (r1, r2) = rayon::join(
            || {
                rayon::join(
                    || self.ath.compute(indexer, prices, indexes, exit),
                    || self.lookback.compute(indexer, blocks, prices, exit),
                )
            },
            || {
                rayon::join(
                    || self.range.compute(indexer, prices, blocks, exit),
                    || self.moving_average.compute(indexer, blocks, prices, exit),
                )
            },
        );
        r1.0?;
        r1.1?;
        r2.0?;
        r2.1?;

        // Phase 2: Depend on lookback
        self.returns
            .compute(indexer, prices, blocks, &self.lookback, exit)?;

        // Phase 3: Depends on returns, moving_average
        self.technical.compute(
            indexer,
            &self.returns,
            prices,
            blocks,
            &self.moving_average,
            exit,
        )?;

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact_deferred_default()
        });
        Ok(())
    }
}
