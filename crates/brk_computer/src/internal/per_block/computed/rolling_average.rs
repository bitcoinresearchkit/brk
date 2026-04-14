//! PerBlock with rolling average (no distribution stats).
//!
//! Stored height data + cumulative + lazy 4-window rolling averages.
//! Rolling averages are computed on-the-fly from the cumulative via DeltaAvg.
//!
//! Type parameters:
//! - `T`: per-block value type
//! - `C`: cumulative type, defaults to `T`. Use a wider type (e.g., `StoredU64`)
//!   when the prefix sum of `T` values could overflow `T`.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode};

use crate::indexes;

use crate::internal::{LazyRollingAvgsFromHeight, NumericValue, WindowStartVec, Windows};

#[derive(Traversable)]
pub struct PerBlockRollingAverage<T, C = T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
{
    pub block: M::Stored<EagerVec<PcoVec<Height, T>>>,
    #[traversable(hidden)]
    pub cumulative: M::Stored<EagerVec<PcoVec<Height, C>>>,
    #[traversable(flatten)]
    pub average: LazyRollingAvgsFromHeight<C>,
}

impl<T, C> PerBlockRollingAverage<T, C>
where
    T: NumericValue + JsonSchema + Into<C>,
    C: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let block: EagerVec<PcoVec<Height, T>> = EagerVec::forced_import(db, name, version)?;
        let cumulative: EagerVec<PcoVec<Height, C>> =
            EagerVec::forced_import(db, &format!("{name}_cumulative"), version + Version::TWO)?;
        let average = LazyRollingAvgsFromHeight::new(
            &format!("{name}_average"),
            version + Version::TWO,
            &cumulative,
            cached_starts,
            indexes,
        );

        Ok(Self {
            block,
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
        compute_height(&mut self.block)?;
        self.compute_rest(max_from, exit)
    }

    /// Compute cumulative from already-populated height data. Rolling averages are lazy.
    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()> {
        self.cumulative
            .compute_cumulative(max_from, &self.block, exit)?;
        Ok(())
    }
}
