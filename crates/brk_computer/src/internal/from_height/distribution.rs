//! ComputedFromHeight using Distribution aggregation (no sum/cumulative).
//!
//! Stored height data + LazyAggVec index views + rolling distribution windows.
//! Use for block-based metrics where sum/cumulative would be misleading
//! (e.g., activity counts that can't be deduplicated across blocks).

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode};

use crate::indexes;

use crate::internal::{ComputedVecValue, NumericValue, RollingDistribution, WindowStarts};

#[derive(Traversable)]
pub struct ComputedFromHeightDistribution<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub height: M::Stored<EagerVec<PcoVec<Height, T>>>,
    #[traversable(flatten)]
    pub rolling: RollingDistribution<T, M>,
}

impl<T> ComputedFromHeightDistribution<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let height: EagerVec<PcoVec<Height, T>> = EagerVec::forced_import(db, name, version)?;
        let rolling = RollingDistribution::forced_import(db, name, version, indexes)?;

        Ok(Self { height, rolling })
    }

    /// Compute height data via closure, then rolling distribution.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        compute_height: impl FnOnce(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: Copy + Ord + From<f64> + Default,
        f64: From<T>,
    {
        compute_height(&mut self.height)?;
        self.compute_rest(max_from, windows, exit)
    }

    /// Compute rolling distribution from already-populated height data.
    pub(crate) fn compute_rest(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Copy + Ord + From<f64> + Default,
        f64: From<T>,
    {
        self.rolling
            .compute_distribution(max_from, windows, &self.height, exit)?;
        Ok(())
    }
}
