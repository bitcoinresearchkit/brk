use std::path::Path;

use brk_core::{Difficultyepoch, Height};
use brk_exit::Exit;
use brk_vec::{AnyStorableVec, Compressed, Result, Version};

use crate::storage::vecs::{Indexes, base::ComputedVec, indexes};

use super::{ComputedType, ComputedVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedVecsFromHeightStrict<T>
where
    T: ComputedType + PartialOrd,
{
    pub height: ComputedVec<Height, T>,
    pub difficultyepoch: ComputedVecBuilder<Difficultyepoch, T>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
}

impl<T> ComputedVecsFromHeightStrict<T>
where
    T: ComputedType + Ord + From<f64>,
    f64: From<T>,
{
    pub fn forced_import(
        path: &Path,
        name: &str,
        version: Version,
        compressed: Compressed,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let height = ComputedVec::forced_import(
            &path.join(format!("height_to_{name}")),
            version,
            compressed,
        )?;

        let options = options.remove_percentiles();

        Ok(Self {
            height,
            difficultyepoch: ComputedVecBuilder::forced_import(path, name, compressed, options)?,
            // halvingepoch: StorableVecGeneator::forced_import(path, name, compressed, options)?,
        })
    }

    pub fn compute<F>(
        &mut self,
        mut compute: F,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()>
    where
        F: FnMut(&mut ComputedVec<Height, T>) -> Result<()>,
    {
        compute(&mut self.height)?;

        self.difficultyepoch.compute(
            starting_indexes.difficultyepoch,
            &mut self.height,
            indexes.difficultyepoch_to_first_height.mut_vec(),
            indexes.difficultyepoch_to_last_height.mut_vec(),
            exit,
        )?;

        Ok(())
    }

    pub fn any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        [
            vec![self.height.any_vec()],
            self.difficultyepoch.any_vecs(),
            // self.halvingepoch.as_any_vecs(),
        ]
        .concat()
    }
}
