use std::marker::PhantomData;

use brk_vec::{
    BaseVecIterator, StoredIndex, StoredType, StoredVec, StoredVecIterator, Value, Version,
};

pub type ComputeFrom3<T, S1I, S1T, S2I, S2T, S3I, S3T> = for<'a> fn(
    usize,
    &mut dyn BaseVecIterator<Item = (S1I, Value<'a, S1T>)>,
    &mut dyn BaseVecIterator<Item = (S2I, Value<'a, S2T>)>,
    &mut dyn BaseVecIterator<Item = (S3I, Value<'a, S3T>)>,
) -> Option<T>;

#[derive(Clone)]
pub struct LazyVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T> {
    version: Version,
    source1: StoredVec<S1I, S1T>,
    source2: StoredVec<S2I, S2T>,
    source3: StoredVec<S3I, S3T>,
    compute: ComputeFrom3<T, S1I, S1T, S2I, S2T, S3I, S3T>,
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
        version: Version,
        source1: StoredVec<S1I, S1T>,
        source2: StoredVec<S2I, S2T>,
        source3: StoredVec<S3I, S3T>,
        compute: ComputeFrom3<T, S1I, S1T, S2I, S2T, S3I, S3T>,
    ) -> Self {
        Self {
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

pub struct LazyVecIterator<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T> {
    lazy: &'a LazyVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>,
    source1: StoredVecIterator<'a, S1I, S1T>,
    source2: StoredVecIterator<'a, S2I, S2T>,
    source3: StoredVecIterator<'a, S3I, S3T>,
    index: usize,
}

impl<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T> Iterator
    for LazyVecIterator<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T>
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
        let opt = (self.lazy.compute)(
            self.index,
            &mut self.lazy.source1.iter(),
            &mut self.lazy.source2.iter(),
            &mut self.lazy.source3.iter(),
        )
        .map(|v| (I::from(self.index), Value::Owned(v)));
        if opt.is_some() {
            self.index += 1;
        }
        opt
    }
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> BaseVecIterator
    for LazyVecIterator<'_, I, T, S1I, S1T, S2I, S2T, S3I, S3T>
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
        todo!();
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
    type IntoIter = LazyVecIterator<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T>;

    fn into_iter(self) -> Self::IntoIter {
        LazyVecIterator {
            lazy: self,
            source1: self.source1.iter(),
            source2: self.source2.iter(),
            source3: self.source3.iter(),
            index: 0,
        }
    }
}
