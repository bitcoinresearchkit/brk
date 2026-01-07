//! DerivedComputedBlockLast - dateindex storage + difficultyepoch + lazy time periods.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, Exit, IterableBoxedVec, IterableCloneableVec, IterableVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedVecValue, DerivedDateLast, LastVec, LazyLast, NumericValue},
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct DerivedComputedBlockLast<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub dateindex: LastVec<DateIndex, T>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dates: DerivedDateLast<T>,
    pub difficultyepoch: LazyLast<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> DerivedComputedBlockLast<T>
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
            dates: DerivedDateLast::from_source(name, v, dateindex.0.boxed_clone(), indexes),
            difficultyepoch: LazyLast::from_source(
                name,
                v,
                height_source,
                indexes
                    .block
                    .difficultyepoch_to_difficultyepoch
                    .boxed_clone(),
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
            &indexes.time.dateindex_to_first_height,
            &indexes.time.dateindex_to_height_count,
            exit,
        )?;
        Ok(())
    }
}
