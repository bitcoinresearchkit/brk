use super::CachedVecStrategy;

/// Returns a fill's reserved bytes unless they become a resident snapshot.
pub(super) struct Reservation<'a, S: CachedVecStrategy> {
    strategy: &'a S,
    bytes: usize,
}

impl<'a, S: CachedVecStrategy> Reservation<'a, S> {
    pub(super) fn new(strategy: &'a S, bytes: usize) -> Option<Self> {
        if bytes > 0 && !strategy.try_reserve(bytes) {
            return None;
        }
        Some(Self { strategy, bytes })
    }

    pub(super) fn retain(mut self) {
        self.strategy.set_resident_bytes(self.bytes);
        self.bytes = 0;
    }
}

impl<S: CachedVecStrategy> Drop for Reservation<'_, S> {
    fn drop(&mut self) {
        if self.bytes > 0 {
            self.strategy.release(self.bytes);
        }
    }
}
