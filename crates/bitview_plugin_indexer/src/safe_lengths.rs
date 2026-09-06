use brk_types::Lengths;
use parking_lot::{ArcRwLockReadGuard, RawRwLock};

use crate::state::State;

/// Pins a published immutable index prefix without waiting for append/compute.
/// Rollback must lower the shared lengths before changing that prefix, so it
/// waits for this guard. Mutable aggregates are not protected by these bounds.
pub struct SafeLengths(ArcRwLockReadGuard<RawRwLock, Lengths>);

impl SafeLengths {
    pub fn lengths(&self) -> Lengths {
        *self.0
    }
}

impl State {
    pub fn pin(&self) -> SafeLengths {
        SafeLengths(self.0.read_arc())
    }

    pub fn try_pin(&self) -> Option<SafeLengths> {
        self.0.try_read_arc().map(SafeLengths)
    }
}
