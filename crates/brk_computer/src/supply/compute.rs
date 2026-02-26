use brk_error::Result;
use brk_types::StoredF32;
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, blocks, distribution, mining, prices, scripts, transactions};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        scripts: &scripts::Vecs,
        blocks: &blocks::Vecs,
        mining: &mining::Vecs,
        transactions: &transactions::Vecs,
        prices: &prices::Vecs,
        distribution: &distribution::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // 1. Compute burned/unspendable supply
        self.burned
            .compute(scripts, mining, &blocks.count, prices, starting_indexes, exit)?;

        // 2. Compute inflation rate at height level: (supply[h] - supply[1y_ago]) / supply[1y_ago] * 100
        let circulating_supply = &distribution.utxo_cohorts.all.metrics.supply.total.sats;
        self.inflation.height.compute_rolling_percentage_change(
            starting_indexes.height,
            &blocks.count.height_1y_ago,
            &circulating_supply.height,
            exit,
        )?;

        // 3. Compute velocity at height level
        self.velocity
            .compute(blocks, transactions, distribution, starting_indexes, exit)?;

        // 4. Compute cap growth rates at height level using 1y lookback
        self.market_cap_growth_rate
            .height
            .compute_rolling_percentage_change(
                starting_indexes.height,
                &blocks.count.height_1y_ago,
                &self.market_cap.height,
                exit,
            )?;

        self.realized_cap_growth_rate
            .height
            .compute_rolling_percentage_change(
                starting_indexes.height,
                &blocks.count.height_1y_ago,
                &distribution.utxo_cohorts.all.metrics.realized.realized_cap.height,
                exit,
            )?;

        // 5. Compute cap growth rate diff: market_cap_growth_rate - realized_cap_growth_rate
        self.cap_growth_rate_diff.height.compute_transform2(
            starting_indexes.height,
            &self.market_cap_growth_rate.height,
            &self.realized_cap_growth_rate.height,
            |(h, a, b, ..)| (h, StoredF32::from(*a - *b)),
            exit,
        )?;

        let _lock = exit.lock();
        self.db.compact()?;

        Ok(())
    }
}
