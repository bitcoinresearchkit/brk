use brk_error::Result;

use brk_indexer::Indexer;
use brk_structs::{DifficultyEpoch, Height, Version};
use vecdb::{AnyCollectableVec, Database, EagerVec, Exit, Format};

use crate::{Indexes, indexes};

use super::{ComputedType, EagerVecBuilder, VecBuilderOptions};

#[derive(Clone)]
pub struct ComputedVecsFromHeightStrict<T>
where
    T: ComputedType + PartialOrd,
{
    pub height: EagerVec<Height, T>,
    pub height_extra: EagerVecBuilder<Height, T>,
    pub difficultyepoch: EagerVecBuilder<DifficultyEpoch, T>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromHeightStrict<T>
where
    T: ComputedType + Ord + From<f64>,
    f64: From<T>,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        format: Format,
        options: VecBuilderOptions,
    ) -> Result<Self> {
        let height = EagerVec::forced_import(db, name, version + VERSION + Version::ZERO, format)?;

        let height_extra = EagerVecBuilder::forced_import(
            db,
            name,
            version + VERSION + Version::ZERO,
            format,
            options.copy_self_extra(),
        )?;

        let options = options.remove_percentiles();

        Ok(Self {
            height,
            height_extra,
            difficultyepoch: EagerVecBuilder::forced_import(
                db,
                name,
                version + VERSION + Version::ZERO,
                format,
                options,
            )?,
            // halvingepoch: StorableVecGeneator::forced_import(db, name, version + VERSION + Version::ZERO, format, options)?,
        })
    }

    pub fn compute<F>(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<Height, T>, &Indexer, &indexes::Vecs, &Indexes, &Exit) -> Result<()>,
    {
        compute(&mut self.height, indexer, indexes, starting_indexes, exit)?;

        self.height_extra
            .extend(starting_indexes.height, &self.height, exit)?;

        self.difficultyepoch.compute(
            starting_indexes.difficultyepoch,
            &self.height,
            &indexes.difficultyepoch_to_first_height,
            &indexes.difficultyepoch_to_height_count,
            exit,
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            vec![&self.height as &dyn AnyCollectableVec],
            self.height_extra.vecs(),
            self.difficultyepoch.vecs(),
            // self.halvingepoch.vecs(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
