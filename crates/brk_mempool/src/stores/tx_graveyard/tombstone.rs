use std::time::{Duration, Instant};

use brk_types::{Transaction, Txid};

use crate::{TxEntry, TxRemoval};

/// A buried mempool tx, retained for reappearance detection and
/// post-mine analytics.
pub struct TxTombstone {
    pub tx: Transaction,
    pub entry: TxEntry,
    removal: TxRemoval,
    removed_at: Instant,
}

impl TxTombstone {
    pub(crate) fn new(
        tx: Transaction,
        entry: TxEntry,
        removal: TxRemoval,
        removed_at: Instant,
    ) -> Self {
        Self {
            tx,
            entry,
            removal,
            removed_at,
        }
    }

    pub fn reason(&self) -> &TxRemoval {
        &self.removal
    }

    pub fn age(&self) -> Duration {
        self.removed_at.elapsed()
    }

    pub(crate) fn removed_at(&self) -> Instant {
        self.removed_at
    }

    pub(crate) fn replaced_by(&self) -> Option<&Txid> {
        match &self.removal {
            TxRemoval::Replaced { by } => Some(by),
            TxRemoval::Vanished => None,
        }
    }
}
