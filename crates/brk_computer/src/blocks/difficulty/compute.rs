use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{StoredF32, StoredU32};
use vecdb::{Exit, TypedVecIterator};

use super::super::TARGET_BLOCKS_PER_DAY_F32;
use super::Vecs;
use crate::{ComputeIndexes, indexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Derive dateindex/period stats from raw difficulty
        self.raw.derive_from(
            indexes,
            starting_indexes,
            &indexer.vecs.blocks.difficulty,
            exit,
        )?;

        // Compute difficulty as hash rate equivalent
        self.as_hash
            .compute_all(indexes, starting_indexes, exit, |v| {
                let multiplier = 2.0_f64.powi(32) / 600.0;
                v.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs.blocks.difficulty,
                    |(i, v, ..)| (i, StoredF32::from(*v * multiplier)),
                    exit,
                )?;
                Ok(())
            })?;

        // Compute difficulty adjustment percentage
        self.adjustment
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_percentage_change(
                    starting_indexes.height,
                    &indexer.vecs.blocks.difficulty,
                    1,
                    exit,
                )?;
                Ok(())
            })?;

        // Compute epoch by dateindex
        let mut height_to_difficultyepoch_iter = indexes.height.difficultyepoch.into_iter();
        self.epoch.compute_all(starting_indexes, exit, |vec| {
            let mut height_count_iter = indexes.dateindex.height_count.into_iter();
            vec.compute_transform(
                starting_indexes.dateindex,
                &indexes.dateindex.first_height,
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

        // Compute blocks before next adjustment
        self.blocks_before_next_adjustment.compute_all(
            indexes,
            starting_indexes,
            exit,
            |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &indexes.height.identity,
                    |(h, ..)| (h, StoredU32::from(h.left_before_next_diff_adj())),
                    exit,
                )?;
                Ok(())
            },
        )?;

        // Compute days before next adjustment
        self.days_before_next_adjustment.compute_all(
            indexes,
            starting_indexes,
            exit,
            |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.blocks_before_next_adjustment.height,
                    |(h, blocks, ..)| (h, (*blocks as f32 / TARGET_BLOCKS_PER_DAY_F32).into()),
                    exit,
                )?;
                Ok(())
            },
        )?;

        Ok(())
    }
}
