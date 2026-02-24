//! ComputedFromHeightCum - stored height + LazyLast + cumulative (from height).
//!
//! Like ComputedFromHeightCumSum but without RollingWindows.
//! Used for distribution metrics where rolling is optional per cohort.
//! Cumulative gets its own ComputedFromHeightLast so it has LazyLast index views.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedFromHeightLast, NumericValue},
};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedFromHeightCum<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub last: ComputedFromHeightLast<T, M>,
    #[traversable(flatten)]
    pub cumulative: ComputedFromHeightLast<T, M>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightCum<T>
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

        Ok(Self { last, cumulative })
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
        compute_height(&mut self.last.height)?;
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.last.height, exit)?;
        Ok(())
    }

    /// Compute cumulative from already-filled height vec.
    pub(crate) fn compute_cumulative(
        &mut self,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Default,
    {
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.last.height, exit)?;
        Ok(())
    }
}
