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
        price: Option<&price::Vecs>,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let circulating_supply = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .supply
            .sats
            .height;

        self.vaulted_supply
            .compute_all(indexes, price, starting_indexes, exit, |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    circulating_supply,
                    &activity.vaultedness.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.active_supply
            .compute_all(indexes, price, starting_indexes, exit, |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    circulating_supply,
                    &activity.liveliness.height,
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
