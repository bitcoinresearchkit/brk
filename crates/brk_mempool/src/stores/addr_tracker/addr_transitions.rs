//! Per-cycle 0↔1+ address transition buffer.
//!
//! Lives on the stack inside [`crate::Mempool::tick_with`], not on a
//! long-lived store, so the set naturally resets between cycles.
//! Same-cycle cancellation (enter→leave, leave→enter, and the 3-step
//! enter→leave→enter / leave→enter→leave variants) is encapsulated on
//! the recording methods so callers just announce raw 0↔1+ flips.

use brk_types::AddrBytes;
use rustc_hash::FxHashSet;

#[derive(Default)]
pub struct AddrTransitions {
    enters: FxHashSet<AddrBytes>,
    leaves: FxHashSet<AddrBytes>,
}

impl AddrTransitions {
    /// Address just went 0 → 1+ live mempool txs. Cancels a pending
    /// `leave` for the same address in this cycle.
    pub fn record_enter(&mut self, bytes: AddrBytes) {
        if !self.leaves.remove(&bytes) {
            self.enters.insert(bytes);
        }
    }

    /// Address just went 1+ → 0 live mempool txs. Cancels a pending
    /// `enter` for the same address in this cycle.
    pub fn record_leave(&mut self, bytes: AddrBytes) {
        if !self.enters.remove(&bytes) {
            self.leaves.insert(bytes);
        }
    }

    pub fn into_vecs(self) -> (Vec<AddrBytes>, Vec<AddrBytes>) {
        (
            self.enters.into_iter().collect(),
            self.leaves.into_iter().collect(),
        )
    }
}
