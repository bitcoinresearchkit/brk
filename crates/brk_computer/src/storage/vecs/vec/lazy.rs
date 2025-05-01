use std::marker::PhantomData;

use brk_vec::{StoredIndex, StoredType, StoredVec, StoredVecIterator, Value, Version};

// LazyVec owns SourceVecs
//
// Functions:
// init()
// version()
// iter()
// len() ?
//
// When .iter() called convert SourcesVecs into iterators
// iter owns source iters
// call compute function to convert index and source iters to T

#[derive(Clone)]
pub struct LazyVec<I, T, S1I, S1T, S2I, S2T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
{
    version: Version,
    source1: StoredVec<S1I, S1T>,
    source2: StoredVec<S2I, S2T>,
    compute: for<'a> fn(
        usize,
        &mut (dyn Iterator<Item = (S1I, Value<'a, S1T>)>),
        &mut (dyn Iterator<Item = (S2I, Value<'a, S2T>)>),
    ) -> Option<T>,
    phantom: PhantomData<I>,
}

impl<I, T, S1I, S1T, S2I, S2T> LazyVec<I, T, S1I, S1T, S2I, S2T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
{
    pub fn init(
        version: Version,
        source1: StoredVec<S1I, S1T>,
        source2: StoredVec<S2I, S2T>,
        compute: for<'a> fn(
            usize,
            &mut (dyn Iterator<Item = (S1I, Value<'a, S1T>)>),
            &mut (dyn Iterator<Item = (S2I, Value<'a, S2T>)>),
        ) -> Option<T>,
    ) -> Self {
        Self {
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

    fn len(&self) -> usize {
        todo!()
    }
}

pub struct LazyVecIterator<'a, I, T, S1I, S1T, S2I, S2T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
{
    lazy: &'a LazyVec<I, T, S1I, S1T, S2I, S2T>,
    source1: StoredVecIterator<'a, S1I, S1T>,
    source2: StoredVecIterator<'a, S2I, S2T>,
    index: usize,
}

impl<I, T, S1I, S1T, S2I, S2T> LazyVecIterator<'_, I, T, S1I, S1T, S2I, S2T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
{
    #[inline]
    pub fn set(&mut self, i: I) -> &mut Self {
        self.index = i.unwrap_to_usize();
        self
    }

    #[inline]
    pub fn set_(&mut self, i: usize) {
        self.index = i;
    }

    #[inline]
    pub fn get(&mut self, i: I) -> Option<Value<'_, T>> {
        self.set(i).next().map(|(_, v)| v)
    }

    #[inline]
    pub fn get_(&mut self, i: usize) -> Option<Value<'_, T>> {
        self.set_(i);
        self.next().map(|(_, v)| v)
    }
}

// impl<'a, I, T, S1I, S1T, S2I, S2T> VecIterator<'a> for LazyVecIterator<'a, I, T, S1I, S1T, S2I, S2T>
// where
//     I: StoredIndex,
//     T: StoredType,
//     S1I: StoredIndex,
//     S1T: StoredType,
//     S2I: StoredIndex,
//     S2T: StoredType,
// {
//     type I = I;
//     type T = T;

//     #[inline]
//     fn mut_index(&mut self) -> &mut usize {
//         &mut self.index
//     }

//     #[inline]
//     fn len(&self) -> usize {
//         todo!();
//         // self.vec.len()
//     }
// }

impl<'a, I, T, S1I, S1T, S2I, S2T> Iterator for LazyVecIterator<'a, I, T, S1I, S1T, S2I, S2T>
where
    I: StoredIndex,
    T: StoredType + 'a,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
{
    type Item = (I, Value<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        let opt = (self.lazy.compute)(
            self.index,
            &mut self.lazy.source1.iter(),
            &mut self.lazy.source2.iter(),
        )
        .map(|v| (I::from(self.index), Value::Owned(v)));
        if opt.is_some() {
            self.index += 1;
        }
        opt
    }

    // fn last(mut self) -> Option<Self::Item>
    // where
    //     Self: Sized,
    // {
    //     let len = self.vec.len();
    //     if len == 0 {
    //         return None;
    //     }
    //     let i = len - 1;
    //     self.get_(i)
    //         .map(|v| (I::from(i), Value::Owned(v.into_inner())))
    // }
}

impl<'a, I, T, S1I, S1T, S2I, S2T> IntoIterator for &'a LazyVec<I, T, S1I, S1T, S2I, S2T>
where
    I: StoredIndex,
    T: StoredType + 'a,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
{
    type Item = (I, Value<'a, T>);
    type IntoIter = LazyVecIterator<'a, I, T, S1I, S1T, S2I, S2T>;

    fn into_iter(self) -> Self::IntoIter {
        LazyVecIterator {
            lazy: self,
            source1: self.source1.iter(),
            source2: self.source2.iter(),
            index: 0,
        }
    }
}
