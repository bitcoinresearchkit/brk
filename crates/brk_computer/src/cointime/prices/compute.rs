use brk_error::Result;
use brk_types::{Cents, Indexes};
use vecdb::Exit;

use super::super::{activity, cap, supply};
use super::Vecs;
use crate::{distribution, prices};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
        prices: &prices::Vecs,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        supply: &supply::Vecs,
        cap: &cap::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let all_metrics = &distribution.utxo_cohorts.all.metrics;
        let circulating_supply = &all_metrics.supply.total.btc.height;
        let realized_price = &all_metrics.realized.price.cents.height;

        self.vaulted
            .compute_all(prices, starting_indexes, exit, |v| {
                Ok(v.compute_transform2(
                    starting_indexes.height,
                    realized_price,
                    &activity.vaultedness.height,
                    |(i, price, vaultedness, ..)| {
                        (i, Cents::from(f64::from(price) / f64::from(vaultedness)))
                    },
                    exit,
                )?)
            })?;

        self.active
            .compute_all(prices, starting_indexes, exit, |v| {
                Ok(v.compute_transform2(
                    starting_indexes.height,
                    realized_price,
                    &activity.liveliness.height,
                    |(i, price, liveliness, ..)| {
                        (i, Cents::from(f64::from(price) / f64::from(liveliness)))
                    },
                    exit,
                )?)
            })?;

        self.true_market_mean
            .compute_all(prices, starting_indexes, exit, |v| {
                Ok(v.compute_transform2(
                    starting_indexes.height,
                    &cap.investor.cents.height,
                    &supply.active.btc.height,
                    |(i, cap_cents, supply_btc, ..)| {
                        (i, Cents::from(f64::from(cap_cents) / f64::from(supply_btc)))
                    },
                    exit,
                )?)
            })?;

        self.cointime
            .compute_all(prices, starting_indexes, exit, |v| {
                Ok(v.compute_transform2(
                    starting_indexes.height,
                    &cap.cointime.cents.height,
                    circulating_supply,
                    |(i, cap_cents, supply_btc, ..)| {
                        (i, Cents::from(f64::from(cap_cents) / f64::from(supply_btc)))
                    },
                    exit,
                )?)
            })?;

        Ok(())
    }
}
