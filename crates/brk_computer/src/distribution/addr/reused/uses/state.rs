use brk_cohort::ByAddrType;
use derive_more::{Deref, DerefMut};

/// Per-block running counter of reused address uses, per address type.
/// Reset at the start of each block (no disk recovery needed since the
/// per-block flow is reconstructed from `process_received` deterministically).
#[derive(Debug, Default, Deref, DerefMut)]
pub struct AddrTypeToReusedAddrUseCount(ByAddrType<u64>);

impl AddrTypeToReusedAddrUseCount {
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
