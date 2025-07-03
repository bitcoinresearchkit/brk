use brk_core::Version;

use super::{BoxedVecIterator, StoredIndex, StoredType};

pub fn i64_to_usize(i: i64, len: usize) -> usize {
    if i >= 0 {
        (i as usize).min(len)
    } else {
        let v = len as i64 + i;
        if v < 0 { 0 } else { v as usize }
    }
}

pub trait AnyVec: Send + Sync {
    fn version(&self) -> Version;
    fn name(&self) -> &str;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn index_type_to_string(&self) -> &'static str;
    fn value_type_to_size_of(&self) -> usize;
    fn etag(&self, to: Option<i64>) -> String {
        let len = self.len();
        format!(
            "{}-{:?}",
            to.map_or(len, |to| {
                if to.is_negative() {
                    len.checked_sub(to.unsigned_abs() as usize)
                        .unwrap_or_default()
                } else {
                    to as usize
                }
            }),
            self.version()
        )
    }

    #[inline]
    fn i64_to_usize(&self, i: i64) -> usize {
        let len = self.len();
        i64_to_usize(i, len)
    }
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
