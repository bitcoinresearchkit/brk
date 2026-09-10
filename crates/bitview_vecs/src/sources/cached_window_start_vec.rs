use vecdb::PinnedCachedVec;

use crate::LazyWindowStartVec;

/// A pinned, process-lifetime cache for a heavily reused window-start vector.
pub type CachedWindowStartVec = PinnedCachedVec<LazyWindowStartVec>;
