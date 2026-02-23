use brk_error::Result;
use vecdb::Exit;

use super::super::activity;
use super::Vecs;
use crate::{ComputeIndexes, distribution};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let circulating_supply = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .sats
            .height;

        self.vaulted_supply.sats.height.compute_multiply(
            starting_indexes.height,
            circulating_supply,
            &activity.vaultedness.height,
            exit,
        )?;

        self.active_supply.sats.height.compute_multiply(
            starting_indexes.height,
            circulating_supply,
            &activity.liveliness.height,
            exit,
        )?;

        Ok(())
    }
}
