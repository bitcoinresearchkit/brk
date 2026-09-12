use std::sync::Arc;

use super::Account;

/// One reservation, moved with its allocation and released exactly once.
#[derive(Debug)]
pub(super) struct Charge {
    account: Arc<Account>,
    bytes: usize,
}

impl Charge {
    pub(super) fn new(account: Arc<Account>, bytes: usize) -> Self {
        Self { account, bytes }
    }

    pub(super) fn grow(&mut self, bytes: usize) -> bool {
        let Some(extra) = self.account.reserve(bytes) else {
            return false;
        };
        self.merge(extra);
        true
    }

    pub(super) fn merge(&mut self, mut other: Self) {
        debug_assert!(Arc::ptr_eq(&self.account, &other.account));
        self.bytes = self
            .bytes
            .checked_add(other.bytes)
            .expect("cache charge overflow");
        other.bytes = 0;
    }
}

impl Drop for Charge {
    fn drop(&mut self) {
        self.account.release(self.bytes);
    }
}
