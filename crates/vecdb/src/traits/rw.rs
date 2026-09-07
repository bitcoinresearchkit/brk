use crate::{StorageMode, StoredVec};

/// Read-write mode: full stored vecs and computation state.
pub struct Rw;

impl StorageMode for Rw {
    type Stored<V: StoredVec + 'static> = V;
    type WriteOnly<T> = T;
}
