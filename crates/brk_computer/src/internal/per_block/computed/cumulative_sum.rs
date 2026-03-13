//! ComputedPerBlockCumulativeWithSums - raw ComputedPerBlock + cumulative ComputedPerBlock + lazy rolling sums.
//!
//! Rolling sums are derived lazily from the cumulative vec via LazyDeltaVec.
//! No rolling sum vecs are stored on disk.
//!
//! Type parameters:
//! - `T`: per-block value type (e.g., `StoredU32` for tx counts)
//! - `M`: storage mode (`Rw` or `Ro`)
//! - `C`: cumulative type, defaults to `T`. Use a wider type (e.g., `StoredU64`)
//!   when the prefix sum of `T` values could overflow `T`.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{CachedWindowStarts, ComputedPerBlock, LazyRollingSumsFromHeight, NumericValue},
};

#[derive(Traversable)]
pub struct ComputedPerBlockCumulativeWithSums<T, C, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
{
    pub raw: ComputedPerBlock<T, M>,
    pub cumulative: ComputedPerBlock<C, M>,
    pub sum: LazyRollingSumsFromHeight<C>,
}

impl<T, C> ComputedPerBlockCumulativeWithSums<T, C>
where
    T: NumericValue + JsonSchema + Into<C>,
    C: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let raw = ComputedPerBlock::forced_import(db, name, version, indexes)?;
        let cumulative =
            ComputedPerBlock::forced_import(db, &format!("{name}_cumulative"), version, indexes)?;
        let sum = LazyRollingSumsFromHeight::new(
            &format!("{name}_sum"),
            version,
            &cumulative.height,
            cached_starts,
            indexes,
        );

        Ok(Self {
            raw,
            cumulative,
            sum,
        })
    }

    /// Compute raw data via closure, then cumulative. Rolling sums are lazy.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        exit: &Exit,
        compute_raw: impl FnOnce(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()>
    where
        C: Default,
    {
        compute_raw(&mut self.raw.height)?;
        self.compute_rest(max_from, exit)
    }

    /// Compute cumulative from already-populated raw data. Rolling sums are lazy.
    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()>
    where
        C: Default,
    {
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.raw.height, exit)?;
        Ok(())
    }
}
