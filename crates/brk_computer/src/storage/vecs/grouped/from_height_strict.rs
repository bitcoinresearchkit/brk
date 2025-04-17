use std::path::Path;

use brk_core::{Difficultyepoch, Height};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyStoredVec, Compressed, Result, Version};

use crate::storage::vecs::{Indexes, base::ComputedVec, indexes};

use super::{ComputedType, ComputedVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedVecsFromHeightStrict<T>
where
    T: ComputedType + PartialOrd,
{
    pub height: ComputedVec<Height, T>,
    pub height_extra: ComputedVecBuilder<Height, T>,
    pub difficultyepoch: ComputedVecBuilder<Difficultyepoch, T>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
}

const VERSION: Version = Version::ZERO;

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
        let version = VERSION + version;

        let height = ComputedVec::forced_import(
            &path.join(format!("height_to_{name}")),
            version,
            compressed,
        )?;

        let height_extra = ComputedVecBuilder::forced_import(
            path,
            name,
            version,
            compressed,
            options.copy_self_extra(),
        )?;

        let options = options.remove_percentiles();

        Ok(Self {
            height,
            height_extra,
            difficultyepoch: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
            // halvingepoch: StorableVecGeneator::forced_import(path, name, version, compressed, options)?,
        })
    }

    pub fn compute<F>(
        &mut self,
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> color_eyre::Result<()>
    where
        F: FnMut(
            &mut ComputedVec<Height, T>,
            &mut Indexer,
            &mut indexes::Vecs,
            &Indexes,
            &Exit,
        ) -> Result<()>,
    {
        compute(&mut self.height, indexer, indexes, starting_indexes, exit)?;

        self.height_extra
            .extend(starting_indexes.height, self.height.mut_vec(), exit)?;

        self.difficultyepoch.compute(
            starting_indexes.difficultyepoch,
            self.height.mut_vec(),
            indexes.difficultyepoch_to_first_height.mut_vec(),
            indexes.difficultyepoch_to_last_height.mut_vec(),
            exit,
        )?;

        Ok(())
    }

    pub fn any_vecs(&self) -> Vec<&dyn AnyStoredVec> {
        [
            vec![self.height.any_vec()],
            self.height_extra.any_vecs(),
            self.difficultyepoch.any_vecs(),
            // self.halvingepoch.as_any_vecs(),
        ]
        .concat()
    }
}
