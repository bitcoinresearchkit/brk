use brk_error::Result;
use brk_types::{Cents, Indexes};
use vecdb::{Exit, VecIndex};

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
        let realized_cap = &all_metrics.realized.cap.cents.height;

        self.vaulted.compute_all(
            prices,
            starting_indexes,
            exit,
            |v| {
                Ok(v.compute_transform2(
                    starting_indexes.height,
                    realized_price,
                    &activity.vaultedness.height,
                    |(i, price, vaultedness, ..)| {
                        (i, Cents::from(f64::from(price) / f64::from(vaultedness)))
                    },
                    exit,
                )?)
            },
        )?;

        self.active.compute_all(
            prices,
            starting_indexes,
            exit,
            |v| {
                Ok(v.compute_multiply(
                    starting_indexes.height,
                    realized_price,
                    &activity.liveliness.height,
                    exit,
                )?)
            },
        )?;

        self.true_market_mean.compute_all(
            prices,
            starting_indexes,
            exit,
            |v| {
                Ok(v.compute_transform2(
                    starting_indexes.height,
                    &cap.investor.cents.height,
                    &supply.active.btc.height,
                    |(i, cap_cents, supply_btc, ..)| {
                        (i, Cents::from(f64::from(cap_cents) / f64::from(supply_btc)))
                    },
                    exit,
                )?)
            },
        )?;

        // cointime_price = cointime_cap / circulating_supply
        self.cointime.compute_all(
            prices,
            starting_indexes,
            exit,
            |v| {
                Ok(v.compute_transform2(
                    starting_indexes.height,
                    &cap.cointime.cents.height,
                    circulating_supply,
                    |(i, cap_cents, supply_btc, ..)| {
                        (i, Cents::from(f64::from(cap_cents) / f64::from(supply_btc)))
                    },
                    exit,
                )?)
            },
        )?;

        // transfer_price = cointime_price - vaulted_price
        self.transfer.cents.height.compute_transform2(
            starting_indexes.height,
            &self.cointime.cents.height,
            &self.vaulted.cents.height,
            |(i, cointime, vaulted, ..)| (i, cointime.saturating_sub(vaulted)),
            exit,
        )?;
        self.transfer.compute_rest(prices, starting_indexes, exit)?;

        // balanced_price = (realized_price + transfer_price) / 2
        self.balanced.cents.height.compute_transform2(
            starting_indexes.height,
            realized_price,
            &self.transfer.cents.height,
            |(i, realized, transfer, ..)| (i, (realized + transfer) / 2u64),
            exit,
        )?;
        self.balanced.compute_rest(prices, starting_indexes, exit)?;

        // terminal_price = 21M × transfer_price / circulating_supply_btc
        self.terminal.cents.height.compute_transform2(
            starting_indexes.height,
            &self.transfer.cents.height,
            circulating_supply,
            |(i, transfer, supply_btc, ..)| {
                let supply = f64::from(supply_btc);
                if supply == 0.0 {
                    (i, Cents::ZERO)
                } else {
                    (i, Cents::from(f64::from(transfer) * 21_000_000.0 / supply))
                }
            },
            exit,
        )?;
        self.terminal.compute_rest(prices, starting_indexes, exit)?;

        // cumulative_market_cap = Σ(market_cap) in dollars
        self.cumulative_market_cap
            .height
            .compute_cumulative(
                starting_indexes.height,
                &all_metrics.supply.total.cents.height,
                exit,
            )?;

        // delta_price = (realized_cap - average_cap) / circulating_supply
        // average_cap = cumulative_market_cap / (height + 1)
        self.delta.cents.height.compute_transform3(
            starting_indexes.height,
            realized_cap,
            &self.cumulative_market_cap.height,
            circulating_supply,
            |(i, realized_cap_cents, cum_mcap_dollars, supply_btc, ..)| {
                let supply = f64::from(supply_btc);
                if supply == 0.0 {
                    return (i, Cents::ZERO);
                }
                let avg_cap_cents = f64::from(cum_mcap_dollars) * 100.0 / (i.to_usize() + 1) as f64;
                let delta = (f64::from(realized_cap_cents) - avg_cap_cents) / supply;
                (i, Cents::from(delta.max(0.0)))
            },
            exit,
        )?;
        self.delta.compute_rest(prices, starting_indexes, exit)?;

        Ok(())
    }
}
