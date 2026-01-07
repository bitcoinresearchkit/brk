use brk_error::Result;
use vecdb::Exit;

use super::super::activity;
use super::Vecs;
use crate::{distribution, indexes, price, ComputeIndexes};

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
        let indexes_to_coinblocks_destroyed = &distribution
            .utxo_cohorts
            .all
            .metrics
            .activity
            .indexes_to_coinblocks_destroyed;

        self.indexes_to_cointime_value_destroyed.compute_all(
            indexes,
            starting_indexes,
            exit,
            |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    &price.usd.chainindexes_to_price_close.height,
                    &indexes_to_coinblocks_destroyed.height,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_cointime_value_created.compute_all(
            indexes,
            starting_indexes,
            exit,
            |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    &price.usd.chainindexes_to_price_close.height,
                    &activity.indexes_to_coinblocks_created.height,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_cointime_value_stored.compute_all(
            indexes,
            starting_indexes,
            exit,
            |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    &price.usd.chainindexes_to_price_close.height,
                    &activity.indexes_to_coinblocks_stored.height,
                    exit,
                )?;
                Ok(())
            },
        )?;

        Ok(())
    }
}
