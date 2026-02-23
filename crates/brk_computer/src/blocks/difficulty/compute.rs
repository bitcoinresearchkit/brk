use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{StoredF32, StoredU32};
use vecdb::Exit;

use super::super::TARGET_BLOCKS_PER_DAY_F32;
use super::Vecs;
use crate::{ComputeIndexes, indexes};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // raw is fully lazy from indexer height source — no compute needed

        // Compute difficulty as hash rate equivalent
        let multiplier = 2.0_f64.powi(32) / 600.0;
        self.as_hash.height.compute_transform(
            starting_indexes.height,
            &indexer.vecs.blocks.difficulty,
            |(i, v, ..)| (i, StoredF32::from(*v * multiplier)),
            exit,
        )?;

        // Compute difficulty adjustment percentage
        self.adjustment.height.compute_percentage_change(
            starting_indexes.height,
            &indexer.vecs.blocks.difficulty,
            1,
            exit,
        )?;

        // Compute epoch by height
        self.epoch.height.compute_transform(
            starting_indexes.height,
            &indexes.height.difficultyepoch,
            |(h, epoch, ..)| (h, epoch),
            exit,
        )?;

        // Compute blocks before next adjustment
        self.blocks_before_next_adjustment.height.compute_transform(
            starting_indexes.height,
            &indexes.height.identity,
            |(h, ..)| (h, StoredU32::from(h.left_before_next_diff_adj())),
            exit,
        )?;

        // Compute days before next adjustment
        self.days_before_next_adjustment.height.compute_transform(
            starting_indexes.height,
            &self.blocks_before_next_adjustment.height,
            |(h, blocks, ..)| (h, (*blocks as f32 / TARGET_BLOCKS_PER_DAY_F32).into()),
            exit,
        )?;

        Ok(())
    }
}
