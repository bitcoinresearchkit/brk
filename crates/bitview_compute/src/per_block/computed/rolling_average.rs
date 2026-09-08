use brk_error::Result;

use bitview_traversable::Traversable;
use brk_exit::Exit;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{
    Budgeted, CachedVec, Database, EagerVec, ImportableVec, PcoVec, ReadableCloneableVec, Rw,
    StorageMode,
};

use crate::{CachePolicy, IndexSources, LazyRollingAvgsFromHeight, NumericValue, Windows};

/// Stored-block fallback for values whose cumulative delta is not exact, such as floats.
#[derive(Traversable)]
pub struct PerBlockRollingAverage<T, C = T, M: StorageMode = Rw, P: CachePolicy = Budgeted>
where
    T: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
{
    /// Value for the represented block. At time-period indexes, the value is
    /// taken from the period's final block.
    pub block: CachedVec<M::Stored<EagerVec<PcoVec<Height, T>>>, P>,
    #[traversable(hidden)]
    cumulative: CachedVec<M::Stored<EagerVec<PcoVec<Height, C>>>, P>,
    #[traversable(flatten)]
    pub average: LazyRollingAvgsFromHeight<C>,
}

impl<T, C, P: CachePolicy> PerBlockRollingAverage<T, C, Rw, P>
where
    T: NumericValue + JsonSchema + Into<C>,
    C: NumericValue + JsonSchema,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Result<Self> {
        let block = P::wrap(EagerVec::<PcoVec<Height, T>>::forced_import(
            db, name, version,
        )?);
        let cumulative = P::wrap(EagerVec::<PcoVec<Height, C>>::forced_import(
            db,
            &format!("{name}_cumulative"),
            version + Version::TWO,
        )?);
        let average = LazyRollingAvgsFromHeight::new(
            &format!("{name}_average"),
            version + Version::TWO,
            &cumulative.read_only_boxed_clone(),
            window_starts,
            indexes,
        );

        Ok(Self {
            block,
            cumulative,
            average,
        })
    }

    /// Compute cumulative from already-populated height data. Rolling averages are lazy.
    pub fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()> {
        self.cumulative
            .compute_cumulative(max_from, &self.block, exit)?;
        Ok(())
    }
}
