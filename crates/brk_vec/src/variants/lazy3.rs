use std::{marker::PhantomData, path::Path};

use brk_core::{Result, Value, Version};

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BaseVecIterator, BoxedAnyIterableVec,
    BoxedVecIterator, CollectableVec, StoredIndex, StoredType,
};

pub type ComputeFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T> = for<'a> fn(
    I,
    &mut dyn BaseVecIterator<Item = (S1I, Value<'a, S1T>)>,
    &mut dyn BaseVecIterator<Item = (S2I, Value<'a, S2T>)>,
    &mut dyn BaseVecIterator<Item = (S3I, Value<'a, S3T>)>,
) -> Option<T>;

#[derive(Clone)]
pub struct LazyVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T> {
    name: String,
    version: Version,
    source1: BoxedAnyIterableVec<S1I, S1T>,
    source2: BoxedAnyIterableVec<S2I, S2T>,
    source3: BoxedAnyIterableVec<S3I, S3T>,
    compute: ComputeFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>,
    phantom: PhantomData<I>,
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> LazyVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
{
    pub fn init(
        value_name: &str,
        version: Version,
        source1: BoxedAnyIterableVec<S1I, S1T>,
        source2: BoxedAnyIterableVec<S2I, S2T>,
        source3: BoxedAnyIterableVec<S3I, S3T>,
        compute: ComputeFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>,
    ) -> Self {
        if ([
            source1.index_type_to_string(),
            source2.index_type_to_string(),
            source3.index_type_to_string(),
        ])
        .into_iter()
        .filter(|t| *t == I::to_string())
        .count()
            == 0
        {
            panic!("At least one should have same index");
        }

        Self {
            name: I::to_folder_name(value_name),
            version,
            source1,
            source2,
            source3,
            compute,
            phantom: PhantomData,
        }
    }

    fn version(&self) -> Version {
        self.version
    }
}

pub struct LazyVecFrom3Iterator<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T> {
    lazy: &'a LazyVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>,
    source1: BoxedVecIterator<'a, S1I, S1T>,
    source2: BoxedVecIterator<'a, S2I, S2T>,
    source3: BoxedVecIterator<'a, S3I, S3T>,
    index: usize,
}

impl<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T> Iterator
    for LazyVecFrom3Iterator<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredType + 'a,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
{
    type Item = (I, Value<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        let index = I::from(self.index);
        let opt = (self.lazy.compute)(
            index,
            &mut *self.source1,
            &mut *self.source2,
            &mut *self.source3,
        )
        .map(|v| (index, Value::Owned(v)));
        if opt.is_some() {
            self.index += 1;
        }
        opt
    }
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> BaseVecIterator
    for LazyVecFrom3Iterator<'_, I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
{
    #[inline]
    fn mut_index(&mut self) -> &mut usize {
        &mut self.index
    }

    #[inline]
    fn len(&self) -> usize {
        let len1 = if self.source1.index_type_to_string() == I::to_string() {
            self.source1.len()
        } else {
            usize::MAX
        };
        let len2 = if self.source2.index_type_to_string() == I::to_string() {
            self.source2.len()
        } else {
            usize::MAX
        };
        let len3 = if self.source3.index_type_to_string() == I::to_string() {
            self.source3.len()
        } else {
            usize::MAX
        };
        len1.min(len2).min(len3)
    }

    #[inline]
    fn path(&self) -> &Path {
        self.source1.path()
    }
}

impl<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T> IntoIterator
    for &'a LazyVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredType + 'a,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
{
    type Item = (I, Value<'a, T>);
    type IntoIter = LazyVecFrom3Iterator<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T>;

    fn into_iter(self) -> Self::IntoIter {
        LazyVecFrom3Iterator {
            lazy: self,
            source1: self.source1.iter(),
            source2: self.source2.iter(),
            source3: self.source3.iter(),
            index: 0,
        }
    }
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> AnyVec for LazyVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
{
    fn version(&self) -> Version {
        self.version()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn index_type_to_string(&self) -> String {
        I::to_string()
    }

    fn len(&self) -> usize {
        let len1 = if self.source1.index_type_to_string() == I::to_string() {
            self.source1.len()
        } else {
            usize::MAX
        };
        let len2 = if self.source2.index_type_to_string() == I::to_string() {
            self.source2.len()
        } else {
            usize::MAX
        };
        let len3 = if self.source3.index_type_to_string() == I::to_string() {
            self.source3.len()
        } else {
            usize::MAX
        };
        len1.min(len2).min(len3)
    }

    fn modified_time(&self) -> Result<std::time::Duration> {
        Ok(self
            .source1
            .modified_time()?
            .min(self.source2.modified_time()?)
            .min(self.source3.modified_time()?))
    }
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> AnyIterableVec<I, T>
    for LazyVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
{
    fn boxed_iter<'a>(&'a self) -> BoxedVecIterator<'a, I, T>
    where
        T: 'a,
    {
        Box::new(self.into_iter())
    }
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> AnyCollectableVec
    for LazyVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
{
    fn collect_range_serde_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
