use std::ops::Range;

use memmap2::Mmap;
use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

use crate::MAX_PAGE_SIZE;

use super::Result;

#[derive(Debug)]
pub enum Values<T> {
    Owned(Box<[T]>),
    Ref(Box<Mmap>),
}

impl<T> Values<T> {
    const PER_PAGE: usize = MAX_PAGE_SIZE / Self::SIZE_OF_T;
    const SIZE_OF_T: usize = size_of::<T>();

    pub fn get(&self, index: usize) -> Result<Option<&T>>
    where
        T: TryFromBytes + IntoBytes + Immutable + KnownLayout,
    {
        let index = Self::index_to_decoded_index(index);

        Ok(match self {
            Self::Owned(a) => a.get(index),
            Self::Ref(m) => {
                let range = Self::index_to_byte_range(index);
                let source = &m[range];
                Some(T::try_ref_from_bytes(source)?)
            }
        })
    }

    pub fn as_arr(&self) -> &[T] {
        match self {
            Self::Owned(a) => a,
            Self::Ref(_) => unreachable!(),
        }
    }

    pub fn as_mmap(&self) -> &Mmap {
        match self {
            Self::Owned(_) => unreachable!(),
            Self::Ref(m) => m,
        }
    }

    #[inline]
    fn index_to_byte_range(index: usize) -> Range<usize> {
        let index = Self::index_to_byte_index(index) as usize;
        index..(index + Self::SIZE_OF_T)
    }

    #[inline]
    fn index_to_byte_index(index: usize) -> u64 {
        (index * Self::SIZE_OF_T) as u64
    }

    #[inline(always)]
    fn index_to_decoded_index(index: usize) -> usize {
        index % Self::PER_PAGE
    }
}

impl<T> From<Box<[T]>> for Values<T> {
    fn from(value: Box<[T]>) -> Self {
        Self::Owned(value)
    }
}

impl<T> From<Mmap> for Values<T> {
    fn from(value: Mmap) -> Self {
        Self::Ref(Box::new(value))
    }
}

impl<T> Default for Values<T> {
    fn default() -> Self {
        Self::Owned(vec![].into_boxed_slice())
    }
}
