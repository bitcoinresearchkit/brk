//! RollingWindows - newtype on Windows with ComputedFromHeightLast per window duration.
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
    internal::{ComputedFromHeightLast, ComputedVecValue, NumericValue, Windows},
};

/// Rolling window start heights — references to the 4 height-ago vecs.
pub struct WindowStarts<'a> {
    pub _24h: &'a EagerVec<PcoVec<Height, Height>>,
    pub _7d: &'a EagerVec<PcoVec<Height, Height>>,
    pub _30d: &'a EagerVec<PcoVec<Height, Height>>,
    pub _1y: &'a EagerVec<PcoVec<Height, Height>>,
}

impl<'a> WindowStarts<'a> {
    pub fn as_array(&self) -> [&'a EagerVec<PcoVec<Height, Height>>; 4] {
        [self._24h, self._7d, self._30d, self._1y]
    }
}

/// 4 rolling window vecs (24h, 7d, 30d, 1y), each with height data + all 17 index views.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingWindows<T, M: StorageMode = Rw>(pub Windows<ComputedFromHeightLast<T, M>>)
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
        Ok(Self(Windows {
            _24h: ComputedFromHeightLast::forced_import(db, &format!("{name}_24h"), v, indexes)?,
            _7d: ComputedFromHeightLast::forced_import(db, &format!("{name}_7d"), v, indexes)?,
            _30d: ComputedFromHeightLast::forced_import(db, &format!("{name}_30d"), v, indexes)?,
            _1y: ComputedFromHeightLast::forced_import(db, &format!("{name}_1y"), v, indexes)?,
        }))
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
            w.height.compute_rolling_sum(max_from, starts, source, exit)?;
        }
        Ok(())
    }
}
