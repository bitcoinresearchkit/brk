//! RollingWindows - newtype on Windows with ComputedFromHeight per window duration.
//!
//! Each of the 4 windows (24h, 7d, 30d, 1y) contains a height-level stored vec
//! plus all 17 LazyAggVec index views.

use std::ops::SubAssign;

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedFromHeight, ComputedVecValue, NumericValue, Windows},
};

/// Rolling window start heights — the 4 height-ago vecs (24h, 7d, 30d, 1y).
pub type WindowStarts<'a> = Windows<&'a EagerVec<PcoVec<Height, Height>>>;

/// 4 rolling window vecs (24h, 7d, 30d, 1y), each with height data + all 17 index views.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingWindows<T, M: StorageMode = Rw>(pub Windows<ComputedFromHeight<T, M>>)
where
    T: ComputedVecValue + PartialOrd + JsonSchema;

const VERSION: Version = Version::ZERO;

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
        let v = version + VERSION;
        Ok(Self(Windows::try_from_fn(|suffix| {
            ComputedFromHeight::forced_import(db, &format!("{name}_{suffix}"), v, indexes)
        })?))
    }

    pub(crate) fn compute_rolling_sum(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        source: &impl ReadableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Default + SubAssign,
    {
        for (w, starts) in self.0.as_mut_array().into_iter().zip(windows.as_array()) {
            w.height.compute_rolling_sum(max_from, *starts, source, exit)?;
        }
        Ok(())
    }
}
