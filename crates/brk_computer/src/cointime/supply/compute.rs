use brk_error::Result;
use brk_types::Indexes;
use vecdb::Exit;

use super::super::activity;
use super::Vecs;
use crate::distribution;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
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

        self.vaulted.sats.height.compute_multiply(
            starting_indexes.height,
            circulating_supply,
            &activity.vaultedness.height,
            exit,
        )?;

        self.active.sats.height.compute_multiply(
            starting_indexes.height,
            circulating_supply,
            &activity.liveliness.height,
            exit,
        )?;

        Ok(())
    }
}
