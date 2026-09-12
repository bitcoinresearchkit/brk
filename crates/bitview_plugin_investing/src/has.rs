use vecdb::{Rw, StorageMode};

use crate::Vecs;

/// Provides access to the investing plugin.
pub trait HasInvesting<M: StorageMode = Rw> {
    fn investing(&self) -> &Vecs<M>;
}
