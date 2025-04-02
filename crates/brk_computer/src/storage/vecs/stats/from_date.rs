use std::path::Path;

use brk_core::{Dateindex, Decadeindex, Monthindex, Quarterindex, Weekindex, Yearindex};
use brk_exit::Exit;
use brk_vec::{AnyStorableVec, Compressed};

use crate::storage::vecs::{Indexes, base::StorableVec, indexes};

use super::{ComputedType, StorableVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct StorableVecsStatsFromDate<T>
where
    T: ComputedType + PartialOrd,
{
    pub weekindex: StorableVecBuilder<Weekindex, T>,
    pub monthindex: StorableVecBuilder<Monthindex, T>,
    pub quarterindex: StorableVecBuilder<Quarterindex, T>,
    pub yearindex: StorableVecBuilder<Yearindex, T>,
    pub decadeindex: StorableVecBuilder<Decadeindex, T>,
}

impl<T> StorableVecsStatsFromDate<T>
where
    T: ComputedType + Ord + From<f64>,
    f64: From<T>,
{
    pub fn forced_import(
        path: &Path,
        compressed: Compressed,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let options = options.remove_percentiles();

        Ok(Self {
            weekindex: StorableVecBuilder::forced_import(path, compressed, options)?,
            monthindex: StorableVecBuilder::forced_import(path, compressed, options)?,
            quarterindex: StorableVecBuilder::forced_import(path, compressed, options)?,
            yearindex: StorableVecBuilder::forced_import(path, compressed, options)?,
            decadeindex: StorableVecBuilder::forced_import(path, compressed, options)?,
        })
    }

    pub fn compute(
        &mut self,
        source: &mut StorableVec<Dateindex, T>,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.weekindex.compute(
            starting_indexes.weekindex,
            source,
            indexes.weekindex_to_first_dateindex.mut_vec(),
            indexes.weekindex_to_last_dateindex.mut_vec(),
            exit,
        )?;

        self.monthindex.compute(
            starting_indexes.monthindex,
            source,
            indexes.monthindex_to_first_dateindex.mut_vec(),
            indexes.monthindex_to_last_dateindex.mut_vec(),
            exit,
        )?;

        self.quarterindex.from_aligned(
            starting_indexes.quarterindex,
            &mut self.monthindex,
            indexes.quarterindex_to_first_monthindex.mut_vec(),
            indexes.quarterindex_to_last_monthindex.mut_vec(),
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

        Ok(())
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        [
            self.weekindex.as_any_vecs(),
            self.monthindex.as_any_vecs(),
            self.quarterindex.as_any_vecs(),
            self.yearindex.as_any_vecs(),
            self.decadeindex.as_any_vecs(),
        ]
        .concat()
    }
}
