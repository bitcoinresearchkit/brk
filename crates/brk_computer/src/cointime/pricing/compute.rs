use brk_error::Result;
use brk_types::Cents;
use vecdb::Exit;

use super::super::{activity, cap, supply};
use super::Vecs;
use crate::{ComputeIndexes, blocks, distribution, prices};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        supply: &supply::Vecs,
        cap: &cap::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let circulating_supply = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .btc
            .height;
        let realized_price = &distribution
            .utxo_cohorts
            .all
            .metrics
            .realized
            .realized_price
            .cents
            .height;

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

        self.active_price.cents.height.compute_transform2(
            starting_indexes.height,
            realized_price,
            &activity.liveliness.height,
            |(i, price, liveliness, ..)| {
                (i, Cents::from(f64::from(price) * f64::from(liveliness)))
            },
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
            &cap.investor_cap.height,
            &supply.active_supply.btc.height,
            |(i, cap_dollars, supply_btc, ..)| {
                (i, Cents::from(f64::from(Cents::from(cap_dollars)) / f64::from(supply_btc)))
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
            &cap.cointime_cap.height,
            circulating_supply,
            |(i, cap_dollars, supply_btc, ..)| {
                (i, Cents::from(f64::from(Cents::from(cap_dollars)) / f64::from(supply_btc)))
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
