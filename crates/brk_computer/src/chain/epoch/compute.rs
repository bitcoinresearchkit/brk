use brk_error::Result;
use brk_types::StoredU32;
use vecdb::{Exit, TypedVecIterator};

use super::Vecs;
use crate::{chain::TARGET_BLOCKS_PER_DAY_F32, indexes, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let mut height_to_difficultyepoch_iter =
            indexes.block.height_to_difficultyepoch.into_iter();
        self.indexes_to_difficultyepoch
            .compute_all(starting_indexes, exit, |vec| {
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

        let mut height_to_halvingepoch_iter = indexes.block.height_to_halvingepoch.into_iter();
        self.indexes_to_halvingepoch
            .compute_all(starting_indexes, exit, |vec| {
                let mut height_count_iter = indexes.time.dateindex_to_height_count.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.time.dateindex_to_first_height,
                    |(di, height, ..)| {
                        (
                            di,
                            height_to_halvingepoch_iter
                                .get_unwrap(height + (*height_count_iter.get_unwrap(di) - 1)),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        // Countdown metrics (moved from mining)
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
                    self.indexes_to_blocks_before_next_difficulty_adjustment
                        .height
                        .as_ref()
                        .unwrap(),
                    |(h, blocks, ..)| (h, (*blocks as f32 / TARGET_BLOCKS_PER_DAY_F32).into()),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_blocks_before_next_halving.compute_all(
            indexes,
            starting_indexes,
            exit,
            |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &indexes.block.height_to_height,
                    |(h, ..)| (h, StoredU32::from(h.left_before_next_halving())),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_days_before_next_halving.compute_all(
            indexes,
            starting_indexes,
            exit,
            |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_blocks_before_next_halving
                        .height
                        .as_ref()
                        .unwrap(),
                    |(h, blocks, ..)| (h, (*blocks as f32 / TARGET_BLOCKS_PER_DAY_F32).into()),
                    exit,
                )?;
                Ok(())
            },
        )?;

        Ok(())
    }
}
