use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{DeltaAvg, LazyDeltaVec, ReadableCloneableVec};

use crate::{
    indexes,
    internal::{CachedWindowStarts, NumericValue, Resolutions, Windows},
};

use super::LazyRollingAvgFromHeight;

/// Lazy rolling averages for all 4 window durations (24h, 1w, 1m, 1y),
/// derived from a cumulative vec + cached window starts.
///
/// Nothing is stored on disk — all values are computed on-the-fly via
/// `LazyDeltaVec<Height, T, T, DeltaAvg>`: `(cum[h] - cum[start-1]) / (h - start + 1)`.
/// T is converted to f64 internally for division, then back to T.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyRollingAvgsFromHeight<T>(pub Windows<LazyRollingAvgFromHeight<T>>)
where
    T: NumericValue + JsonSchema;

impl<T> LazyRollingAvgsFromHeight<T>
where
    T: NumericValue + JsonSchema,
{
    pub fn new(
        name: &str,
        version: Version,
        cumulative: &(impl ReadableCloneableVec<Height, T> + 'static),
        cached_starts: &CachedWindowStarts,
        indexes: &indexes::Vecs,
    ) -> Self {
        let cum_source = cumulative.read_only_boxed_clone();

        Self(cached_starts.0.map_with_suffix(|suffix, cached_start| {
            let full_name = format!("{name}_{suffix}");
            let cached = cached_start.clone();
            let starts_version = cached.version();
            let avg = LazyDeltaVec::<Height, T, T, DeltaAvg>::new(
                &full_name,
                version,
                cum_source.clone(),
                starts_version,
                move || cached.get(),
            );
            let resolutions = Resolutions::forced_import(
                &full_name,
                avg.read_only_boxed_clone(),
                version,
                indexes,
            );
            LazyRollingAvgFromHeight {
                height: avg,
                resolutions: Box::new(resolutions),
            }
        }))
    }
}
