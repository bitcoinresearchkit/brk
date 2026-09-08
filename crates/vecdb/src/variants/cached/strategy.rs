use super::{CacheBudget, CachedVec, CachedVecBudget};
use crate::TypedVec;

/// Per-cache policy and accounting, shared by all clones of a cached vector.
pub trait CachedVecStrategy: CachedVecBudget + Clone + 'static {
    fn wrap<V: TypedVec>(source: V, budget: &'static CacheBudget) -> CachedVec<V, Self>;
    fn set_resident_bytes(&self, bytes: usize);
    fn take_resident_bytes(&self) -> usize;
}
