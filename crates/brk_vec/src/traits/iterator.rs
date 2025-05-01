use crate::Value;

use super::{StoredIndex, StoredType};

// pub trait BaseVecIterator: Iterator {
//     fn mut_index(&mut self) -> &mut usize;

//     fn len(&self) -> usize;

//     fn is_empty(&self) -> bool {
//         self.len() == 0
//     }

//     #[inline]
//     fn set_(&mut self, i: usize) -> &mut Self {
//         *self.mut_index() = i;
//         self
//     }

//     fn skip(self, _: usize) -> std::iter::Skip<Self>
//     where
//         Self: Sized,
//     {
//         todo!("")
//     }
// }

pub trait VecIterator<'a>: Iterator<Item = (Self::I, Value<'a, Self::T>)> + 'a {
    type I: StoredIndex;
    type T: StoredType;

    fn mut_index(&mut self) -> &mut usize;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    fn set_(&mut self, i: usize) -> &mut Self {
        *self.mut_index() = i;
        self
    }

    fn skip(self, _: usize) -> std::iter::Skip<Self>
    where
        Self: Sized,
    {
        todo!("")
    }

    //     fn set(&mut self, i: Self::I) -> &mut Self;

    //     fn get_(&mut self, i: usize) -> Option<Value<'a, Self::T>>;

    //     fn get(&mut self, i: Self::I) -> Option<Value<'a, Self::T>>;

    //     fn unwrap_get_inner(&mut self, i: Self::I) -> Self::T;

    //     fn get_inner(&mut self, i: Self::I) -> Option<Self::T>;

    //     fn last(self) -> Option<Self::Item>;
    // }

    // impl<'a, I, T, Iter> VecIterator<'a, I, T> for Iter
    // where
    //     I: StoredIndex,
    //     T: StoredType + 'a,
    //     Iter: Iterator<Item = (I, Value<'a, T>)> + BaseVecIterator,
    // {
    #[inline]
    fn set(&mut self, i: Self::I) -> &mut Self {
        self.set_(i.unwrap_to_usize())
    }

    #[inline]
    fn get_(&mut self, i: usize) -> Option<Value<'a, Self::T>> {
        self.set_(i);
        self.next().map(|(_, v)| v)
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
}

impl<'a, I, T, Iter> VecIterator<'a> for Iter
where
    Iter: Iterator<Item = (I, Value<'a, T>)> + 'a,
    I: StoredIndex,
    T: StoredType + 'a,
{
    type I = I;
    type T = T;

    fn len(&self) -> usize {
        todo!()
    }

    fn mut_index(&mut self) -> &mut usize {
        todo!()
    }
}
// pub trait VecIterator<'a>: Iterator<Item = (Self::I, Value<'a, Self::T>)> + 'a {
