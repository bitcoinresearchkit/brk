//! ComputedFromHeightCumulativeSum - stored height + LazyAggVec + cumulative (from height) + RollingWindows (sum).
//!
//! Like ComputedFromHeightCumulativeFull but with rolling sum only (no distribution).
//! Used for count metrics where distribution stats aren't meaningful.
//! Cumulative gets its own ComputedFromHeightLast so it has LazyAggVec index views too.

use std::ops::SubAssign;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedFromHeightLast, NumericValue, RollingWindows, WindowStarts},
};

#[derive(Traversable)]
#[traversable(merge)]
pub struct ComputedFromHeightCumulativeSum<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub height: M::Stored<EagerVec<PcoVec<Height, T>>>,
    #[traversable(flatten)]
    pub cumulative: ComputedFromHeightLast<T, M>,
    #[traversable(flatten)]
    pub rolling: RollingWindows<T, M>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightCumulativeSum<T>
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
        let rolling = RollingWindows::forced_import(db, name, v, indexes)?;

        Ok(Self {
            height,
            cumulative,
            rolling,
        })
    }

    /// Compute height data via closure, then cumulative + rolling sum.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        compute_height: impl FnOnce(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: Default + SubAssign,
    {
        compute_height(&mut self.height)?;
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.height, exit)?;
        self.rolling
            .compute_rolling_sum(max_from, windows, &self.height, exit)?;
        Ok(())
    }
}
