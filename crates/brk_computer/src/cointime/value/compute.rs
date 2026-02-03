use brk_error::Result;
use brk_types::{Bitcoin, Close, Dollars, StoredF64};
use vecdb::{Exit, TypedVecIterator};

use super::super::activity;
use super::Vecs;
use crate::{ComputeIndexes, distribution, indexes, price};

impl Vecs {
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        price: &price::Vecs,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        exit: &Exit,
    ) -> Result<()> {
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
            .bitcoin
            .height;

        self.cointime_value_destroyed
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    &price.usd.split.close.height,
                    &coinblocks_destroyed.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.cointime_value_created
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    &price.usd.split.close.height,
                    &activity.coinblocks_created.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.cointime_value_stored
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    &price.usd.split.close.height,
                    &activity.coinblocks_stored.height,
                    exit,
                )?;
                Ok(())
            })?;

        // VOCDD: Value of Coin Days Destroyed = price × (CDD / circulating_supply)
        // Supply-adjusted to account for growing supply over time
        // This is a key input for Reserve Risk / HODL Bank calculation
        self.vocdd
            .compute_all(indexes, starting_indexes, exit, |vec| {
                let mut supply_iter = circulating_supply.into_iter();
                vec.compute_transform2(
                    starting_indexes.height,
                    &price.usd.split.close.height,
                    &coindays_destroyed.height,
                    |(i, price, cdd, _): (_, Close<Dollars>, StoredF64, _)| {
                        let supply: Bitcoin = supply_iter.get_unwrap(i);
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
