//! ComputedDateAverage - dateindex storage + lazy periods for average-value aggregation.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, PcoVec};

use crate::{ComputeIndexes, indexes};

use crate::internal::{ComputedVecValue, LazyPeriodsAverage};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedDateAverage<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub dateindex: EagerVec<PcoVec<DateIndex, T>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: LazyPeriodsAverage<T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedDateAverage<T>
where
    T: ComputedVecValue + JsonSchema + 'static,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let dateindex = EagerVec::forced_import(db, name, version + VERSION)?;

        Ok(Self {
            rest: LazyPeriodsAverage::from_source(
                name,
                version + VERSION,
                dateindex.boxed_clone(),
                indexes,
            ),
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
