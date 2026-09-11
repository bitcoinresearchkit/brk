use crate::{
    AnyVec, VecIndex, VecValue, Version, cache::CachePolicy, short_type_name, vec_region_name,
};

use super::{super::CompressionStrategy, ReadOnlyCompressedVec};

impl<I, T, S, C: CachePolicy> AnyVec for ReadOnlyCompressedVec<I, T, S, C>
where
    I: VecIndex,
    T: VecValue,
    S: CompressionStrategy<T>,
{
    #[inline]
    fn version(&self) -> Version {
        self.base.version()
    }

    #[inline]
    fn name(&self) -> &str {
        self.base.name()
    }

    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }

    #[inline]
    fn index_type_to_string(&self) -> &'static str {
        I::to_string()
    }

    #[inline]
    fn value_type_to_size_of(&self) -> usize {
        size_of::<T>()
    }

    #[inline]
    fn value_type_to_string(&self) -> &'static str {
        short_type_name::<T>()
    }

    #[inline]
    fn region_names(&self) -> Vec<String> {
        let base = vec_region_name(self.base.name(), I::to_string());
        let pages = format!("{base}_pages");
        vec![base, pages]
    }
}
