use brk_error::Result;
use vecdb::Exit;

use super::super::{activity, cap, supply};
use super::Vecs;
use crate::{ComputeIndexes, distribution, indexes, price, utils::OptionExt};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        price: &price::Vecs,
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
            .supply
            .bitcoin
            .height;
        let realized_price = &distribution
            .utxo_cohorts
            .all
            .metrics
            .realized
            .u()
            .realized_price
            .height;

        self.vaulted_price
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    realized_price,
                    &activity.vaultedness.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.vaulted_price_ratio.compute_rest(
            price,
            starting_indexes,
            exit,
            Some(&self.vaulted_price.dateindex.0),
        )?;

        self.active_price
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    realized_price,
                    &activity.liveliness.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.active_price_ratio.compute_rest(
            price,
            starting_indexes,
            exit,
            Some(&self.active_price.dateindex.0),
        )?;

        self.true_market_mean
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    &cap.investor_cap.height,
                    &supply.active_supply.bitcoin.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.true_market_mean_ratio.compute_rest(
            price,
            starting_indexes,
            exit,
            Some(&self.true_market_mean.dateindex.0),
        )?;

        // cointime_price = cointime_cap / circulating_supply
        self.cointime_price
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    &cap.cointime_cap.height,
                    circulating_supply,
                    exit,
                )?;
                Ok(())
            })?;

        self.cointime_price_ratio.compute_rest(
            price,
            starting_indexes,
            exit,
            Some(&self.cointime_price.dateindex.0),
        )?;

        Ok(())
    }
}
