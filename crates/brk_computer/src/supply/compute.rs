use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, blocks, distribution, indexes, scripts, transactions};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        scripts: &scripts::Vecs,
        blocks: &blocks::Vecs,
        transactions: &transactions::Vecs,
        distribution: &distribution::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // 1. Compute burned/unspendable supply
        self.burned
            .compute(indexes, scripts, blocks, starting_indexes, exit)?;

        // 2. Compute inflation rate: daily_subsidy / circulating_supply * 365 * 100
        let circulating_supply = &distribution.utxo_cohorts.all.metrics.supply.total.sats;
        self.inflation.compute_all(starting_indexes, exit, |v| {
            v.compute_transform2(
                starting_indexes.dateindex,
                &blocks.rewards.subsidy.sats.dateindex.sum_cum.sum.0,
                &circulating_supply.dateindex.0,
                |(i, subsidy_1d_sum, supply, ..)| {
                    let inflation = if *supply > 0 {
                        365.0 * *subsidy_1d_sum as f64 / *supply as f64 * 100.0
                    } else {
                        0.0
                    };
                    (i, inflation.into())
                },
                exit,
            )?;
            Ok(())
        })?;

        // 3. Compute velocity
        self.velocity
            .compute(transactions, distribution, starting_indexes, exit)?;

        // 4. Compute cap growth rates
        if let Some(market_cap) = self.market_cap.as_ref() {
            let mcap_dateindex = &market_cap.dateindex.0;
            self.market_cap_growth_rate
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage_change(
                        starting_indexes.dateindex,
                        mcap_dateindex,
                        30,
                        exit,
                    )?;
                    Ok(())
                })?;
        }

        if let Some(realized) = distribution.utxo_cohorts.all.metrics.realized.as_ref() {
            let rcap_dateindex = &realized.realized_cap.dateindex.0;
            self.realized_cap_growth_rate
                .compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage_change(
                        starting_indexes.dateindex,
                        rcap_dateindex,
                        30,
                        exit,
                    )?;
                    Ok(())
                })?;
        }

        // Note: circulating, market_cap, cap_growth_rate_diff are lazy

        let _lock = exit.lock();
        self.db.compact()?;

        Ok(())
    }
}
