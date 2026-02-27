use brk_error::Result;
use brk_types::{Bitcoin, Dollars, StoredF64};
use vecdb::Exit;

use super::super::activity;
use super::Vecs;
use crate::{ComputeIndexes, blocks, distribution, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = blocks.count.window_starts();

        let coinblocks_destroyed = &distribution
            .utxo_cohorts
            .all
            .metrics
            .activity
            .coinblocks_destroyed;

        let coindays_destroyed = &distribution
            .utxo_cohorts
            .all
            .metrics
            .activity
            .coindays_destroyed;

        let circulating_supply = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .btc
            .height;

        self.cointime_value_destroyed
            .compute(starting_indexes.height, &window_starts, exit, |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    &prices.price.usd,
                    &coinblocks_destroyed.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.cointime_value_created
            .compute(starting_indexes.height, &window_starts, exit, |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    &prices.price.usd,
                    &activity.coinblocks_created.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.cointime_value_stored
            .compute(starting_indexes.height, &window_starts, exit, |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    &prices.price.usd,
                    &activity.coinblocks_stored.height,
                    exit,
                )?;
                Ok(())
            })?;

        // VOCDD: Value of Coin Days Destroyed = price × (CDD / circulating_supply)
        // Supply-adjusted to account for growing supply over time
        // This is a key input for Reserve Risk / HODL Bank calculation
        self.vocdd
            .compute(starting_indexes.height, &window_starts, exit, |vec| {
                vec.compute_transform3(
                    starting_indexes.height,
                    &prices.price.usd,
                    &coindays_destroyed.height,
                    circulating_supply,
                    |(i, price, cdd, supply, _): (_, Dollars, StoredF64, Bitcoin, _)| {
                        let supply_f64 = f64::from(supply);
                        if supply_f64 == 0.0 {
                            (i, StoredF64::from(0.0))
                        } else {
                            // VOCDD = price × (CDD / supply)
                            let vocdd = f64::from(price) * f64::from(cdd) / supply_f64;
                            (i, StoredF64::from(vocdd))
                        }
                    },
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
