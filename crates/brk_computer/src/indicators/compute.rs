use brk_error::Result;
use brk_types::{Bitcoin, Dollars, Indexes, StoredF32};
use vecdb::Exit;

use super::{gini, Vecs};
use crate::{distribution, internal::RatioDollarsBp32, market, mining, transactions};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        mining: &mining::Vecs,
        distribution: &distribution::Vecs,
        transactions: &transactions::Vecs,
        market: &market::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        // Puell Multiple: daily_subsidy_usd / sma_365d_subsidy_usd
        self.puell_multiple
            .bps
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                &mining.rewards.subsidy.block.usd,
                &mining.rewards.subsidy.average._1y.usd.height,
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
                &transactions.volume.transfer_volume.sum._24h.usd.height,
                exit,
            )?;

        // Thermocap Multiple: market_cap / thermo_cap
        self.thermo_cap_multiple
            .bps
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                market_cap,
                &mining.rewards.subsidy.cumulative.usd.height,
                exit,
            )?;

        let all_metrics = &distribution.utxo_cohorts.all.metrics;
        let all_activity = &all_metrics.activity;
        let supply_total_sats = &all_metrics.supply.total.sats.height;

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

        // Supply-Adjusted CYD = CYD / circulating_supply_btc
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
        self.dormancy.supply_adjusted
            .height
            .compute_transform2(
                starting_indexes.height,
                &all_activity.dormancy._24h.height,
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

        // Stock-to-Flow: supply / annual_issuance
        // annual_issuance ≈ subsidy_per_block × 52560 (blocks/year)
        self.stock_to_flow.height.compute_transform2(
            starting_indexes.height,
            supply_total_sats,
            &mining.rewards.subsidy.block.sats,
            |(i, supply_sats, subsidy_sats, ..)| {
                let annual_flow = subsidy_sats.as_u128() as f64 * 52560.0;
                if annual_flow == 0.0 {
                    (i, StoredF32::from(0.0f32))
                } else {
                    (i, StoredF32::from(
                        (supply_sats.as_u128() as f64 / annual_flow) as f32,
                    ))
                }
            },
            exit,
        )?;

        // Dormancy Flow: supply_btc / dormancy
        self.dormancy.flow.height.compute_transform2(
            starting_indexes.height,
            supply_total_sats,
            &all_activity.dormancy._24h.height,
            |(i, supply_sats, dormancy, ..)| {
                let d = f64::from(dormancy);
                if d == 0.0 {
                    (i, StoredF32::from(0.0f32))
                } else {
                    let supply = f64::from(Bitcoin::from(supply_sats));
                    (i, StoredF32::from((supply / d) as f32))
                }
            },
            exit,
        )?;

        // Seller Exhaustion Constant: % supply_in_profit × 30d_volatility
        self.seller_exhaustion
            .height
            .compute_transform3(
                starting_indexes.height,
                &all_metrics.supply.in_profit.sats.height,
                &market.volatility._1m.height,
                supply_total_sats,
                |(i, profit_sats, volatility, total_sats, ..)| {
                    let total = total_sats.as_u128() as f64;
                    if total == 0.0 {
                        (i, StoredF32::from(0.0f32))
                    } else {
                        let pct_in_profit = profit_sats.as_u128() as f64 / total;
                        (i, StoredF32::from(
                            (pct_in_profit * f64::from(volatility)) as f32,
                        ))
                    }
                },
                exit,
            )?;

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact_deferred_default()
        });
        Ok(())
    }
}
