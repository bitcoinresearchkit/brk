//! ComputedDerivedBlockFirst - dateindex storage + difficultyepoch + lazy time periods (first value).

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, Exit, IterableBoxedVec, IterableCloneableVec, IterableVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedVecValue, LazyPeriodsFirst, FirstVec, LazyFirst, NumericValue},
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedDerivedBlockFirst<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub dateindex: FirstVec<DateIndex, T>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dates: LazyPeriodsFirst<T>,
    pub difficultyepoch: LazyFirst<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedDerivedBlockFirst<T>
where
    T: NumericValue + JsonSchema,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        height_source: IterableBoxedVec<Height, T>,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let dateindex = FirstVec::forced_import(db, name, version + VERSION)?;
        let v = version + VERSION;

        Ok(Self {
            dates: LazyPeriodsFirst::from_source(name, v, dateindex.0.boxed_clone(), indexes),
            difficultyepoch: LazyFirst::from_source(
                name,
                v,
                height_source,
                indexes.difficultyepoch.identity.boxed_clone(),
            ),
            dateindex,
        })
    }

    pub fn derive_from(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        height_source: &impl IterableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()> {
        self.dateindex.compute_first(
            starting_indexes.dateindex,
            height_source,
            &indexes.dateindex.first_height,
            &indexes.dateindex.height_count,
            exit,
        )?;
        Ok(())
    }

    pub fn compute_all<F>(&mut self, mut compute: F) -> Result<()>
    where
        F: FnMut(&mut FirstVec<DateIndex, T>) -> Result<()>,
    {
        compute(&mut self.dateindex)?;
        Ok(())
    }
}
