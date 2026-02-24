//! ComputedFromHeightCumSum - stored height + LazyLast + cumulative (from height) + RollingWindows (sum).
//!
//! Like ComputedFromHeightCumFull but with rolling sum only (no distribution).
//! Used for count metrics where distribution stats aren't meaningful.
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
    internal::{ComputedFromHeightLast, NumericValue, RollingWindows, WindowStarts},
};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedFromHeightCumSum<T, M: StorageMode = Rw>
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
    pub rolling: RollingWindows<T, M>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightCumSum<T>
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
        let rolling = RollingWindows::forced_import(db, name, v, indexes)?;

        Ok(Self {
            last,
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
        compute_height(&mut self.last.height)?;
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.last.height, exit)?;
        self.rolling
            .compute_rolling_sum(max_from, windows, &self.last.height, exit)?;
        Ok(())
    }
}
