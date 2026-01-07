use brk_error::Result;
use vecdb::Exit;

use super::super::{activity, cap, supply};
use super::Vecs;
use crate::{distribution, indexes, price, utils::OptionExt, ComputeIndexes};

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
        let circulating_supply = &distribution.utxo_cohorts.all.metrics.supply.height_to_supply_value.bitcoin;
        let realized_price = &distribution
            .utxo_cohorts
            .all
            .metrics
            .realized
            .u()
            .indexes_to_realized_price
            .height;

        self.indexes_to_vaulted_price
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    realized_price,
                    &activity.indexes_to_vaultedness.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_vaulted_price_ratio.compute_rest(
            price,
            starting_indexes,
            exit,
            Some(&self.indexes_to_vaulted_price.dateindex.0),
        )?;

        self.indexes_to_active_price
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    realized_price,
                    &activity.indexes_to_liveliness.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_active_price_ratio.compute_rest(
            price,
            starting_indexes,
            exit,
            Some(&self.indexes_to_active_price.dateindex.0),
        )?;

        self.indexes_to_true_market_mean.compute_all(
            indexes,
            starting_indexes,
            exit,
            |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    &cap.indexes_to_investor_cap.height,
                    &supply.indexes_to_active_supply.bitcoin.height,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_true_market_mean_ratio.compute_rest(
            price,
            starting_indexes,
            exit,
            Some(&self.indexes_to_true_market_mean.dateindex.0),
        )?;

        // cointime_price = cointime_cap / circulating_supply
        self.indexes_to_cointime_price
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    &cap.indexes_to_cointime_cap.height,
                    circulating_supply,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_cointime_price_ratio.compute_rest(
            price,
            starting_indexes,
            exit,
            Some(&self.indexes_to_cointime_price.dateindex.0),
        )?;

        Ok(())
    }
}
