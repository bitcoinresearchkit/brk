//! ComputedFromHeightAndDateMin - height storage + dateindex storage + lazy periods.
//!
//! Use this when both height and dateindex are stored EagerVecs with min-value aggregation.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, PcoVec};

use crate::{ComputeIndexes, indexes};

use crate::internal::{ComputedFromDateMin, ComputedVecValue, LazyMin};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedFromHeightAndDateMin<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub height: EagerVec<PcoVec<Height, T>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: ComputedFromDateMin<T>,
    pub difficultyepoch: LazyMin<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightAndDateMin<T>
where
    T: ComputedVecValue + JsonSchema + 'static,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Self::forced_import_inner(db, name, version, indexes, false)
    }

    /// Import without adding _min suffix to lazy vecs.
    pub fn forced_import_raw(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Self::forced_import_inner(db, name, version, indexes, true)
    }

    fn forced_import_inner(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        raw: bool,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height: EagerVec<PcoVec<Height, T>> = EagerVec::forced_import(db, name, v)?;
        let rest = if raw {
            ComputedFromDateMin::forced_import_raw(db, name, v, indexes)?
        } else {
            ComputedFromDateMin::forced_import(db, name, v, indexes)?
        };
        let difficultyepoch = if raw {
            LazyMin::from_source_raw(
                name,
                v,
                height.boxed_clone(),
                indexes.difficultyepoch.identity.boxed_clone(),
            )
        } else {
            LazyMin::from_source(
                name,
                v,
                height.boxed_clone(),
                indexes.difficultyepoch.identity.boxed_clone(),
            )
        };

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
