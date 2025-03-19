use std::path::Path;

use brk_core::{
    Dateindex, Decadeindex, Difficultyepoch, Halvingepoch, Height, Monthindex, Weekindex, Yearindex,
};
use brk_indexer::{Indexer, Indexes};
use brk_vec::{AnyStorableVec, Compressed};

use crate::storage::vecs::{base::StorableVec, indexes};

use super::{StorableVecGeneator, StorableVecGeneatorOptions, StoredType};

#[derive(Clone)]
pub struct StorableVecGeneatorByIndex<T>
where
    T: StoredType,
{
    pub dateindex: StorableVecGeneator<Dateindex, T>,
    pub weekindex: StorableVecGeneator<Weekindex, T>,
    pub difficultyepoch: StorableVecGeneator<Difficultyepoch, T>,
    pub monthindex: StorableVecGeneator<Monthindex, T>,
    pub yearindex: StorableVecGeneator<Yearindex, T>,
    pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
    pub decadeindex: StorableVecGeneator<Decadeindex, T>,
}

impl<T> StorableVecGeneatorByIndex<T>
where
    T: StoredType,
{
    pub fn forced_import(
        path: &Path,
        compressed: Compressed,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let dateindex = StorableVecGeneator::forced_import(path, compressed, options)?;

        let options = options.remove_percentiles();

        Ok(Self {
            dateindex,
            weekindex: StorableVecGeneator::forced_import(path, compressed, options)?,
            difficultyepoch: StorableVecGeneator::forced_import(path, compressed, options)?,
            monthindex: StorableVecGeneator::forced_import(path, compressed, options)?,
            yearindex: StorableVecGeneator::forced_import(path, compressed, options)?,
            halvingepoch: StorableVecGeneator::forced_import(path, compressed, options)?,
            decadeindex: StorableVecGeneator::forced_import(path, compressed, options)?,
        })
    }

    pub fn compute(
        &mut self,
        source: &mut StorableVec<Height, T>,
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
    ) {
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        [
            self.dateindex.as_any_vecs(),
            self.weekindex.as_any_vecs(),
            self.difficultyepoch.as_any_vecs(),
            self.monthindex.as_any_vecs(),
            self.yearindex.as_any_vecs(),
            self.halvingepoch.as_any_vecs(),
            self.decadeindex.as_any_vecs(),
        ]
        .concat()
    }
}
