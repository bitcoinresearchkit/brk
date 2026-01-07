use brk_error::Result;
use brk_types::StoredU32;
use vecdb::{Exit, TypedVecIterator};

use super::Vecs;
use super::super::TARGET_BLOCKS_PER_DAY_F32;
use crate::{indexes, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let mut height_to_difficultyepoch_iter =
            indexes.block.height_to_difficultyepoch.into_iter();
        self.indexes_to_difficultyepoch.compute_all(starting_indexes, exit, |vec| {
                let mut height_count_iter = indexes.time.dateindex_to_height_count.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.time.dateindex_to_first_height,
                    |(di, height, ..)| {
                        (
                            di,
                            height_to_difficultyepoch_iter
                                .get_unwrap(height + (*height_count_iter.get_unwrap(di) - 1)),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_blocks_before_next_difficulty_adjustment
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &indexes.block.height_to_height,
                    |(h, ..)| (h, StoredU32::from(h.left_before_next_diff_adj())),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_days_before_next_difficulty_adjustment
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.indexes_to_blocks_before_next_difficulty_adjustment.height,
                    |(h, blocks, ..)| (h, (*blocks as f32 / TARGET_BLOCKS_PER_DAY_F32).into()),
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
