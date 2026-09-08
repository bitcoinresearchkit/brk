use super::{CachedVec, CachedVecStrategy};
use crate::TypedVec;

impl<V: TypedVec, S: CachedVecStrategy> TypedVec for CachedVec<V, S> {
    type I = V::I;
    type T = V::T;
}
