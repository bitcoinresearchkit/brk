use brk_error::Result;
use brk_types::{Cents, Indexes};
use vecdb::Exit;

use super::super::{activity, cap, supply};
use super::Vecs;
use crate::{blocks, distribution, prices};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        supply: &supply::Vecs,
        cap: &cap::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let all_metrics = &distribution.utxo_cohorts.all.metrics;
        let circulating_supply = &all_metrics.supply.total.btc.height;
        let realized_price = &all_metrics.realized.realized_price.cents.height;

        self.vaulted_price.cents.height.compute_transform2(
            starting_indexes.height,
            realized_price,
            &activity.vaultedness.height,
            |(i, price, vaultedness, ..)| {
                (i, Cents::from(f64::from(price) / f64::from(vaultedness)))
            },
            exit,
        )?;

        self.vaulted_price_ratio.compute_rest(
            blocks,
            prices,
            starting_indexes,
            exit,
            &self.vaulted_price.cents.height,
        )?;

        self.active_price.cents.height.compute_multiply(
            starting_indexes.height,
            realized_price,
            &activity.liveliness.height,
            exit,
        )?;

        self.active_price_ratio.compute_rest(
            blocks,
            prices,
            starting_indexes,
            exit,
            &self.active_price.cents.height,
        )?;

        self.true_market_mean.cents.height.compute_transform2(
            starting_indexes.height,
            &cap.investor_cap.cents.height,
            &supply.active_supply.btc.height,
            |(i, cap_cents, supply_btc, ..)| {
                (i, Cents::from(f64::from(cap_cents) / f64::from(supply_btc)))
            },
            exit,
        )?;

        self.true_market_mean_ratio.compute_rest(
            blocks,
            prices,
            starting_indexes,
            exit,
            &self.true_market_mean.cents.height,
        )?;

        // cointime_price = cointime_cap / circulating_supply
        self.cointime_price.cents.height.compute_transform2(
            starting_indexes.height,
            &cap.cointime_cap.cents.height,
            circulating_supply,
            |(i, cap_cents, supply_btc, ..)| {
                (i, Cents::from(f64::from(cap_cents) / f64::from(supply_btc)))
            },
            exit,
        )?;

        self.cointime_price_ratio.compute_rest(
            blocks,
            prices,
            starting_indexes,
            exit,
            &self.cointime_price.cents.height,
        )?;

        Ok(())
    }
}
