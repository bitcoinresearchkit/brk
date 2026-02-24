//! ComputedHeightDerivedCumFull - LazyLast index views + cumulative (from height) + RollingFull.
//!
//! For metrics derived from indexer sources (no stored height vec).
//! Cumulative gets its own ComputedFromHeightLast so it has LazyLast index views too.

use std::ops::SubAssign;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableBoxedVec, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        ComputedFromHeightLast, ComputedHeightDerivedLast, NumericValue, RollingFull, WindowStarts,
    },
};

#[derive(Traversable)]
#[traversable(merge)]
pub struct ComputedHeightDerivedCumFull<T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
{
    #[traversable(flatten)]
    pub last: ComputedHeightDerivedLast<T>,
    #[traversable(flatten)]
    pub cumulative: ComputedFromHeightLast<T, M>,
    #[traversable(flatten)]
    pub rolling: RollingFull<T, M>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedHeightDerivedCumFull<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        height_source: ReadableBoxedVec<Height, T>,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let last = ComputedHeightDerivedLast::forced_import(name, height_source, v, indexes);
        let cumulative = ComputedFromHeightLast::forced_import(
            db,
            &format!("{name}_cumulative"),
            v,
            indexes,
        )?;
        let rolling = RollingFull::forced_import(db, name, v, indexes)?;

        Ok(Self {
            last,
            cumulative,
            rolling,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        height_source: &impl ReadableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<f64> + Default + SubAssign + Copy + Ord,
        f64: From<T>,
    {
        self.cumulative
            .height
            .compute_cumulative(max_from, height_source, exit)?;
        self.rolling.compute(max_from, windows, height_source, exit)?;
        Ok(())
    }
}
