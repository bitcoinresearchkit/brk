use std::path::Path;

use brk_core::{Dateindex, Decadeindex, Difficultyepoch, Height, Monthindex, Weekindex, Yearindex};
use brk_exit::Exit;
use brk_vec::{AnyStorableVec, Compressed};

use crate::storage::vecs::{Indexes, base::StorableVec, indexes};

use super::{ComputedType, StorableVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct StorableVecsStatsFromHeight<T>
where
    T: ComputedType + PartialOrd,
{
    pub dateindex: StorableVecBuilder<Dateindex, T>,
    pub weekindex: StorableVecBuilder<Weekindex, T>,
    pub difficultyepoch: StorableVecBuilder<Difficultyepoch, T>,
    pub monthindex: StorableVecBuilder<Monthindex, T>,
    pub yearindex: StorableVecBuilder<Yearindex, T>,
    // pub halvingepoch: StorableVecGeneator<Halvingepoch, T>, // TODO
    pub decadeindex: StorableVecBuilder<Decadeindex, T>,
}

impl<T> StorableVecsStatsFromHeight<T>
where
    T: ComputedType + Ord + From<f64>,
    f64: From<T>,
{
    pub fn forced_import(
        path: &Path,
        compressed: Compressed,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let dateindex = StorableVecBuilder::forced_import(path, compressed, options)?;

        let options = options.remove_percentiles();

        Ok(Self {
            dateindex,
            weekindex: StorableVecBuilder::forced_import(path, compressed, options)?,
            difficultyepoch: StorableVecBuilder::forced_import(path, compressed, options)?,
            monthindex: StorableVecBuilder::forced_import(path, compressed, options)?,
            yearindex: StorableVecBuilder::forced_import(path, compressed, options)?,
            // halvingepoch: StorableVecGeneator::forced_import(path, compressed, options)?,
            decadeindex: StorableVecBuilder::forced_import(path, compressed, options)?,
        })
    }

    pub fn compute(
        &mut self,
        source: &mut StorableVec<Height, T>,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.dateindex.compute(
            starting_indexes.dateindex,
            source,
            indexes.dateindex_to_first_height.mut_vec(),
            indexes.dateindex_to_last_height.mut_vec(),
            exit,
        )?;

        self.weekindex.from_aligned(
            starting_indexes.weekindex,
            &mut self.dateindex,
            indexes.weekindex_to_first_dateindex.mut_vec(),
            indexes.weekindex_to_last_dateindex.mut_vec(),
            exit,
        )?;

        self.monthindex.from_aligned(
            starting_indexes.monthindex,
            &mut self.dateindex,
            indexes.monthindex_to_first_dateindex.mut_vec(),
            indexes.monthindex_to_last_dateindex.mut_vec(),
            exit,
        )?;

        self.yearindex.from_aligned(
            starting_indexes.yearindex,
            &mut self.monthindex,
            indexes.yearindex_to_first_monthindex.mut_vec(),
            indexes.yearindex_to_last_monthindex.mut_vec(),
            exit,
        )?;

        self.decadeindex.from_aligned(
            starting_indexes.decadeindex,
            &mut self.yearindex,
            indexes.decadeindex_to_first_yearindex.mut_vec(),
            indexes.decadeindex_to_last_yearindex.mut_vec(),
            exit,
        )?;

        self.difficultyepoch.compute(
            starting_indexes.difficultyepoch,
            source,
            indexes.difficultyepoch_to_first_height.mut_vec(),
            indexes.difficultyepoch_to_last_height.mut_vec(),
            exit,
        )?;

        Ok(())
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        [
            self.dateindex.as_any_vecs(),
            self.weekindex.as_any_vecs(),
            self.difficultyepoch.as_any_vecs(),
            self.monthindex.as_any_vecs(),
            self.yearindex.as_any_vecs(),
            // self.halvingepoch.as_any_vecs(),
            self.decadeindex.as_any_vecs(),
        ]
        .concat()
    }
}
