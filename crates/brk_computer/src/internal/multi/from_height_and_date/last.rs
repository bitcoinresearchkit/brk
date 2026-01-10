//! ComputedFromHeightAndDateLast - height storage + dateindex storage + lazy periods.
//!
//! Use this when both height and dateindex are stored EagerVecs with last-value aggregation.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, PcoVec};

use crate::{indexes, internal::ComputedFromDateLast, ComputeIndexes};

use crate::internal::{ComputedVecValue, LazyLast};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedFromHeightAndDateLast<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub height: EagerVec<PcoVec<Height, T>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: ComputedFromDateLast<T>,
    pub difficultyepoch: LazyLast<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightAndDateLast<T>
where
    T: ComputedVecValue + JsonSchema + 'static,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height: EagerVec<PcoVec<Height, T>> = EagerVec::forced_import(db, name, v)?;
        let rest = ComputedFromDateLast::forced_import(db, name, v, indexes)?;
        let difficultyepoch = LazyLast::from_source(
            name,
            v,
            height.boxed_clone(),
            indexes.difficultyepoch.identity.boxed_clone(),
        );

        Ok(Self {
            height,
            rest,
            difficultyepoch,
        })
    }

    /// Compute rest (dateindex + periods) with the given compute function.
    pub fn compute_rest<F>(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<DateIndex, T>>) -> Result<()>,
    {
        self.rest.compute_all(starting_indexes, exit, compute)
    }
}
