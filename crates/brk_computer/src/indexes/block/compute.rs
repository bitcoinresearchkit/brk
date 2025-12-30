use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{Date, DateIndex, DifficultyEpoch, HalvingEpoch};
use vecdb::{Exit, TypedVecIterator};

use super::Vecs;

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &brk_indexer::Indexes,
        exit: &Exit,
    ) -> Result<(DateIndex, DifficultyEpoch, HalvingEpoch)> {
        self.height_to_txindex_count.compute_count_from_indexes(
            starting_indexes.height,
            &indexer.vecs.tx.height_to_first_txindex,
            &indexer.vecs.tx.txindex_to_txid,
            exit,
        )?;

        self.height_to_height.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.block.height_to_weight,
            exit,
        )?;

        self.height_to_date.compute_transform(
            starting_indexes.height,
            &indexer.vecs.block.height_to_timestamp,
            |(h, t, ..)| (h, Date::from(t)),
            exit,
        )?;

        let mut prev_timestamp_fixed = None;
        self.height_to_timestamp_fixed.compute_transform(
            starting_indexes.height,
            &indexer.vecs.block.height_to_timestamp,
            |(h, timestamp, height_to_timestamp_fixed_iter)| {
                if prev_timestamp_fixed.is_none()
                    && let Some(prev_h) = h.decremented()
                {
                    prev_timestamp_fixed.replace(
                        height_to_timestamp_fixed_iter
                            .into_iter()
                            .get_unwrap(prev_h),
                    );
                }
                let timestamp_fixed =
                    prev_timestamp_fixed.map_or(timestamp, |prev_d| prev_d.max(timestamp));
                prev_timestamp_fixed.replace(timestamp_fixed);
                (h, timestamp_fixed)
            },
            exit,
        )?;

        self.height_to_date_fixed.compute_transform(
            starting_indexes.height,
            &self.height_to_timestamp_fixed,
            |(h, t, ..)| (h, Date::from(t)),
            exit,
        )?;

        let decremented_starting_height = starting_indexes.height.decremented().unwrap_or_default();

        // DateIndex (computed before time module needs it)
        let starting_dateindex = self
            .height_to_dateindex
            .into_iter()
            .get(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_dateindex.compute_transform(
            starting_indexes.height,
            &self.height_to_date_fixed,
            |(h, d, ..)| (h, DateIndex::try_from(d).unwrap()),
            exit,
        )?;

        let starting_dateindex = if let Some(dateindex) = self
            .height_to_dateindex
            .into_iter()
            .get(decremented_starting_height)
        {
            starting_dateindex.min(dateindex)
        } else {
            starting_dateindex
        };

        // Difficulty epoch
        let starting_difficultyepoch = self
            .height_to_difficultyepoch
            .into_iter()
            .get(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_difficultyepoch.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.block.height_to_weight,
            exit,
        )?;

        self.difficultyepoch_to_first_height.compute_coarser(
            starting_indexes.height,
            &self.height_to_difficultyepoch,
            exit,
        )?;

        self.difficultyepoch_to_difficultyepoch.compute_from_index(
            starting_difficultyepoch,
            &self.difficultyepoch_to_first_height,
            exit,
        )?;

        self.difficultyepoch_to_height_count.compute_count_from_indexes(
            starting_difficultyepoch,
            &self.difficultyepoch_to_first_height,
            &self.height_to_date,
            exit,
        )?;

        // Halving epoch
        let starting_halvingepoch = self
            .height_to_halvingepoch
            .into_iter()
            .get(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_halvingepoch.compute_from_index(
            starting_indexes.height,
            &indexer.vecs.block.height_to_weight,
            exit,
        )?;

        self.halvingepoch_to_first_height.compute_coarser(
            starting_indexes.height,
            &self.height_to_halvingepoch,
            exit,
        )?;

        self.halvingepoch_to_halvingepoch.compute_from_index(
            starting_halvingepoch,
            &self.halvingepoch_to_first_height,
            exit,
        )?;

        Ok((starting_dateindex, starting_difficultyepoch, starting_halvingepoch))
    }
}
