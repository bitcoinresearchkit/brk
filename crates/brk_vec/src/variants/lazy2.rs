use std::{borrow::Cow, marker::PhantomData};

use brk_core::{Result, Version};

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BaseVecIterator, BoxedAnyIterableVec,
    BoxedVecIterator, CollectableVec, StoredIndex, StoredType,
};

pub type ComputeFrom2<I, T, S1I, S1T, S2I, S2T> = for<'a> fn(
    I,
    &mut dyn BaseVecIterator<Item = (S1I, Cow<'a, S1T>)>,
    &mut dyn BaseVecIterator<Item = (S2I, Cow<'a, S2T>)>,
) -> Option<T>;

#[derive(Clone)]
pub struct LazyVecFrom2<I, T, S1I, S1T, S2I, S2T>
where
    S1T: Clone,
    S2T: Clone,
{
    name: String,
    version: Version,
    source1: BoxedAnyIterableVec<S1I, S1T>,
    source2: BoxedAnyIterableVec<S2I, S2T>,
    compute: ComputeFrom2<I, T, S1I, S1T, S2I, S2T>,
    phantom: PhantomData<I>,
}

impl<I, T, S1I, S1T, S2I, S2T> LazyVecFrom2<I, T, S1I, S1T, S2I, S2T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
{
    pub fn init(
        name: &str,
        version: Version,
        source1: BoxedAnyIterableVec<S1I, S1T>,
        source2: BoxedAnyIterableVec<S2I, S2T>,
        compute: ComputeFrom2<I, T, S1I, S1T, S2I, S2T>,
    ) -> Self {
        if ([
            source1.index_type_to_string(),
            source2.index_type_to_string(),
        ])
        .into_iter()
        .filter(|t| *t == I::to_string())
        .count()
            == 0
        {
            panic!("At least one should have same index");
        }

        Self {
            name: name.to_string(),
            version,
            source1,
            source2,
            compute,
            phantom: PhantomData,
        }
    }

    fn version(&self) -> Version {
        self.version
    }
}

pub struct LazyVecFrom2Iterator<'a, I, T, S1I, S1T, S2I, S2T>
where
    S1T: Clone,
    S2T: Clone,
{
    lazy: &'a LazyVecFrom2<I, T, S1I, S1T, S2I, S2T>,
    source1: BoxedVecIterator<'a, S1I, S1T>,
    source2: BoxedVecIterator<'a, S2I, S2T>,
    index: usize,
}

impl<'a, I, T, S1I, S1T, S2I, S2T> Iterator for LazyVecFrom2Iterator<'a, I, T, S1I, S1T, S2I, S2T>
where
    I: StoredIndex,
    T: StoredType + 'a,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
{
    type Item = (I, Cow<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        let index = I::from(self.index);
        let opt = (self.lazy.compute)(index, &mut *self.source1, &mut *self.source2)
            .map(|v| (index, Cow::Owned(v)));
        if opt.is_some() {
            self.index += 1;
        }
        opt
    }
}

impl<I, T, S1I, S1T, S2I, S2T> BaseVecIterator
    for LazyVecFrom2Iterator<'_, I, T, S1I, S1T, S2I, S2T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
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
        len1.min(len2)
    }

    #[inline]
    fn name(&self) -> &str {
        self.source1.name()
    }
}

impl<'a, I, T, S1I, S1T, S2I, S2T> IntoIterator for &'a LazyVecFrom2<I, T, S1I, S1T, S2I, S2T>
where
    I: StoredIndex,
    T: StoredType + 'a,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
{
    type Item = (I, Cow<'a, T>);
    type IntoIter = LazyVecFrom2Iterator<'a, I, T, S1I, S1T, S2I, S2T>;

    fn into_iter(self) -> Self::IntoIter {
        LazyVecFrom2Iterator {
            lazy: self,
            source1: self.source1.iter(),
            source2: self.source2.iter(),
            index: 0,
        }
    }
}

impl<I, T, S1I, S1T, S2I, S2T> AnyVec for LazyVecFrom2<I, T, S1I, S1T, S2I, S2T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
{
    fn version(&self) -> Version {
        self.version()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn index_type_to_string(&self) -> &'static str {
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
        len1.min(len2)
    }

    #[inline]
    fn value_type_to_size_of(&self) -> usize {
        size_of::<T>()
    }
}

impl<I, T, S1I, S1T, S2I, S2T> AnyIterableVec<I, T> for LazyVecFrom2<I, T, S1I, S1T, S2I, S2T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
{
    fn boxed_iter<'a>(&'a self) -> BoxedVecIterator<'a, I, T>
    where
        T: 'a,
    {
        Box::new(self.into_iter())
    }
}

impl<I, T, S1I, S1T, S2I, S2T> AnyCollectableVec for LazyVecFrom2<I, T, S1I, S1T, S2I, S2T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
{
    fn collect_range_serde_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
