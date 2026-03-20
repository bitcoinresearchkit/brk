use brk_error::Result;
use brk_types::{Bitcoin, Indexes, StoredF64};
use vecdb::Exit;

use super::Vecs;
use crate::distribution;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
        distribution: &distribution::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let all_metrics = &distribution.utxo_cohorts.all.metrics;
        let circulating_supply = &all_metrics.supply.total.sats.height;

        self.coinblocks_created
            .compute(starting_indexes.height, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    circulating_supply,
                    |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                    exit,
                )?;
                Ok(())
            })?;

        self.coinblocks_stored
            .compute(starting_indexes.height, exit, |vec| {
                vec.compute_subtract(
                    starting_indexes.height,
                    &self.coinblocks_created.block.height,
                    &distribution.coinblocks_destroyed.block.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.liveliness.height.compute_divide(
            starting_indexes.height,
            &distribution.coinblocks_destroyed.cumulative.height,
            &self.coinblocks_created.cumulative.height,
            exit,
        )?;

        self.ratio.height.compute_divide(
            starting_indexes.height,
            &self.liveliness.height,
            &self.vaultedness.height,
            exit,
        )?;

        Ok(())
    }
}
