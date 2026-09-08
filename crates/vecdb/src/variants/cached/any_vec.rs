use crate::{AnyVec, TypedVec, Version, short_type_name};

use super::{CachedVec, CachedVecStrategy};

impl<V: TypedVec, S: CachedVecStrategy> AnyVec for CachedVec<V, S> {
    #[inline(always)]
    fn version(&self) -> Version {
        self.inner.version()
    }

    #[inline(always)]
    fn name(&self) -> &str {
        self.inner.name()
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline(always)]
    fn index_type_to_string(&self) -> &'static str {
        self.inner.index_type_to_string()
    }

    #[inline(always)]
    fn region_names(&self) -> Vec<String> {
        self.inner.region_names()
    }

    #[inline(always)]
    fn value_type_to_size_of(&self) -> usize {
        size_of::<V::T>()
    }

    #[inline(always)]
    fn value_type_to_string(&self) -> &'static str {
        short_type_name::<V::T>()
    }
}
