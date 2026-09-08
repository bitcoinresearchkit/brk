use crate::TypedVec;

use super::{CachedVec, CachedVecStrategy};

impl<V: TypedVec, S: CachedVecStrategy> TypedVec for CachedVec<V, S> {
    type I = V::I;
    type T = V::T;
}
