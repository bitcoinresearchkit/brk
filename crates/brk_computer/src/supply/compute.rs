use brk_error::Result;
use brk_types::Indexes;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, distribution, mining, prices, scripts, transactions};

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
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // 1. Compute burned/unspendable supply
        self.burned
            .compute(scripts, mining, &blocks.count, prices, starting_indexes, exit)?;

        // 2. Compute inflation rate: (supply[h] / supply[1y_ago]) - 1
        let circulating_supply = &distribution.utxo_cohorts.all.metrics.supply.total.sats;
        self.inflation_rate.bps.height.compute_rolling_ratio_change(
            starting_indexes.height,
            &blocks.count.height_1y_ago,
            &circulating_supply.height,
            exit,
        )?;

        // 3. Compute velocity at height level
        self.velocity
            .compute(blocks, transactions, distribution, starting_indexes, exit)?;

        // 4. Compute cap growth rates using 1y lookback
        self.market_cap_growth_rate
            .bps
            .height
            .compute_rolling_ratio_change(
                starting_indexes.height,
                &blocks.count.height_1y_ago,
                &self.market_cap.height,
                exit,
            )?;

        self.realized_cap_growth_rate
            .bps
            .height
            .compute_rolling_ratio_change(
                starting_indexes.height,
                &blocks.count.height_1y_ago,
                &distribution.utxo_cohorts.all.metrics.realized.realized_cap.height,
                exit,
            )?;

        // 5. Compute cap growth rate diff: market - realized
        self.market_minus_realized_cap_growth_rate.height.compute_subtract(
            starting_indexes.height,
            &self.market_cap_growth_rate.bps.height,
            &self.realized_cap_growth_rate.bps.height,
            exit,
        )?;

        let _lock = exit.lock();
        self.db.compact()?;

        Ok(())
    }
}
