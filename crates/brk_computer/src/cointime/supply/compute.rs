use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use super::super::activity;
use crate::{distribution, indexes, price, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        price: Option<&price::Vecs>,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let circulating_supply = &distribution.utxo_cohorts.all.metrics.supply.height_to_supply;

        self.indexes_to_vaulted_supply.compute_all(
            indexes,
            price,
            starting_indexes,
            exit,
            |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    circulating_supply,
                    &activity.indexes_to_vaultedness.height,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_active_supply.compute_all(
            indexes,
            price,
            starting_indexes,
            exit,
            |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    circulating_supply,
                    &activity.indexes_to_liveliness.height,
                    exit,
                )?;
                Ok(())
            },
        )?;

        Ok(())
    }
}
