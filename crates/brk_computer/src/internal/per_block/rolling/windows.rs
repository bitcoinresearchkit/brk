//! RollingWindows - newtype on Windows with ComputedPerBlock per window duration.
//!
//! Each of the 4 windows (24h, 1w, 1m, 1y) contains a height-level stored vec
//! plus all 17 LazyAggVec index views.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, PcoVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedPerBlock, ComputedVecValue, NumericValue, RollingWindow24h, Windows, WindowsFrom1w},
};

/// Rolling window start heights — the 4 height-ago vecs (24h, 1w, 1m, 1y).
pub type WindowStarts<'a> = Windows<&'a EagerVec<PcoVec<Height, Height>>>;

/// 4 rolling window vecs (24h, 1w, 1m, 1y), each with height data + all 17 index views.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingWindows<T, M: StorageMode = Rw>(pub Windows<ComputedPerBlock<T, M>>)
where
    T: ComputedVecValue + PartialOrd + JsonSchema;

impl<T> RollingWindows<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(Windows::try_from_fn(|suffix| {
            ComputedPerBlock::forced_import(db, &format!("{name}_{suffix}"), version, indexes)
        })?))
    }
}

/// Single 24h rolling window backed by ComputedPerBlock (1 stored vec).
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingWindow24hPerBlock<T, M: StorageMode = Rw>(
    pub RollingWindow24h<ComputedPerBlock<T, M>>,
)
where
    T: ComputedVecValue + PartialOrd + JsonSchema;

impl<T> RollingWindow24hPerBlock<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(RollingWindow24h {
            _24h: ComputedPerBlock::forced_import(
                db,
                &format!("{name}_24h"),
                version,
                indexes,
            )?,
        }))
    }
}

/// Extended rolling windows: 1w + 1m + 1y (3 stored vecs).
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingWindowsFrom1w<T, M: StorageMode = Rw>(pub WindowsFrom1w<ComputedPerBlock<T, M>>)
where
    T: ComputedVecValue + PartialOrd + JsonSchema;

impl<T> RollingWindowsFrom1w<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(WindowsFrom1w::try_from_fn(|suffix| {
            ComputedPerBlock::forced_import(db, &format!("{name}_{suffix}"), version, indexes)
        })?))
    }
}
