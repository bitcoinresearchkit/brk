use std::path::Path;

use brk_core::{DifficultyEpoch, Height};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyStoredVec, Compressed, Result, Version};

use crate::storage::{ComputedType, EagerVec, Indexes, indexes};

use super::{ComputedVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedVecsFromHeightStrict<T>
where
    T: ComputedType + PartialOrd,
{
    pub height: EagerVec<Height, T>,
    pub height_extra: ComputedVecBuilder<Height, T>,
    pub difficultyepoch: ComputedVecBuilder<DifficultyEpoch, T>,
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

        let height =
            EagerVec::forced_import(&path.join(format!("height_to_{name}")), version, compressed)?;

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
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> color_eyre::Result<()>
    where
        F: FnMut(&mut EagerVec<Height, T>, &Indexer, &indexes::Vecs, &Indexes, &Exit) -> Result<()>,
    {
        compute(&mut self.height, indexer, indexes, starting_indexes, exit)?;

        self.height_extra
            .extend(starting_indexes.height, self.height.vec(), exit)?;

        self.difficultyepoch.compute(
            starting_indexes.difficultyepoch,
            self.height.vec(),
            indexes.difficultyepoch_to_first_height.vec(),
            indexes.difficultyepoch_to_height_count.vec(),
            exit,
        )?;

        Ok(())
    }

    pub fn any_vecs(&self) -> Vec<&dyn AnyStoredVec> {
        [
            vec![self.height.any_vec()],
            self.height_extra.any_vecs(),
            self.difficultyepoch.any_vecs(),
            // self.halvingepoch.any_vecs(),
        ]
        .concat()
    }
}
