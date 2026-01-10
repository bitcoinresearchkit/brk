//! ComputedFromHeight with full stats aggregation.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, PcoVec};

use crate::{ComputeIndexes, indexes};

use crate::internal::{ComputedVecValue, ComputedHeightDerivedFull, NumericValue};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedFromHeightFull<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    #[traversable(rename = "base")]
    pub height: EagerVec<PcoVec<Height, T>>,
    #[deref]
    #[deref_mut]
    pub rest: ComputedHeightDerivedFull<T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightFull<T>
where
    T: NumericValue + JsonSchema,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height: EagerVec<PcoVec<Height, T>> = EagerVec::forced_import(db, name, v)?;

        let rest = ComputedHeightDerivedFull::forced_import(
            db,
            name,
            height.boxed_clone(),
            v,
            indexes,
        )?;

        Ok(Self { height, rest })
    }

    pub fn compute_all<F>(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    {
        compute(&mut self.height)?;
        self.rest.derive_from(indexes, starting_indexes, &self.height, exit)
    }
}
