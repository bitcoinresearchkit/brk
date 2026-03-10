use brk_error::Result;
use brk_types::{Bitcoin, Dollars, Indexes, StoredF32};
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

        // Thermocap Multiple: market_cap / thermo_cap
        // thermo_cap = cumulative subsidy in USD
        self.thermocap_multiple
            .bps
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                market_cap,
                &mining.rewards.subsidy.cumulative.usd.height,
                exit,
            )?;

        let all_activity = &distribution.utxo_cohorts.all.metrics.activity;
        let supply_total_sats = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .sats
            .height;

        // Supply-Adjusted CDD = sum_24h(CDD) / circulating_supply_btc
        self.coindays_destroyed_supply_adjusted
            .height
            .compute_transform2(
                starting_indexes.height,
                &all_activity.coindays_destroyed.sum._24h.height,
                supply_total_sats,
                |(i, cdd_24h, supply_sats, ..)| {
                    let supply = f64::from(Bitcoin::from(supply_sats));
                    if supply == 0.0 {
                        (i, StoredF32::from(0.0f32))
                    } else {
                        (i, StoredF32::from((f64::from(cdd_24h) / supply) as f32))
                    }
                },
                exit,
            )?;

        // Supply-Adjusted CYD = CYD / circulating_supply_btc (CYD = 1y rolling sum of CDD)
        self.coinyears_destroyed_supply_adjusted
            .height
            .compute_transform2(
                starting_indexes.height,
                &all_activity.coinyears_destroyed.height,
                supply_total_sats,
                |(i, cyd, supply_sats, ..)| {
                    let supply = f64::from(Bitcoin::from(supply_sats));
                    if supply == 0.0 {
                        (i, StoredF32::from(0.0f32))
                    } else {
                        (i, StoredF32::from((f64::from(cyd) / supply) as f32))
                    }
                },
                exit,
            )?;

        // Supply-Adjusted Dormancy = dormancy / circulating_supply_btc
        self.dormancy_supply_adjusted
            .height
            .compute_transform2(
                starting_indexes.height,
                &all_activity.dormancy.height,
                supply_total_sats,
                |(i, dormancy, supply_sats, ..)| {
                    let supply = f64::from(Bitcoin::from(supply_sats));
                    if supply == 0.0 {
                        (i, StoredF32::from(0.0f32))
                    } else {
                        (i, StoredF32::from((f64::from(dormancy) / supply) as f32))
                    }
                },
                exit,
            )?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
