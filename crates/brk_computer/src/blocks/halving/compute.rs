use brk_error::Result;
use brk_types::StoredU32;
use vecdb::Exit;

use super::super::TARGET_BLOCKS_PER_DAY_F32;
use super::Vecs;
use crate::{ComputeIndexes, indexes};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.epoch.height.compute_transform(
            starting_indexes.height,
            &indexes.height.halvingepoch,
            |(h, epoch, ..)| (h, epoch),
            exit,
        )?;

        self.blocks_before_next_halving.height.compute_transform(
            starting_indexes.height,
            &indexes.height.identity,
            |(h, ..)| (h, StoredU32::from(h.left_before_next_halving())),
            exit,
        )?;

        self.days_before_next_halving.height.compute_transform(
            starting_indexes.height,
            &self.blocks_before_next_halving.height,
            |(h, blocks, ..)| (h, (*blocks as f32 / TARGET_BLOCKS_PER_DAY_F32).into()),
            exit,
        )?;

        Ok(())
    }
}
