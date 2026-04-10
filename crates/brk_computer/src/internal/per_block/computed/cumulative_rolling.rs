//! PerBlockCumulativeRolling - base EagerVec + cumulative PerBlock + lazy rolling sums.
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
use vecdb::{Database, EagerVec, Exit, ImportableVec, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        LazyRollingAvgsFromHeight, LazyRollingSumsFromHeight, NumericValue, PerBlock,
        WindowStartVec, Windows,
    },
};

#[derive(Traversable)]
pub struct PerBlockCumulativeRolling<T, C, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
{
    pub block: M::Stored<EagerVec<PcoVec<Height, T>>>,
    pub cumulative: PerBlock<C, M>,
    pub sum: LazyRollingSumsFromHeight<C>,
    pub average: LazyRollingAvgsFromHeight<C>,
}

impl<T, C> PerBlockCumulativeRolling<T, C>
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
        let block = EagerVec::forced_import(db, name, version)?;
        let cumulative =
            PerBlock::forced_import(db, &format!("{name}_cumulative"), version, indexes)?;
        let sum = LazyRollingSumsFromHeight::new(
            &format!("{name}_sum"),
            version,
            &cumulative.height,
            cached_starts,
            indexes,
        );
        let average = LazyRollingAvgsFromHeight::new(
            &format!("{name}_average"),
            version,
            &cumulative.height,
            cached_starts,
            indexes,
        );

        Ok(Self {
            block,
            cumulative,
            sum,
            average,
        })
    }

    /// Compute base data via closure, then cumulative. Rolling sums are lazy.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        exit: &Exit,
        compute_base: impl FnOnce(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()>
    where
        C: Default,
    {
        compute_base(&mut self.block)?;
        self.compute_rest(max_from, exit)
    }

    /// Compute cumulative from already-populated base data. Rolling sums are lazy.
    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()>
    where
        C: Default,
    {
        self.cumulative
            .height
            .compute_cumulative(max_from, &self.block, exit)?;
        Ok(())
    }
}
