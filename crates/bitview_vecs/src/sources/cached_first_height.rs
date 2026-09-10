use vecdb::PinnedCachedVec;

use crate::LazyFirstHeightVec;

/// Pinned first-height lookup derived from one monotonic source.
pub type CachedFirstHeightVec<I> = PinnedCachedVec<LazyFirstHeightVec<I>>;
