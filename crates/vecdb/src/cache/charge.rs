use std::ptr;

use super::CacheBudget;

/// One reservation, moved with its allocation and released exactly once.
#[derive(Debug)]
pub(super) struct Charge {
    budget: &'static CacheBudget,
    bytes: usize,
}

impl Charge {
    pub(super) fn new(budget: &'static CacheBudget, bytes: usize) -> Self {
        Self { budget, bytes }
    }

    pub(super) fn grow(&mut self, bytes: usize) -> bool {
        let Some(extra) = self.budget.reserve(bytes) else {
            return false;
        };
        self.merge(extra);
        true
    }

    pub(super) fn merge(&mut self, mut other: Self) {
        debug_assert!(ptr::eq(self.budget, other.budget));
        self.bytes = self
            .bytes
            .checked_add(other.bytes)
            .expect("cache charge overflow");
        other.bytes = 0;
    }

    pub(super) fn shrink_to(&mut self, bytes: usize) {
        debug_assert!(bytes <= self.bytes);
        self.budget.release(self.bytes - bytes);
        self.bytes = bytes;
    }
}

impl Drop for Charge {
    fn drop(&mut self) {
        self.budget.release(self.bytes);
    }
}
