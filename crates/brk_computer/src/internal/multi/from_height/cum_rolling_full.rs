//! ComputedFromHeightCumFull - stored height + LazyLast + cumulative (from height) + RollingFull.
//!
//! For metrics with stored per-block data, cumulative sums, and rolling windows.
//! Cumulative gets its own ComputedFromHeightLast so it has LazyLast index views too.

use std::ops::SubAssign;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedFromHeightLast, NumericValue, RollingFull, WindowStarts},
};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedFromHeightCumFull<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub last: ComputedFromHeightLast<T, M>,
    #[traversable(flatten)]
    pub cumulative: ComputedFromHeightLast<T, M>,
    #[traversable(flatten)]
    pub rolling: RollingFull<T, M>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightCumFull<T>
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

        let last = ComputedFromHeightLast::forced_import(db, name, v, indexes)?;
        let cumulative = ComputedFromHeightLast::forced_import(
            db,
            &format!("{name}_cumulative"),
            v,
            indexes,
        )?;
        let rolling = RollingFull::forced_import(db, name, v, indexes)?;

        Ok(Self {
            last,
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
        compute_height(&mut self.last.height)?;
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.last.height, exit)?;
        self.rolling
            .compute(max_from, windows, &self.last.height, exit)?;
        Ok(())
    }
}
