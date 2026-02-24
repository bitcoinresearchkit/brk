//! ComputedFromHeight with full stats aggregation.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, ReadableCloneableVec, Rw, StorageMode};

use crate::indexes;

use crate::internal::{ComputedHeightDerivedFull, ComputedVecValue, NumericValue};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedFromHeightFull<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    #[traversable(rename = "base")]
    pub height: M::Stored<EagerVec<PcoVec<Height, T>>>,
    #[deref]
    #[deref_mut]
    pub rest: Box<ComputedHeightDerivedFull<T, M>>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightFull<T>
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

        let rest = ComputedHeightDerivedFull::forced_import(
            db,
            name,
            height.read_only_boxed_clone(),
            v,
            indexes,
        )?;

        Ok(Self {
            height,
            rest: Box::new(rest),
        })
    }
}
