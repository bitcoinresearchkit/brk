//! Stored cumulative source and rolling statistics for externally supplied block values.

use brk_error::Result;

use bitview_traversable::Traversable;
use brk_exit::Exit;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{
    Budgeted, Database, ReadOnlyClone, ReadableCloneableVec, ReadableVec, Rw, StorageMode,
};

use crate::{
    CachePolicy, IndexSources, NumericValue, PerBlock, RollingComplete, WindowStarts, Windows,
};

#[derive(Traversable)]
pub struct PerBlockRolling<T, M: StorageMode = Rw, S: CachePolicy = Budgeted>
where
    T: NumericValue + JsonSchema,
{
    /// Cumulative value through the represented block. At time-period indexes,
    /// the value is taken at the period's final block.
    pub cumulative: PerBlock<T, M, S>,
    #[traversable(flatten)]
    pub rolling: RollingComplete<T, M>,
}

impl<T, S: CachePolicy> PerBlockRolling<T, Rw, S>
where
    T: NumericValue + JsonSchema,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Result<Self> {
        let cumulative =
            PerBlock::forced_import(db, &format!("{name}_cumulative"), version, indexes)?;
        let cumulative_source = cumulative.height.read_only_clone();
        let rolling = RollingComplete::forced_import(
            db,
            name,
            version,
            indexes,
            &cumulative_source,
            window_starts,
        )?;

        Ok(Self {
            cumulative,
            rolling,
        })
    }

    pub fn cumulative_source(
        &self,
    ) -> &(impl ReadableVec<Height, T> + Clone + 'static + use<T, S>) {
        self.cumulative.resolutions.height_source()
    }

    pub fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        height_source: &impl ReadableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<f64> + Default + Copy + Ord,
        f64: From<T>,
    {
        self.cumulative
            .height
            .compute_cumulative(max_from, height_source, exit)?;
        self.rolling
            .compute(max_from, windows, height_source, exit)?;
        Ok(())
    }
}
