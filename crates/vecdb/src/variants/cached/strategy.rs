use super::CachedVecBudget;

/// Per-cache policy and accounting, shared by all clones of a cached vector.
pub trait CachedVecStrategy: CachedVecBudget + Clone + 'static {
    fn set_resident_bytes(&self, bytes: usize);
    fn take_resident_bytes(&self) -> usize;
}
