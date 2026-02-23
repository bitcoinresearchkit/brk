//! ComputedFromHeight using Sum-only aggregation.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, ReadableCloneableVec, Rw, StorageMode};

use crate::indexes;

use crate::internal::{ComputedHeightDerivedSum, ComputedVecValue, NumericValue};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedFromHeightSum<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub height: M::Stored<EagerVec<PcoVec<Height, T>>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: Box<ComputedHeightDerivedSum<T>>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightSum<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height: EagerVec<PcoVec<Height, T>> = EagerVec::forced_import(db, name, v)?;

        let rest =
            ComputedHeightDerivedSum::forced_import(name, height.read_only_boxed_clone(), v, indexes);

        Ok(Self { height, rest: Box::new(rest) })
    }
}
