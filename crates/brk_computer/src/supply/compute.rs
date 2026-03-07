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
        self.burned.compute(
            scripts,
            mining,
            &blocks.count,
            prices,
            starting_indexes,
            exit,
        )?;

        // 2. Compute inflation rate: (supply[h] / supply[1y_ago]) - 1
        let circulating_supply = &distribution.utxo_cohorts.all.metrics.supply.total.sats;
        self.inflation_rate
            .bps
            .height
            .compute_rolling_ratio_change(
                starting_indexes.height,
                &blocks.count.height_1y_ago,
                &circulating_supply.height,
                exit,
            )?;

        // 3. Compute velocity at height level
        self.velocity
            .compute(blocks, transactions, distribution, starting_indexes, exit)?;

        // 4. Compute cap growth rates across 4 windows
        let window_starts = blocks.count.window_starts();

        let realized_cap = &distribution
            .utxo_cohorts
            .all
            .metrics
            .realized
            .realized_cap
            .height;

        let mcgr_arr = self.market_cap_growth_rate.0.as_mut_array();
        let rcgr_arr = self.realized_cap_growth_rate.0.as_mut_array();
        let diff_arr = self.market_minus_realized_cap_growth_rate.0.as_mut_array();
        let starts_arr = window_starts.as_array();

        for i in 0..4 {
            mcgr_arr[i].bps.height.compute_rolling_ratio_change(
                starting_indexes.height,
                *starts_arr[i],
                &self.market_cap.height,
                exit,
            )?;
            rcgr_arr[i].bps.height.compute_rolling_ratio_change(
                starting_indexes.height,
                *starts_arr[i],
                realized_cap,
                exit,
            )?;
            diff_arr[i].height.compute_subtract(
                starting_indexes.height,
                &mcgr_arr[i].bps.height,
                &rcgr_arr[i].bps.height,
                exit,
            )?;
        }

        let _lock = exit.lock();
        self.db.compact()?;

        Ok(())
    }
}
