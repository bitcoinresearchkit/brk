//! ComputedHeightDerivedLast - dateindex storage + difficultyepoch + lazy time periods.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, Exit, IterableBoxedVec, IterableCloneableVec, IterableVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedVecValue, LazyDateDerivedLast, LastVec, LazyLast, NumericValue},
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedHeightDerivedLast<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub dateindex: LastVec<DateIndex, T>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dates: LazyDateDerivedLast<T>,
    pub difficultyepoch: LazyLast<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedHeightDerivedLast<T>
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
        let dateindex = LastVec::forced_import(db, name, version + VERSION)?;
        let v = version + VERSION;

        Ok(Self {
            dates: LazyDateDerivedLast::from_source(name, v, dateindex.boxed_clone(), indexes),
            difficultyepoch: LazyLast::from_source(
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
        self.dateindex.compute_last(
            starting_indexes.dateindex,
            height_source,
            &indexes.dateindex.first_height,
            &indexes.dateindex.height_count,
            exit,
        )?;
        Ok(())
    }
}
