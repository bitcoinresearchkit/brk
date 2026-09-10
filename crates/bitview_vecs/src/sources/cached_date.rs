use vecdb::PinnedCachedVec;

use crate::LazyDateVec;

/// Pinned date lookup derived from one monotonic source.
pub type CachedDateVec<I> = PinnedCachedVec<LazyDateVec<I>>;
