use bitview_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use vecdb::{BinaryTransform, ReadableCloneableVec, UnaryTransform};

use crate::{
    CachedBlockCountReader, FixedRatio, IndexSources, LazyPercentPerBlock,
    LazyPercentRollingWindows, LazyRatioWithCachedBlockCount, LazyRollingRatioWithCachedBlockCount,
    NumericValue, RatioU64, Windows,
};

/// Fully lazy variant of `PercentCumulativeRolling` — no stored vecs.
///
/// Mirrors the flat shape of `PercentCumulativeRolling`: cumulative and
/// rolling window fields are both flattened to the same tree level, so
/// consumers see `{ ppm, percent, ratio, _24h, _1w, _1m, _1y }`.
#[derive(Clone, Traversable)]
pub struct LazyPercentCumulativeRolling<B: FixedRatio> {
    #[traversable(flatten)]
    pub cumulative: LazyPercentPerBlock<B>,
    #[traversable(flatten)]
    pub rolling: LazyPercentRollingWindows<B>,
}

impl<B: FixedRatio> LazyPercentCumulativeRolling<B> {
    pub fn from_cached_block_count(
        name: &str,
        version: Version,
        numerator: &impl ReadableCloneableVec<Height, StoredU64>,
        denominator: CachedBlockCountReader,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
        indexes: &IndexSources,
    ) -> Self {
        let source_name = format!("{name}_{}_source", B::SUFFIX);
        let numerator = numerator.read_only_boxed_clone();
        let source = LazyRatioWithCachedBlockCount::<B, RatioU64<B>>::new(
            &source_name,
            version,
            numerator.read_only_boxed_clone(),
            denominator.clone(),
        );
        let cumulative = LazyPercentPerBlock::from_height_source(name, version, &source, indexes);
        let rolling =
            LazyPercentRollingWindows(window_starts.map_with_suffix(|suffix, window_start| {
                let full_name = format!("{name}_{suffix}");
                let source = LazyRollingRatioWithCachedBlockCount::<B, RatioU64<B>>::new(
                    &format!("{full_name}_{}_source", B::SUFFIX),
                    version,
                    numerator.clone(),
                    denominator.clone(),
                    window_start.read_only_boxed_clone(),
                );
                LazyPercentPerBlock::from_height_source(&full_name, version, &source, indexes)
            }));

        Self {
            cumulative,
            rolling,
        }
    }

    /// Derive cumulative and rolling ratios from one potentially disk-backed
    /// cumulative numerator. The denominator and window starts are pinned
    /// metadata, so range reads touch only the numerator source.
    /// Pass its owning cache (or compact reader), shared with other consumers.
    pub fn from_cumulative_ratio<S, D, F>(
        name: &str,
        version: Version,
        numerator: &impl ReadableCloneableVec<Height, S>,
        denominator: &impl ReadableCloneableVec<Height, D>,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
        indexes: &IndexSources,
    ) -> Self
    where
        S: NumericValue,
        D: NumericValue,
        F: BinaryTransform<S, D, B> + Send + Sync + 'static,
    {
        let cumulative = LazyPercentPerBlock::from_ratio::<S, D, F>(
            name,
            version,
            numerator,
            denominator,
            indexes,
        );
        let rolling = LazyPercentRollingWindows::from_cumulative_ratio::<S, D, F>(
            name,
            version,
            numerator,
            denominator,
            window_starts,
            indexes,
        );
        Self {
            cumulative,
            rolling,
        }
    }

    /// Same ratio, with the pinned value as numerator and the sole
    /// potentially disk-backed source as denominator.
    pub fn from_cumulative_ratio_with_numerator<S, D, F>(
        name: &str,
        version: Version,
        numerator: &impl ReadableCloneableVec<Height, S>,
        denominator: &impl ReadableCloneableVec<Height, D>,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
        indexes: &IndexSources,
    ) -> Self
    where
        S: NumericValue,
        D: NumericValue,
        F: BinaryTransform<S, D, B> + Send + Sync + 'static,
    {
        let cumulative = LazyPercentPerBlock::from_ratio_with_numerator::<S, D, F>(
            name,
            version,
            numerator,
            denominator,
            indexes,
        );
        let rolling = LazyPercentRollingWindows::from_cumulative_ratio_with_numerator::<S, D, F>(
            name,
            version,
            numerator,
            denominator,
            window_starts,
            indexes,
        );
        Self {
            cumulative,
            rolling,
        }
    }

    pub fn from_lazy_source<F: UnaryTransform<B, B>>(
        name: &str,
        version: Version,
        source: &Self,
    ) -> Self {
        let cumulative =
            LazyPercentPerBlock::from_lazy_percent::<F>(name, version, &source.cumulative);
        let rolling =
            LazyPercentRollingWindows::from_lazy_rolling::<F>(name, version, &source.rolling);
        Self {
            cumulative,
            rolling,
        }
    }
}
