//! RollingDelta - raw change + growth rate (%) across time windows.
//!
//! Three tiers:
//! - `RollingDelta1m` — 1m window only (2 stored vecs: change + rate). Default for all cohorts.
//! - `RollingDeltaExcept1m` — 24h + 1w + 1y windows (6 stored vecs). Extended tier only.
//! - `RollingDelta` — all 4 windows (8 stored vecs). Used for standalone global metrics.
//!
//! For a monotonic source (e.g., cumulative address count):
//! - `change._24h` = count_now - count_24h_ago
//! - `rate._24h` = (count_now - count_24h_ago) / count_24h_ago in BPS

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints32, BasisPointsSigned32, Height, Version};
use schemars::JsonSchema;
use vecdb::{AnyVec, Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode, VecIndex};

use crate::{
    indexes,
    internal::{
        ComputedFromHeight, NumericValue, PercentFromHeight, PercentRollingWindows,
        RollingWindows, WindowStarts,
    },
};

/// Pre-collect source data from the earliest needed offset.
/// Returns (source_data, offset) for use in compute_delta_window.
fn collect_source<S: NumericValue>(
    source: &impl ReadableVec<Height, S>,
    skip: usize,
    earliest_starts: &impl ReadableVec<Height, Height>,
) -> (Vec<S>, usize) {
    let source_len = source.len();
    let offset = if skip > 0 && skip < earliest_starts.len() {
        earliest_starts.collect_one_at(skip).unwrap().to_usize()
    } else {
        0
    };
    (source.collect_range_at(offset, source_len), offset)
}

/// Shared computation: change = current - ago, rate = change / ago.
fn compute_delta_window<S, C, B>(
    change_h: &mut EagerVec<PcoVec<Height, C>>,
    rate_bps_h: &mut EagerVec<PcoVec<Height, B>>,
    max_from: Height,
    starts: &impl ReadableVec<Height, Height>,
    source_data: &[S],
    offset: usize,
    exit: &Exit,
) -> Result<()>
where
    S: NumericValue,
    C: NumericValue,
    B: NumericValue,
{
    change_h.compute_transform(
        max_from,
        starts,
        |(h, ago_h, ..)| {
            let current: f64 = source_data[h.to_usize() - offset].into();
            let ago: f64 = source_data[ago_h.to_usize() - offset].into();
            (h, C::from(current - ago))
        },
        exit,
    )?;

    rate_bps_h.compute_transform(
        max_from,
        &*change_h,
        |(h, change, ..)| {
            let current_f: f64 = source_data[h.to_usize() - offset].into();
            let change_f: f64 = change.into();
            let ago = current_f - change_f;
            let rate = if ago == 0.0 { 0.0 } else { change_f / ago };
            (h, B::from(rate))
        },
        exit,
    )?;

    Ok(())
}

#[derive(Traversable)]
pub struct RollingDelta<S, C = S, M: StorageMode = Rw>
where
    S: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
{
    pub change: RollingWindows<C, M>,
    pub rate: PercentRollingWindows<BasisPoints32, M>,
    _phantom: std::marker::PhantomData<S>,
}

impl<S, C> RollingDelta<S, C>
where
    S: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            change: RollingWindows::forced_import(
                db,
                &format!("{name}_change"),
                version,
                indexes,
            )?,
            rate: PercentRollingWindows::forced_import(
                db,
                &format!("{name}_rate"),
                version,
                indexes,
            )?,
            _phantom: std::marker::PhantomData,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        source: &impl ReadableVec<Height, S>,
        exit: &Exit,
    ) -> Result<()> {
        // Pre-collect once using the widest window (1y has earliest ago heights)
        let skip = self.change.0._24h.height.len();
        let (source_data, offset) = collect_source(source, skip, windows._1y);

        for ((change_w, rate_w), starts) in self
            .change
            .0
            .as_mut_array()
            .into_iter()
            .zip(self.rate.0.as_mut_array())
            .zip(windows.as_array())
        {
            compute_delta_window(
                &mut change_w.height,
                &mut rate_w.bps.height,
                max_from,
                *starts,
                &source_data,
                offset,
                exit,
            )?;
        }
        Ok(())
    }
}

