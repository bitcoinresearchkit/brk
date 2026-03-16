//! PerBlock with rolling average (no distribution stats).
//!
//! Stored height data + f64 cumulative + lazy 4-window rolling averages.
//! Rolling averages are computed on-the-fly from the cumulative via DeltaAvg.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode};

use crate::indexes;

use crate::internal::{CachedWindowStarts, LazyRollingAvgsFromHeight, NumericValue};

#[derive(Traversable)]
pub struct PerBlockRollingAverage<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub base: M::Stored<EagerVec<PcoVec<Height, T>>>,
    #[traversable(hidden)]
    pub cumulative: M::Stored<EagerVec<PcoVec<Height, f64>>>,
    #[traversable(flatten)]
    pub average: LazyRollingAvgsFromHeight<T>,
}

impl<T> PerBlockRollingAverage<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let base: EagerVec<PcoVec<Height, T>> = EagerVec::forced_import(db, name, version)?;
        let cumulative: EagerVec<PcoVec<Height, f64>> =
            EagerVec::forced_import(db, &format!("{name}_cumulative"), version)?;
        let average = LazyRollingAvgsFromHeight::new(
            &format!("{name}_average"),
            version + Version::ONE,
            &cumulative,
            cached_starts,
            indexes,
        );

        Ok(Self {
            base,
            cumulative,
            average,
        })
    }

    /// Compute height data via closure, then cumulative. Rolling averages are lazy.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        exit: &Exit,
        compute_height: impl FnOnce(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()> {
        compute_height(&mut self.base)?;
        self.compute_rest(max_from, exit)
    }

    /// Compute cumulative from already-populated height data. Rolling averages are lazy.
    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()> {
        self.cumulative
            .compute_cumulative(max_from, &self.base, exit)?;
        Ok(())
    }
}
