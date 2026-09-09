use vecdb::DeltaSub;

use crate::LazyDeltaFromHeight;

/// A lazy rolling sum from height and its resolution views.
pub type LazyRollingSumFromHeight<T> = LazyDeltaFromHeight<T, T, DeltaSub>;
