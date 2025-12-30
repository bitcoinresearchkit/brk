use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Timestamp;
use vecdb::{Exit, TypedVecIterator};

use super::Vecs;
use crate::{Indexes, indexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.timeindexes_to_timestamp
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.dateindex,
                    &indexes.time.dateindex_to_date,
                    |(di, d, ..)| (di, Timestamp::from(d)),
                    exit,
                )?;
                Ok(())
            })?;

        let mut height_to_timestamp_iter = indexer.vecs.block.height_to_timestamp.iter()?;

        self.difficultyepoch_to_timestamp.compute_transform(
            starting_indexes.difficultyepoch,
            &indexes.block.difficultyepoch_to_first_height,
            |(i, h, ..)| (i, height_to_timestamp_iter.get_unwrap(h)),
            exit,
        )?;

        self.halvingepoch_to_timestamp.compute_transform(
            starting_indexes.halvingepoch,
            &indexes.block.halvingepoch_to_first_height,
            |(i, h, ..)| (i, height_to_timestamp_iter.get_unwrap(h)),
            exit,
        )?;

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

        Ok(())
    }
}
