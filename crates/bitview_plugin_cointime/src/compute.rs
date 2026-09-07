use brk_error::Result;

use bitview_plugin::{ComputePlugin, UpdateContext};

use super::Vecs;
use crate::Dependencies;

impl ComputePlugin for Vecs {
    type Dependencies<'a> = Dependencies<'a>;
    type Output = ();

    fn compute(
        &mut self,
        dependencies: Self::Dependencies<'_>,
        context: UpdateContext<'_>,
    ) -> Result<Self::Output> {
        let Dependencies {
            indexer,
            price: prices,
            blocks,
            inflation_rate,
            velocity_native,
            velocity_fiat,
            distribution,
        } = dependencies;
        let inflation_rate = &inflation_rate.ppm.height;
        let velocity_native = &velocity_native.height;
        let velocity_fiat = &velocity_fiat.height;
        let exit = context.exit();

        self.db.sync_bg_tasks()?;

        // Activity computes first (liveliness, vaultedness, etc.)
        super::activity::compute(&mut self.activity, indexer, distribution, exit)?;
        super::age_range::compute(&mut self.age_range, indexer, distribution, exit)?;

        // Age-range supply is lazy over the same cached inputs as aggregates.
        // Adjusted and value compute independently.
        let (r1, r2) = rayon::join(
            || {
                super::aggregate::compute(
                    &mut self.aggregate,
                    indexer,
                    distribution,
                    &mut self.age_range,
                    &mut self.supply.active_supply_in_loss_share.bounded,
                    exit,
                )
            },
            || {
                rayon::join(
                    || {
                        super::adjusted::compute(
                            &mut self.adjusted,
                            indexer,
                            inflation_rate,
                            velocity_native,
                            velocity_fiat,
                            &self.activity,
                            exit,
                        )
                    },
                    || {
                        super::value::compute(
                            &mut self.value,
                            indexer,
                            prices,
                            distribution,
                            &self.activity,
                            exit,
                        )
                    },
                )
            },
        );
        r1?;
        r2.0?;
        r2.1?;

        // Cap depends on activity + value
        super::cap::compute(
            &mut self.cap,
            indexer,
            distribution,
            &self.activity,
            &self.value,
            exit,
        )?;

        // Phase 4: pricing and reserve_risk are independent
        let (r3, r4) = rayon::join(
            || {
                super::prices::compute(
                    &mut self.prices,
                    indexer,
                    distribution,
                    &self.activity,
                    &self.supply,
                    &self.cap,
                    exit,
                )
            },
            || {
                super::reserve_risk::compute(
                    &mut self.reserve_risk,
                    indexer,
                    blocks,
                    prices,
                    &self.value,
                    exit,
                )
            },
        );
        r3?;
        r4?;

        context.compact_database(&self.db);

        Ok(())
    }
}
