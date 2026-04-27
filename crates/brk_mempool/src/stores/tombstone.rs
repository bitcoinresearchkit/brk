use std::time::{Duration, Instant};

use brk_types::Transaction;

use super::Entry;
use crate::steps::preparer::Removal;

/// A buried mempool tx, retained for reappearance detection and
/// post-mine analytics.
pub struct Tombstone {
    pub tx: Transaction,
    pub entry: Entry,
    removal: Removal,
    removed_at: Instant,
}

impl Tombstone {
    pub(super) fn new(
        tx: Transaction,
        entry: Entry,
        removal: Removal,
        removed_at: Instant,
    ) -> Self {
        Self {
            tx,
            entry,
            removal,
            removed_at,
        }
    }

    pub fn reason(&self) -> &Removal {
        &self.removal
    }

    pub fn age(&self) -> Duration {
        self.removed_at.elapsed()
    }

    pub(super) fn removed_at(&self) -> Instant {
        self.removed_at
    }

    pub(super) fn replaced_by(&self) -> Option<&brk_types::Txid> {
        match &self.removal {
            Removal::Replaced { by } => Some(by),
            Removal::Vanished => None,
        }
    }
}
