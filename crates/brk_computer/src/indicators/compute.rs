use brk_error::Result;
use brk_types::{Dollars, Indexes};
use vecdb::Exit;

use super::{gini, Vecs};
use crate::{distribution, internal::RatioDollarsBp32, mining, transactions};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        mining: &mining::Vecs,
        distribution: &distribution::Vecs,
        transactions: &transactions::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // Puell Multiple: daily_subsidy_usd / sma_365d_subsidy_usd
        self.puell_multiple
            .bps
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                &mining.rewards.subsidy.base.usd.height,
                &mining.rewards.subsidy_sma_1y.usd.height,
                exit,
            )?;

        // Gini coefficient (UTXO distribution inequality)
        gini::compute(&mut self.gini, distribution, starting_indexes, exit)?;

        // RHODL Ratio: 1d-1w realized cap / 1y-2y realized cap
        self.rhodl_ratio
            .bps
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                &distribution
                    .utxo_cohorts
                    .age_range
                    ._1d_to_1w
                    .metrics
                    .realized
                    .cap
                    .usd
                    .height,
                &distribution
                    .utxo_cohorts
                    .age_range
                    ._1y_to_2y
                    .metrics
                    .realized
                    .cap
                    .usd
                    .height,
                exit,
            )?;

        // NVT: market_cap / tx_volume_24h
        let market_cap = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .usd
            .height;
        self.nvt
            .bps
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                market_cap,
                &transactions.volume.sent_sum.rolling._24h.usd.height,
                exit,
            )?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
