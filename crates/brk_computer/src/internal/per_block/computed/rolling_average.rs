//! ComputedPerBlock with rolling average (no distribution stats).
//!
//! Stored height data + 4-window rolling averages (24h, 1w, 1m, 1y).
//! Use instead of ComputedPerBlockDistribution when only the average
//! is analytically useful (e.g., block interval, activity counts).

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode};

use crate::indexes;

use crate::internal::{NumericValue, RollingWindows, WindowStarts};

#[derive(Traversable)]
pub struct ComputedPerBlockRollingAverage<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    pub height: M::Stored<EagerVec<PcoVec<Height, T>>>,
    #[traversable(flatten)]
    pub average: RollingWindows<T, M>,
}

impl<T> ComputedPerBlockRollingAverage<T>
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
        let average =
            RollingWindows::forced_import(db, &format!("{name}_average"), version + Version::ONE, indexes)?;

        Ok(Self { height, average })
    }

    /// Compute height data via closure, then rolling averages.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
        compute_height: impl FnOnce(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()>
    where
        T: Default,
        f64: From<T>,
    {
        compute_height(&mut self.height)?;
        self.compute_rest(max_from, windows, exit)
    }

    /// Compute rolling averages from already-populated height data.
    pub(crate) fn compute_rest(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Default,
        f64: From<T>,
    {
        for (w, starts) in self.average.0.as_mut_array().into_iter().zip(windows.as_array()) {
            w.height
                .compute_rolling_average(max_from, *starts, &self.height, exit)?;
        }
        Ok(())
    }
}
