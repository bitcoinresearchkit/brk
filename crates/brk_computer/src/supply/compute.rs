use brk_error::Result;
use brk_types::{Indexes, Sats};
use vecdb::Exit;

/// Initial block subsidy (50 BTC) in sats, as f64 for floating-point comparisons.
const INITIAL_SUBSIDY: f64 = Sats::ONE_BTC_U64 as f64 * 50.0;

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
            prices,
            starting_indexes,
            exit,
        )?;

        // 2. Compute inflation rate: (supply[h] / supply[1y_ago]) - 1
        // Skip when lookback supply <= first block (50 BTC = 5B sats),
        // i.e. the lookback points to block 0 or 1 in the genesis era.
        let circulating_supply = &distribution.utxo_cohorts.all.metrics.supply.total.sats;
        self.inflation_rate
            .bps
            .height
            .compute_rolling_from_window_starts(
                starting_indexes.height,
                &blocks.lookback._1y,
                &circulating_supply.height,
                exit,
                |current, previous| {
                    if previous.is_nan() || previous <= INITIAL_SUBSIDY {
                        f64::NAN
                    } else {
                        current / previous - 1.0
                    }
                },
            )?;

        // 3. Compute velocity at height level
        self.velocity
            .compute(blocks, transactions, distribution, starting_indexes, exit)?;

        // 4. market_cap_rate - realized_cap_rate per window
        let all_realized = &distribution.utxo_cohorts.all.metrics.realized;
        let mcr_arr = self.market_cap_delta.rate.as_array();
        let diff_arr = self.market_minus_realized_cap_growth_rate.0.as_mut_array();

        let rcr_rates = [
            &all_realized.cap.delta.rate._24h.bps.height,
            &all_realized.cap.delta.rate._1w.bps.height,
            &all_realized.cap.delta.rate._1m.bps.height,
            &all_realized.cap.delta.rate._1y.bps.height,
        ];

        for i in 0..4 {
            diff_arr[i].height.compute_subtract(
                starting_indexes.height,
                &mcr_arr[i].bps.height,
                rcr_rates[i],
                exit,
            )?;
        }

        let _lock = exit.lock();
        self.db.compact()?;

        Ok(())
    }
}
