use std::path::Path;

use brk_core::{Difficultyepoch, Height};
use brk_exit::Exit;
use brk_vec::{AnyStorableVec, Compressed};

use crate::storage::vecs::{Indexes, base::StorableVec, indexes};

use super::{ComputedType, StorableVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct StorableVecsStatsFromHeightStrict<T>
where
    T: ComputedType + PartialOrd,
{
    pub difficultyepoch: StorableVecBuilder<Difficultyepoch, T>,
    // pub halvingepoch: StorableVecGeneator<Halvingepoch, T>, // TODO
}

impl<T> StorableVecsStatsFromHeightStrict<T>
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
            difficultyepoch: StorableVecBuilder::forced_import(path, compressed, options)?,
            // halvingepoch: StorableVecGeneator::forced_import(path, compressed, options)?,
        })
    }

    pub fn compute(
        &mut self,
        source: &mut StorableVec<Height, T>,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
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
            self.difficultyepoch.as_any_vecs(),
            // self.halvingepoch.as_any_vecs(),
        ]
        .concat()
    }
}
