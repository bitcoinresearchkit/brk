use brk_error::Result;
use brk_types::Indexes;
use vecdb::Exit;

use crate::{blocks, indexes, prices};

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        indexes: &indexes::Vecs,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        // Phase 1: Independent sub-modules in parallel
        let (r1, r2) = rayon::join(
            || {
                rayon::join(
                    || self.ath.compute(prices, indexes, starting_indexes, exit),
                    || self.lookback.compute(blocks, prices, starting_indexes, exit),
                )
            },
            || {
                rayon::join(
                    || self.range.compute(prices, blocks, starting_indexes, exit),
                    || {
                        self.moving_average
                            .compute(blocks, prices, starting_indexes, exit)
                    },
                )
            },
        );
        r1.0?;
        r1.1?;
        r2.0?;
        r2.1?;

        // Phase 2: Depend on lookback
        self.returns
            .compute(prices, blocks, &self.lookback, starting_indexes, exit)?;

        // Phase 3: Depends on returns, moving_average
        self.technical.compute(
            &self.returns,
            prices,
            blocks,
            &self.moving_average,
            starting_indexes,
            exit,
        )?;

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact()
        });
        Ok(())
    }
}
