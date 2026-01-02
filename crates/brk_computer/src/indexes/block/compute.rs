use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{DateIndex, DifficultyEpoch, HalvingEpoch};
use vecdb::{Exit, TypedVecIterator};

use super::Vecs;
use crate::blocks;

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        blocks_time: &blocks::time::Vecs,
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

        let decremented_starting_height = starting_indexes.height.decremented().unwrap_or_default();

        // DateIndex (uses blocks_time.height_to_date_fixed computed in blocks::time::compute_early)
        let starting_dateindex = self
            .height_to_dateindex
            .into_iter()
            .get(decremented_starting_height)
            .unwrap_or_default();

        self.height_to_dateindex.compute_transform(
            starting_indexes.height,
            &blocks_time.height_to_date_fixed,
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
            &blocks_time.height_to_date,
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