/// 1m-only delta: change + growth rate for the 1-month window.
/// Default tier for all cohorts (2 stored vecs).
#[derive(Traversable)]
pub struct RollingDelta1m<S, C = S, M: StorageMode = Rw>
where
    S: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
{
    pub change_1m: ComputedFromHeight<C, M>,
    pub rate_1m: PercentFromHeight<BasisPointsSigned32, M>,
    _phantom: std::marker::PhantomData<S>,
}

impl<S, C> RollingDelta1m<S, C>
where
    S: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            change_1m: ComputedFromHeight::forced_import(
                db,
                &format!("{name}_change_1m"),
                version,
                indexes,
            )?,
            rate_1m: PercentFromHeight::forced_import(
                db,
                &format!("{name}_rate_1m"),
                version,
                indexes,
            )?,
            _phantom: std::marker::PhantomData,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        height_1m_ago: &impl ReadableVec<Height, Height>,
        source: &impl ReadableVec<Height, S>,
        exit: &Exit,
    ) -> Result<()> {
        let skip = self.change_1m.height.len();
        let (source_data, offset) = collect_source(source, skip, height_1m_ago);

        compute_delta_window(
            &mut self.change_1m.height,
            &mut self.rate_1m.bps.height,
            max_from,
            height_1m_ago,
            &source_data,
            offset,
            exit,
        )
    }
}

/// Extended delta: 24h + 1w + 1y windows (6 stored vecs).
/// Only for All/LTH/STH cohorts (Extended tier).
#[derive(Traversable)]
pub struct RollingDeltaExcept1m<S, C = S, M: StorageMode = Rw>
where
    S: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
{
    #[traversable(rename = "24h")]
    pub change_24h: ComputedFromHeight<C, M>,
    pub change_1w: ComputedFromHeight<C, M>,
    pub change_1y: ComputedFromHeight<C, M>,
    #[traversable(rename = "24h")]
    pub rate_24h: PercentFromHeight<BasisPointsSigned32, M>,
    pub rate_1w: PercentFromHeight<BasisPointsSigned32, M>,
    pub rate_1y: PercentFromHeight<BasisPointsSigned32, M>,
    _phantom: std::marker::PhantomData<S>,
}

impl<S, C> RollingDeltaExcept1m<S, C>
where
    S: NumericValue + JsonSchema,
    C: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            change_24h: ComputedFromHeight::forced_import(
                db,
                &format!("{name}_change_24h"),
                version,
                indexes,
            )?,
            change_1w: ComputedFromHeight::forced_import(
                db,
                &format!("{name}_change_1w"),
                version,
                indexes,
            )?,
            change_1y: ComputedFromHeight::forced_import(
                db,
                &format!("{name}_change_1y"),
                version,
                indexes,
            )?,
            rate_24h: PercentFromHeight::forced_import(
                db,
                &format!("{name}_rate_24h"),
                version,
                indexes,
            )?,
            rate_1w: PercentFromHeight::forced_import(
                db,
                &format!("{name}_rate_1w"),
                version,
                indexes,
            )?,
            rate_1y: PercentFromHeight::forced_import(
                db,
                &format!("{name}_rate_1y"),
                version,
                indexes,
            )?,
            _phantom: std::marker::PhantomData,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        source: &impl ReadableVec<Height, S>,
        exit: &Exit,
    ) -> Result<()> {
        // Pre-collect once using the widest window (1y has earliest ago heights)
        let skip = self.change_24h.height.len();
        let (source_data, offset) = collect_source(source, skip, windows._1y);

        let changes = [&mut self.change_24h, &mut self.change_1w, &mut self.change_1y];
        let rates = [&mut self.rate_24h, &mut self.rate_1w, &mut self.rate_1y];
        let starts = [windows._24h, windows._1w, windows._1y];

        for ((change_w, rate_w), starts) in changes.into_iter().zip(rates).zip(starts) {
            compute_delta_window(
                &mut change_w.height,
                &mut rate_w.bps.height,
                max_from,
                starts,
                &source_data,
                offset,
                exit,
            )?;
        }
        Ok(())
    }
}
