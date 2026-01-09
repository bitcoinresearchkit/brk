use brk_error::Result;
use brk_types::StoredU32;
use vecdb::{Exit, TypedVecIterator};

use super::super::TARGET_BLOCKS_PER_DAY_F32;
use super::Vecs;
use crate::{ComputeIndexes, indexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let mut height_to_halvingepoch_iter = indexes.height.halvingepoch.into_iter();
        self.halvingepoch
            .compute_all(starting_indexes, exit, |vec| {
                let mut height_count_iter = indexes.dateindex.height_count.into_iter();
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.dateindex.first_height,
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

        self.blocks_before_next_halving
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &indexes.height.identity,
                    |(h, ..)| (h, StoredU32::from(h.left_before_next_halving())),
                    exit,
                )?;
                Ok(())
            })?;

        self.days_before_next_halving
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.blocks_before_next_halving.height,
                    |(h, blocks, ..)| (h, (*blocks as f32 / TARGET_BLOCKS_PER_DAY_F32).into()),
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
