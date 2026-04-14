use brk_cohort::ByAddrType;
use derive_more::{Deref, DerefMut};

/// Per-block running counter of reused-address events, per address type.
/// Shared runtime container for both output-side events
/// (`output_to_reused_addr_count`, outputs landing on addresses that
/// had already received ≥ 1 prior output) and input-side events
/// (`input_from_reused_addr_count`, inputs spending from addresses
/// with lifetime `funded_txo_count > 1`). Reset at the start of each
/// block (no disk recovery needed since per-block flow is
/// reconstructed deterministically from `process_received` /
/// `process_sent`).
#[derive(Debug, Default, Deref, DerefMut)]
pub struct AddrTypeToReusedAddrEventCount(ByAddrType<u64>);

impl AddrTypeToReusedAddrEventCount {
    #[inline]
    pub(crate) fn sum(&self) -> u64 {
        self.0.values().sum()
    }

    #[inline]
    pub(crate) fn reset(&mut self) {
        for v in self.0.values_mut() {
            *v = 0;
        }
    }
}
