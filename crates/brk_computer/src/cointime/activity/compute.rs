use brk_error::Result;
use brk_types::{Bitcoin, CheckedSub, Indexes, StoredF64};
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, distribution};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
        blocks: &blocks::Vecs,
        distribution: &distribution::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = blocks.lookback.window_starts();

        let all_metrics = &distribution.utxo_cohorts.all.metrics;
        let circulating_supply = &all_metrics.supply.total.sats.height;

        self.coinblocks_created
            .compute(starting_indexes.height, &window_starts, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    circulating_supply,
                    |(i, v, ..)| (i, StoredF64::from(Bitcoin::from(v))),
                    exit,
                )?;
                Ok(())
            })?;

        self.coinblocks_stored
            .compute(starting_indexes.height, &window_starts, exit, |vec| {
                vec.compute_subtract(
                    starting_indexes.height,
                    &self.coinblocks_created.height,
                    &all_metrics.activity.coinblocks_destroyed.raw.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.liveliness.height.compute_divide(
            starting_indexes.height,
            &all_metrics.activity.coinblocks_destroyed.cumulative.height,
            &self.coinblocks_created.cumulative.height,
            exit,
        )?;

        self.vaultedness.height.compute_transform(
            starting_indexes.height,
            &self.liveliness.height,
            |(i, v, ..)| (i, StoredF64::from(1.0).checked_sub(v).unwrap()),
            exit,
        )?;

        self.activity_to_vaultedness_ratio.height.compute_divide(
            starting_indexes.height,
            &self.liveliness.height,
            &self.vaultedness.height,
            exit,
        )?;

        Ok(())
    }
}
