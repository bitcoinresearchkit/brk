//! ComputedFromHeightCumulativeFull - stored height + LazyAggVec + cumulative (from height) + RollingFull.
//!
//! For metrics with stored per-block data, cumulative sums, and rolling windows.
//! Cumulative gets its own ComputedFromHeightLast so it has LazyAggVec index views too.

use std::ops::SubAssign;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedFromHeightLast, NumericValue, RollingFull, WindowStarts},
};

#[derive(Traversable)]
pub struct ComputedFromHeightCumulativeFull<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub height: M::Stored<EagerVec<PcoVec<Height, T>>>,
    pub cumulative: ComputedFromHeightLast<T, M>,
    pub rolling: RollingFull<T, M>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightCumulativeFull<T>
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
        let rolling = RollingFull::forced_import(db, name, v, indexes)?;

        Ok(Self {
            height,
            cumulative,
            rolling,
        })
    }

    /// Compute height data via closure, then cumulative + rolling.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        compute_height: impl FnOnce(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: From<f64> + Default + SubAssign + Copy + Ord,
        f64: From<T>,
    {
        compute_height(&mut self.height)?;
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.height, exit)?;
        self.rolling
            .compute(max_from, windows, &self.height, exit)?;
        Ok(())
    }
}
