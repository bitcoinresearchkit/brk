use super::{CachedVec, CachedVecStrategy};
use crate::{EagerVec, ReadableBoxedVec, ReadableCloneableVec, StoredVec};

impl<V: StoredVec, S: CachedVecStrategy> ReadableCloneableVec<V::I, V::T>
    for CachedVec<EagerVec<V>, S>
{
    fn read_only_boxed_clone(&self) -> ReadableBoxedVec<V::I, V::T> {
        CachedVec::read_only_boxed_clone(self)
    }
}
