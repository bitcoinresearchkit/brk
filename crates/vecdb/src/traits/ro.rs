use crate::{StorageMode, StoredVec};

/// Read-only mode: lean stored vecs and no writer-only computation state.
pub struct Ro;

impl StorageMode for Ro {
    type Stored<V: StoredVec + 'static> = V::ReadOnly;
    type WriteOnly<T> = ();
}
