use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::super::activity;
use super::Vecs;
use crate::{distribution, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        prices: &prices::Vecs,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;
        let circulating_supply = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .sats
            .height;

        self.vaulted.sats.height.compute_multiply(
            starting_height,
            circulating_supply,
            &activity.vaultedness.height,
            exit,
        )?;

        self.active.sats.height.compute_multiply(
            starting_height,
            circulating_supply,
            &activity.liveliness.height,
            exit,
        )?;

        self.vaulted.compute(prices, starting_height, exit)?;
        self.active.compute(prices, starting_height, exit)?;

        Ok(())
    }
}
