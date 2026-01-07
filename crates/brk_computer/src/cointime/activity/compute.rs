use brk_error::Result;
use brk_types::{Bitcoin, CheckedSub, StoredF64};
use vecdb::{Exit, TypedVecIterator};

use super::Vecs;
use crate::{distribution, indexes, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        distribution: &distribution::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let circulating_supply = &distribution.utxo_cohorts.all.metrics.supply.height_to_supply;

        self.indexes_to_coinblocks_created
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    circulating_supply,
                    |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                    exit,
                )?;
                Ok(())
            })?;

        let indexes_to_coinblocks_destroyed = &distribution
            .utxo_cohorts
            .all
            .metrics
            .activity
            .indexes_to_coinblocks_destroyed;

        self.indexes_to_coinblocks_stored
            .compute_all(indexes, starting_indexes, exit, |vec| {
                let mut coinblocks_destroyed_iter = indexes_to_coinblocks_destroyed
                    .height
                    .into_iter();
                vec.compute_transform(
                    starting_indexes.height,
                    &self.indexes_to_coinblocks_created.height,
                    |(i, created, ..)| {
                        let destroyed = coinblocks_destroyed_iter.get_unwrap(i);
                        (i, created.checked_sub(destroyed).unwrap())
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_liveliness
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    indexes_to_coinblocks_destroyed.height_cumulative.inner(),
                    self.indexes_to_coinblocks_created.height_cumulative.inner(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_vaultedness
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    &self.indexes_to_liveliness.height,
                    |(i, v, ..)| (i, StoredF64::from(1.0).checked_sub(v).unwrap()),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_activity_to_vaultedness_ratio.compute_all(
            indexes,
            starting_indexes,
            exit,
            |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    &self.indexes_to_liveliness.height,
                    &self.indexes_to_vaultedness.height,
                    exit,
                )?;
                Ok(())
            },
        )?;

        Ok(())
    }
}
