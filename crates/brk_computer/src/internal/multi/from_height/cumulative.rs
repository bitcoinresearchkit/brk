//! ComputedFromHeightCumulative - stored height + LazyAggVec + cumulative (from height).
//!
//! Like ComputedFromHeightCumulativeSum but without RollingWindows.
//! Used for distribution metrics where rolling is optional per cohort.
//! Cumulative gets its own ComputedFromHeightLast so it has LazyAggVec index views.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedFromHeightLast, NumericValue},
};

#[derive(Traversable)]
pub struct ComputedFromHeightCumulative<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub height: M::Stored<EagerVec<PcoVec<Height, T>>>,
    pub cumulative: ComputedFromHeightLast<T, M>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightCumulative<T>
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
        let cumulative =
            ComputedFromHeightLast::forced_import(db, &format!("{name}_cumulative"), v, indexes)?;

        Ok(Self { height, cumulative })
    }

    /// Compute height data via closure, then cumulative only (no rolling).
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        exit: &Exit,
        compute_height: impl FnOnce(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: Default,
    {
        compute_height(&mut self.height)?;
        self.compute_rest(max_from, exit)
    }

    /// Compute cumulative from already-filled height vec.
    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()>
    where
        T: Default,
    {
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.height, exit)?;
        Ok(())
    }
}
