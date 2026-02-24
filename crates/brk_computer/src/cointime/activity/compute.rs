use brk_error::Result;
use brk_types::{Bitcoin, CheckedSub, StoredF64};
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, blocks, distribution};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        blocks: &blocks::Vecs,
        distribution: &distribution::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = blocks.count.window_starts();

        let circulating_supply = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .sats
            .height;

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

        let coinblocks_destroyed = &distribution
            .utxo_cohorts
            .all
            .metrics
            .activity
            .coinblocks_destroyed;

        self.coinblocks_stored
            .compute(starting_indexes.height, &window_starts, exit, |vec| {
                vec.compute_transform2(
                    starting_indexes.height,
                    &self.coinblocks_created.height,
                    &coinblocks_destroyed.height,
                    |(i, created, destroyed, ..)| (i, created.checked_sub(destroyed).unwrap()),
                    exit,
                )?;
                Ok(())
            })?;

        self.liveliness.height.compute_divide(
            starting_indexes.height,
            &coinblocks_destroyed.cumulative.height,
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
