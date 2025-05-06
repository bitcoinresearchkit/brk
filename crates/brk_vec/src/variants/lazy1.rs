use std::marker::PhantomData;

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BaseVecIterator, BoxedAnyIterableVec,
    BoxedVecIterator, CollectableVec, Result, StoredIndex, StoredType, Value, Version,
};

pub type ComputeFrom1<I, T, S1I, S1T> =
    for<'a> fn(I, &mut dyn BaseVecIterator<Item = (S1I, Value<'a, S1T>)>) -> Option<T>;

#[derive(Clone)]
pub struct LazyVecFrom1<I, T, S1I, S1T> {
    name: String,
    version: Version,
    source: BoxedAnyIterableVec<S1I, S1T>,
    compute: ComputeFrom1<I, T, S1I, S1T>,
    phantom: PhantomData<I>,
}

impl<I, T, S1I, S1T> LazyVecFrom1<I, T, S1I, S1T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
{
    pub fn init(
        name: &str,
        version: Version,
        source: BoxedAnyIterableVec<S1I, S1T>,
        compute: ComputeFrom1<I, T, S1I, S1T>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            version,
            source,
            compute,
            phantom: PhantomData,
        }
    }

    fn version(&self) -> Version {
        self.version
    }
}

pub struct LazyVecFrom1Iterator<'a, I, T, S1I, S1T> {
    lazy: &'a LazyVecFrom1<I, T, S1I, S1T>,
    source: BoxedVecIterator<'a, S1I, S1T>,
    index: usize,
}

impl<'a, I, T, S1I, S1T> Iterator for LazyVecFrom1Iterator<'a, I, T, S1I, S1T>
where
    I: StoredIndex,
    T: StoredType + 'a,
    S1I: StoredIndex,
    S1T: StoredType,
{
    type Item = (I, Value<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len() {
            return None;
        }
        let index = I::from(self.index);
        let opt = (self.lazy.compute)(index, &mut *self.source).map(|v| (index, Value::Owned(v)));
        if opt.is_some() {
            self.index += 1;
        }
        opt
    }
}

impl<I, T, S1I, S1T> BaseVecIterator for LazyVecFrom1Iterator<'_, I, T, S1I, S1T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
{
    #[inline]
    fn mut_index(&mut self) -> &mut usize {
        &mut self.index
    }

    #[inline]
    fn len(&self) -> usize {
        self.source.len()
    }
}

impl<'a, I, T, S1I, S1T> IntoIterator for &'a LazyVecFrom1<I, T, S1I, S1T>
where
    I: StoredIndex,
    T: StoredType + 'a,
    S1I: StoredIndex,
    S1T: StoredType,
{
    type Item = (I, Value<'a, T>);
    type IntoIter = LazyVecFrom1Iterator<'a, I, T, S1I, S1T>;

    fn into_iter(self) -> Self::IntoIter {
        LazyVecFrom1Iterator {
            lazy: self,
            source: self.source.iter(),
            index: 0,
        }
    }
}

impl<I, T, S1I, S1T> AnyVec for LazyVecFrom1<I, T, S1I, S1T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
{
    fn version(&self) -> Version {
        self.version()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn index_type_to_string(&self) -> &str {
        I::to_string()
    }

    fn len(&self) -> usize {
        self.source.len()
    }

    fn modified_time(&self) -> Result<std::time::Duration> {
        self.source.modified_time()
    }
}

impl<I, T, S1I, S1T> AnyIterableVec<I, T> for LazyVecFrom1<I, T, S1I, S1T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
{
    fn boxed_iter<'a>(&'a self) -> BoxedVecIterator<'a, I, T>
    where
        T: 'a,
    {
        Box::new(self.into_iter())
    }
}

impl<I, T, S1I, S1T> AnyCollectableVec for LazyVecFrom1<I, T, S1I, S1T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
{
    fn collect_range_serde_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
