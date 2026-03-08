use brk_error::Result;
use brk_types::Indexes;
use vecdb::Exit;

use crate::{blocks, distribution, indexes, mining, prices, transactions};

use super::Vecs;

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        mining: &mining::Vecs,
        distribution: &distribution::Vecs,
        transactions: &transactions::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // Phase 1: Independent sub-modules in parallel
        let (r1, r2) = rayon::join(
            || {
                rayon::join(
                    || self.ath.compute(prices, blocks, starting_indexes, exit),
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
        let (r3, r4) = rayon::join(
            || {
                self.returns
                    .compute(prices, blocks, &self.lookback, starting_indexes, exit)
            },
            || {
                self.dca.compute(
                    indexes,
                    prices,
                    blocks,
                    &self.lookback,
                    starting_indexes,
                    exit,
                )
            },
        );
        r3?;
        r4?;

        // Phase 3: Depends on returns, range, moving_average
        self.indicators.compute(
            &mining.rewards,
            &self.returns,
            &self.range,
            prices,
            blocks,
            distribution,
            transactions,
            &self.moving_average,
            starting_indexes,
            exit,
        )?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
