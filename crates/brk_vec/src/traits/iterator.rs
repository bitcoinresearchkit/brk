use std::iter::Skip;

use crate::Value;

use super::{StoredIndex, StoredType};

pub trait BaseVecIterator: Iterator {
    fn mut_index(&mut self) -> &mut usize;

    #[inline]
    fn set_(&mut self, i: usize) {
        *self.mut_index() = i;
    }

    #[inline]
    fn next_at(&mut self, i: usize) -> Option<Self::Item> {
        self.set_(i);
        self.next()
    }

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn skip(self, _: usize) -> Skip<Self>
    where
        Self: Sized,
    {
        todo!("")
    }
}

pub trait VecIterator<'a>: BaseVecIterator<Item = (Self::I, Value<'a, Self::T>)> {
    type I: StoredIndex;
    type T: StoredType + 'a;

    #[inline]
    fn set(&mut self, i: Self::I) {
        self.set_(i.unwrap_to_usize())
    }

    #[inline]
    fn get_(&mut self, i: usize) -> Option<Value<'a, Self::T>> {
        self.next_at(i).map(|(_, v)| v)
    }

    #[inline]
    fn get(&mut self, i: Self::I) -> Option<Value<'a, Self::T>> {
        self.get_(i.unwrap_to_usize())
    }

    #[inline]
    fn unwrap_get_inner(&mut self, i: Self::I) -> Self::T {
        self.get_(i.unwrap_to_usize()).unwrap().into_inner()
    }

    #[inline]
    fn get_inner(&mut self, i: Self::I) -> Option<Self::T> {
        self.get_(i.unwrap_to_usize()).map(|v| v.into_inner())
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let len = self.len();
        if len == 0 {
            return None;
        }
        let i = len - 1;
        self.set_(i);
        self.next().map(|(i, v)| (i, Value::Owned(v.into_inner())))
    }

    fn index_type_to_string(&self) -> &str {
        Self::I::to_string()
    }
}

impl<'a, I, T, Iter> VecIterator<'a> for Iter
where
    Iter: BaseVecIterator<Item = (I, Value<'a, T>)>,
    I: StoredIndex,
    T: StoredType + 'a,
{
    type I = I;
    type T = T;
}

pub type BoxedVecIterator<'a, I, T> =
    Box<dyn VecIterator<'a, I = I, T = T, Item = (I, Value<'a, T>)> + 'a>;
