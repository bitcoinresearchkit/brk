//! ComputedVecsDate using only max-value aggregation.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, PcoVec};

use crate::{ComputeIndexes, indexes};

use crate::internal::{ComputedVecValue, LazyDateDerivedMax};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedFromDateMax<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub dateindex: EagerVec<PcoVec<DateIndex, T>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: LazyDateDerivedMax<T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromDateMax<T>
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

    /// Import without adding _max suffix to lazy vecs.
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
        let dateindex = EagerVec::forced_import(db, name, version + VERSION)?;

        let rest = if raw {
            LazyDateDerivedMax::from_source_raw(
                name,
                version + VERSION,
                dateindex.boxed_clone(),
                indexes,
            )
        } else {
            LazyDateDerivedMax::from_source(
                name,
                version + VERSION,
                dateindex.boxed_clone(),
                indexes,
            )
        };

        Ok(Self {
            rest,
            dateindex,
        })
    }

    pub fn compute_all<F>(
        &mut self,
        _starting_indexes: &ComputeIndexes,
        _exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<DateIndex, T>>) -> Result<()>,
    {
        compute(&mut self.dateindex)?;
        Ok(())
    }
}
