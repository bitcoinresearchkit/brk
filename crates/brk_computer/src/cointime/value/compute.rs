use brk_error::Result;
use vecdb::Exit;

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

        Ok(())
    }
}
