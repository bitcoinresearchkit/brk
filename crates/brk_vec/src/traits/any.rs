use std::time::Duration;

use crate::{Result, Version};

use super::{BoxedVecIterator, StoredIndex, StoredType};

pub trait AnyVec: Send + Sync {
    fn version(&self) -> Version;
    fn name(&self) -> String;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn modified_time(&self) -> Result<Duration>;
    fn index_type_to_string(&self) -> &str;
}

pub trait AnyIterableVec<I, T>: AnyVec {
    #[allow(clippy::wrong_self_convention)]
    fn boxed_iter<'a>(&'a self) -> BoxedVecIterator<'a, I, T>
    where
        I: StoredIndex,
        T: StoredType + 'a;

    fn iter<'a>(&'a self) -> BoxedVecIterator<'a, I, T>
    where
        I: StoredIndex,
        T: StoredType + 'a,
    {
        self.boxed_iter()
    }

    fn iter_at<'a>(&'a self, i: I) -> BoxedVecIterator<'a, I, T>
    where
        I: StoredIndex,
        T: StoredType + 'a,
    {
        let mut iter = self.boxed_iter();
        iter.set(i);
        iter
    }

    fn iter_at_<'a>(&'a self, i: usize) -> BoxedVecIterator<'a, I, T>
    where
        I: StoredIndex,
        T: StoredType + 'a,
    {
        let mut iter = self.boxed_iter();
        iter.set_(i);
        iter
    }
}

pub trait CloneableAnyIterableVec<I, T>: AnyIterableVec<I, T> {
    fn boxed_clone(&self) -> Box<dyn CloneableAnyIterableVec<I, T>>;
}

impl<I, T, U> CloneableAnyIterableVec<I, T> for U
where
    U: 'static + AnyIterableVec<I, T> + Clone,
{
    fn boxed_clone(&self) -> Box<dyn CloneableAnyIterableVec<I, T>> {
        Box::new(self.clone())
    }
}

impl<I, T> Clone for Box<dyn CloneableAnyIterableVec<I, T>> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}

pub type BoxedAnyIterableVec<I, T> = Box<dyn CloneableAnyIterableVec<I, T>>;
