use brk_error::Result;
use brk_types::Indexes;
use vecdb::Exit;

use super::super::activity;
use super::Vecs;
use crate::{distribution, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
        prices: &prices::Vecs,
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

        self.vaulted
            .compute(prices, starting_indexes.height, exit)?;
        self.active.compute(prices, starting_indexes.height, exit)?;

        Ok(())
    }
}
