use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::ReadableCloneableVec;

use crate::{CachedWindowStartVec, NumericValue, Windows};

use super::LazyRollingSumFromHeight;

/// Lazy rolling sums for all 4 window durations (24h, 1w, 1m, 1y),
/// derived from a cumulative vec + cached window starts.
///
/// Nothing is stored on disk — all values are computed on-the-fly via
/// `LazyDeltaVec<Height, T, T, DeltaSub>`: `cum[h] - cum[window_start[h] - 1]`,
/// using zero when the window starts at genesis.
///
/// Implements `Traversable` to expose `_24h`, `_1w`, `_1m`, `_1y` with
/// the same tree structure as the old `RollingWindows<T>`.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyRollingSumsFromHeight<T>(
    /// Total of the per-block values over the trailing window ending at the
    /// represented block. At time-period indexes, the value is taken at the
    /// period's final block.
    pub Windows<LazyRollingSumFromHeight<T>>,
)
where
    T: NumericValue + JsonSchema;

impl<T> LazyRollingSumsFromHeight<T>
where
    T: NumericValue + JsonSchema,
{
    pub fn new(
        name: &str,
        version: Version,
        cumulative: &(impl ReadableCloneableVec<Height, T> + 'static),
        cached_starts: &Windows<&CachedWindowStartVec>,
        indexes: &crate::IndexSources,
    ) -> Self {
        let cum_source = cumulative.read_only_boxed_clone();

        Self(cached_starts.map_with_suffix(|suffix, cached_start| {
            LazyRollingSumFromHeight::new(
                &format!("{name}_{suffix}"),
                version,
                cum_source.clone(),
                cached_start,
                indexes,
            )
        }))
    }

    /// Build rolling sums from compact in-memory cumulative state, where the
    /// derived full-height histories are cheap to recompute.
    pub fn from_compact_cumulative(
        name: &str,
        version: Version,
        cumulative: &(impl ReadableCloneableVec<Height, T> + 'static),
        cached_starts: &Windows<&CachedWindowStartVec>,
        indexes: &crate::IndexSources,
    ) -> Self {
        Self::new(name, version, cumulative, cached_starts, indexes)
    }
}
