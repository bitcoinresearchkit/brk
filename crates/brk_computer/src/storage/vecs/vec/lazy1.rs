use std::marker::PhantomData;

use brk_vec::{
    BaseVecIterator, StoredIndex, StoredType, StoredVec, StoredVecIterator, Value, Version,
};

pub type ComputeFrom1<T, S1I, S1T> =
    for<'a> fn(usize, &mut dyn BaseVecIterator<Item = (S1I, Value<'a, S1T>)>) -> Option<T>;

#[derive(Clone)]
pub struct LazyVecFrom1<I, T, S1I, S1T> {
    version: Version,
    source: StoredVec<S1I, S1T>,
    compute: ComputeFrom1<T, S1I, S1T>,
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
        version: Version,
        source: StoredVec<S1I, S1T>,
        compute: ComputeFrom1<T, S1I, S1T>,
    ) -> Self {
        Self {
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

pub struct LazyVecIterator<'a, I, T, S1I, S1T> {
    lazy: &'a LazyVecFrom1<I, T, S1I, S1T>,
    source: StoredVecIterator<'a, S1I, S1T>,
    index: usize,
}

impl<'a, I, T, S1I, S1T> Iterator for LazyVecIterator<'a, I, T, S1I, S1T>
where
    I: StoredIndex,
    T: StoredType + 'a,
    S1I: StoredIndex,
    S1T: StoredType,
{
    type Item = (I, Value<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        let opt = (self.lazy.compute)(self.index, &mut self.lazy.source.iter())
            .map(|v| (I::from(self.index), Value::Owned(v)));
        if opt.is_some() {
            self.index += 1;
        }
        opt
    }
}

impl<I, T, S1I, S1T> BaseVecIterator for LazyVecIterator<'_, I, T, S1I, S1T>
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
        todo!();
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
    type IntoIter = LazyVecIterator<'a, I, T, S1I, S1T>;

    fn into_iter(self) -> Self::IntoIter {
        LazyVecIterator {
            lazy: self,
            source: self.source.iter(),
            index: 0,
        }
    }
}
