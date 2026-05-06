use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Bitcoin, StoredF64};
use vecdb::Exit;

use super::Vecs;
use crate::distribution;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        distribution: &distribution::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;
        let all_metrics = &distribution.utxo_cohorts.all.metrics;
        let circulating_supply = &all_metrics.supply.total.sats.height;

        self.coinblocks_created
            .compute(starting_height, exit, |vec| {
                vec.compute_transform(
                    starting_height,
                    circulating_supply,
                    |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                    exit,
                )?;
                Ok(())
            })?;

        self.coinblocks_stored
            .compute(starting_height, exit, |vec| {
                vec.compute_subtract(
                    starting_height,
                    &self.coinblocks_created.block,
                    &distribution.coinblocks_destroyed.block,
                    exit,
                )?;
                Ok(())
            })?;

        self.liveliness.height.compute_divide(
            starting_height,
            &distribution.coinblocks_destroyed.cumulative.height,
            &self.coinblocks_created.cumulative.height,
            exit,
        )?;

        self.ratio.height.compute_divide(
            starting_height,
            &self.liveliness.height,
            &self.vaultedness.height,
            exit,
        )?;

        Ok(())
    }
}
