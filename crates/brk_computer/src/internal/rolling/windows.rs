//! RollingWindows - newtype on Windows with ComputedFromHeight per window duration.
//!
//! Each of the 4 windows (24h, 1w, 1m, 1y) contains a height-level stored vec
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

/// Rolling window start heights — the 4 height-ago vecs (24h, 1w, 1m, 1y).
pub type WindowStarts<'a> = Windows<&'a EagerVec<PcoVec<Height, Height>>>;

/// 4 rolling window vecs (24h, 1w, 1m, 1y), each with height data + all 17 index views.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingWindows<T, M: StorageMode = Rw>(pub Windows<ComputedFromHeight<T, M>>)
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
            ComputedFromHeight::forced_import(db, &format!("{name}_{suffix}"), version, indexes)
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
            w.height
                .compute_rolling_sum(max_from, *starts, source, exit)?;
        }
        Ok(())
    }
}

/// Single 24h rolling window (1 stored vec).
#[derive(Traversable)]
pub struct RollingWindow24h<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    #[traversable(rename = "24h")]
    pub _24h: ComputedFromHeight<T, M>,
}

impl<T> RollingWindow24h<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            _24h: ComputedFromHeight::forced_import(
                db,
                &format!("{name}_24h"),
                version,
                indexes,
            )?,
        })
    }

    pub(crate) fn compute_rolling_sum(
        &mut self,
        max_from: Height,
        height_24h_ago: &impl ReadableVec<Height, Height>,
        source: &impl ReadableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Default + SubAssign,
    {
        self._24h
            .height
            .compute_rolling_sum(max_from, height_24h_ago, source, exit)?;
        Ok(())
    }
}

/// Extended rolling windows: 1w + 1m + 1y (3 stored vecs).
#[derive(Traversable)]
pub struct RollingWindowsFrom1w<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    #[traversable(rename = "1w")]
    pub _1w: ComputedFromHeight<T, M>,
    #[traversable(rename = "1m")]
    pub _1m: ComputedFromHeight<T, M>,
    #[traversable(rename = "1y")]
    pub _1y: ComputedFromHeight<T, M>,
}

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
        Ok(Self {
            _1w: ComputedFromHeight::forced_import(
                db,
                &format!("{name}_1w"),
                version,
                indexes,
            )?,
            _1m: ComputedFromHeight::forced_import(
                db,
                &format!("{name}_1m"),
                version,
                indexes,
            )?,
            _1y: ComputedFromHeight::forced_import(
                db,
                &format!("{name}_1y"),
                version,
                indexes,
            )?,
        })
    }

    pub fn as_array(&self) -> [&ComputedFromHeight<T>; 3] {
        [&self._1w, &self._1m, &self._1y]
    }

    pub fn as_mut_array(&mut self) -> [&mut ComputedFromHeight<T>; 3] {
        [&mut self._1w, &mut self._1m, &mut self._1y]
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
        self._1w
            .height
            .compute_rolling_sum(max_from, windows._1w, source, exit)?;
        self._1m
            .height
            .compute_rolling_sum(max_from, windows._1m, source, exit)?;
        self._1y
            .height
            .compute_rolling_sum(max_from, windows._1y, source, exit)?;
        Ok(())
    }
}
