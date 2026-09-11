use bitview_collections::Windows;
use bitview_compute::NumericValue;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_exit::Exit;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{
    Budgeted, CacheBudget, CachePolicy, Database, EagerVec, ImportOptions, ImportableVec, PcoVec,
    ReadableCloneableVec, Rw, StorageMode,
};

use crate::{IndexSources, LazyRollingAvgsFromHeight};

/// Stored-block fallback for values whose cumulative delta is not exact, such as floats.
#[derive(Traversable)]
pub struct PerBlockRollingAverage<T, C = T, M: StorageMode = Rw, P: CachePolicy = Budgeted>
where
    T: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
{
    /// Value for the represented block. At time-period indexes, the value is
    /// taken from the period's final block.
    pub block: M::Stored<EagerVec<PcoVec<Height, T, P>>>,
    #[traversable(hidden)]
    cumulative: M::Stored<EagerVec<PcoVec<Height, C, P>>>,
    #[traversable(flatten)]
    pub average: LazyRollingAvgsFromHeight<C>,
}

impl<T, C, P: CachePolicy> PerBlockRollingAverage<T, C, Rw, P>
where
    T: NumericValue + JsonSchema + Into<C>,
    C: NumericValue + JsonSchema,
{
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Result<Self> {
        let block = EagerVec::<PcoVec<Height, T, P>>::forced_import_with(
            ImportOptions::new(db, name, version).with_cache_budget(cache),
        )?;
        let cumulative = EagerVec::<PcoVec<Height, C, P>>::forced_import_with(
            ImportOptions::new(db, &format!("{name}_cumulative"), version + Version::TWO)
                .with_cache_budget(cache),
        )?;
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
