use super::Publication;
use parking_lot::{ArcRwLockReadGuard, RawRwLock};
use std::time::Duration;

/// Keeps the complete pipeline's mutable state stable for one logical read.
pub struct PublicationReadGuard {
    _guard: ArcRwLockReadGuard<RawRwLock, ()>,
}

impl Publication {
    /// Attempts to stabilize the pipeline without waiting.
    pub fn try_read(&self) -> Option<PublicationReadGuard> {
        Some(PublicationReadGuard {
            _guard: self.0.gate.try_read_arc()?,
        })
    }

    /// Waits up to `timeout` on a blocking worker.
    pub fn read_for(&self, timeout: Duration) -> Option<PublicationReadGuard> {
        Some(PublicationReadGuard {
            _guard: self.0.gate.try_read_arc_for(timeout)?,
        })
    }
}
