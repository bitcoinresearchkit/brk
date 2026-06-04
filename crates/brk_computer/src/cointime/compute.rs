use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, distribution, mining, price, supply};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        prices: &price::Vecs,
        blocks: &blocks::Vecs,
        mining: &mining::Vecs,
        supply_vecs: &supply::Vecs,
        distribution: &distribution::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        // Activity computes first (liveliness, vaultedness, etc.)
        self.activity.compute(indexer, distribution, exit)?;

        // Phase 2: supply, adjusted, value are independent (all depend only on activity)
        let (r1, r2) = rayon::join(
            || {
                self.supply
                    .compute(indexer, prices, distribution, &self.activity, exit)
            },
            || {
                rayon::join(
                    || {
                        self.adjusted
                            .compute(indexer, supply_vecs, &self.activity, exit)
                    },
                    || {
                        self.value
                            .compute(indexer, prices, distribution, &self.activity, exit)
                    },
                )
            },
        );
        r1?;
        r2.0?;
        r2.1?;

        // Cap depends on activity + value
        self.cap.compute(
            indexer,
            mining,
            distribution,
            &self.activity,
            &self.value,
            exit,
        )?;

        // Phase 4: pricing and reserve_risk are independent
        let (r3, r4) = rayon::join(
            || {
                self.prices.compute(
                    indexer,
                    prices,
                    distribution,
                    &self.activity,
                    &self.supply,
                    &self.cap,
                    exit,
                )
            },
            || {
                self.reserve_risk
                    .compute(indexer, blocks, prices, &self.value, exit)
            },
        );
        r3?;
        r4?;

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact_deferred_default()
        });
        Ok(())
    }
}
