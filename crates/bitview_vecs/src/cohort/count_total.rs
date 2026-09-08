use bitview_collections::Windows;
use bitview_transforms::RatioU64;
use bitview_traversable::Traversable;
use brk_types::{Height, PartsPerMillion32, StoredU64, Version};
use vecdb::{CachedBoxedVec, CachedReadableVec, CachedVec, LazyVec, ReadableCloneableVec};

use crate::{IndexSources, LazyPerBlockCumulativeRolling, LazyPercentCumulativeRolling};

/// Total-count views and the shared cumulative denominator used by their breakdowns.
#[derive(Clone, Traversable)]
pub struct CountTotal {
    pub all: LazyPerBlockCumulativeRolling<StoredU64>,
    #[traversable(skip)]
    denominator: CachedBoxedVec<Height, StoredU64>,
}

impl CountTotal {
    /// Reuse the owner's cached total as the denominator without another retained cache.
    pub fn from_source(
        name: &str,
        version: Version,
        source: CachedBoxedVec<Height, StoredU64>,
        indexes: &IndexSources,
        windows: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Self {
        let view = Self::views(name, version, &source, |_, value| value, indexes, windows);
        Self {
            all: view,
            denominator: source,
        }
    }

    /// Retain one transformed denominator, shared by every derived share.
    /// The caller selects the adjustment (for example, excluding coinbase transactions).
    pub fn from_transformed_source(
        name: &str,
        version: Version,
        source: &impl ReadableCloneableVec<Height, StoredU64>,
        transform: fn(Height, StoredU64) -> StoredU64,
        indexes: &IndexSources,
        windows: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Self {
        let all = Self::views(name, version, source, transform, indexes, windows);
        let denominator = CachedVec::wrap(all.cumulative.height.clone()).cached_boxed_clone();
        Self { all, denominator }
    }

    fn views(
        name: &str,
        version: Version,
        source: &impl ReadableCloneableVec<Height, StoredU64>,
        transform: fn(Height, StoredU64) -> StoredU64,
        indexes: &IndexSources,
        windows: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> LazyPerBlockCumulativeRolling<StoredU64> {
        let source = LazyVec::init(
            &format!("{name}_cumulative_source"),
            version,
            source.read_only_boxed_clone(),
            transform,
        );
        LazyPerBlockCumulativeRolling::from_cumulative_source(
            name, version, &source, windows, indexes,
        )
    }

    pub fn lazy_share(
        &self,
        name: &str,
        version: Version,
        numerator: &impl ReadableCloneableVec<Height, StoredU64>,
        windows: &Windows<&impl ReadableCloneableVec<Height, Height>>,
        indexes: &IndexSources,
    ) -> LazyPercentCumulativeRolling<PartsPerMillion32> {
        LazyPercentCumulativeRolling::from_cumulative_ratio::<
            StoredU64,
            StoredU64,
            RatioU64<PartsPerMillion32>,
        >(
            name,
            version,
            numerator,
            &self.denominator,
            windows,
            indexes,
        )
    }

    pub fn invalidate(&self) {
        self.denominator.invalidate();
    }
}
