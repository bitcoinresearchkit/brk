use std::{
    error,
    fmt::Debug,
    io,
    ops::{Add, Sub},
    path::Path,
};

use derive_deref::{Deref, DerefMut};
use storable_vec::{StorableVecIndex, StorableVecType, Version};

#[derive(Debug, Deref, DerefMut)]
pub struct StorableVec<I, T>(storable_vec::StorableVec<I, T>);

const FLUSH_EVERY: usize = 10_000;

impl<I, T> StorableVec<I, T>
where
    I: StorableVecIndex,
    T: StorableVecType,
{
    pub fn import(path: &Path, version: Version) -> io::Result<Self> {
        Ok(Self(storable_vec::StorableVec::import(path, version)?))
    }

    fn flush_vec_if_needed(&mut self) -> io::Result<()> {
        if self.pushed_len() == FLUSH_EVERY {
            self.flush()
        } else {
            Ok(())
        }
    }

    pub fn compute_inverse_more_to_less(&mut self, other: &storable_vec::StorableVec<T, I>) -> storable_vec::Result<()>
    where
        I: StorableVecType,
        T: StorableVecIndex,
    {
        other.iter_from(self.last()?.map(|v| *v).unwrap_or_default(), |(v, i)| {
            self.push_if_needed(*i, v)
        })
    }

    pub fn compute_inverse_less_to_more(
        &mut self,
        first_indexes: &storable_vec::StorableVec<T, I>,
        last_indexes: &storable_vec::StorableVec<T, I>,
    ) -> color_eyre::Result<()>
    where
        I: StorableVecType,
        T: StorableVecIndex,
    {
        let (mut file_last, mut buf_last) = last_indexes.prepare_to_read_at_(self.len())?;
        first_indexes.iter_from(T::from(self.len()), |(value, first_index)| {
            let first_index: usize = (*first_index)
                .try_into()
                .map_err(|_| storable_vec::Error::FailedKeyTryIntoUsize)?;
            let last_index = last_indexes.read_exact(&mut file_last, &mut buf_last)?;
            let last_index: usize = (*last_index)
                .try_into()
                .map_err(|_| storable_vec::Error::FailedKeyTryIntoUsize)?;
            (first_index..last_index).try_for_each(|index| self.push_if_needed(I::from(index), value))?;
            Ok(())
        })?;
        self.flush()?;
        Ok(())
    }

    pub fn compute_transform<A, F>(&mut self, other: &storable_vec::StorableVec<I, A>, t: F) -> storable_vec::Result<()>
    where
        A: StorableVecType,
        F: Fn(&A) -> T,
    {
        other.iter_from(I::from(self.len()), |(i, a)| self.push_if_needed(i, t(a)))
    }

    pub fn compute_is_first_ordered<A>(
        &mut self,
        self_to_other: &storable_vec::StorableVec<I, A>,
        other_to_self: &storable_vec::StorableVec<A, I>,
    ) -> storable_vec::Result<()>
    where
        A: StorableVecIndex + StorableVecType,
    {
        // let mut prev_a_opt = None;
        // self_to_other.iter_from(I::from(self.len()), |(i, a)| {
        //     if prev_a_opt.is_none() {
        //         prev_a_opt.replace(a);
        //         self.push_if_needed(i, other_to_self.read_at(a) == i);
        //     } else {
        //         let prev_a = prev_a_opt.unwrap();
        //         if a != prev_a
        //     }
        //     other_to_self.seek_read(a);
        //     self.push_if_needed(i, t(a));
        //     Ok(())
        // })
        Ok(())
    }

    pub fn compute_last_index_from_first(
        &mut self,
        first_index_vec: &storable_vec::StorableVec<I, T>,
        final_len: usize,
    ) -> color_eyre::Result<()>
    where
        T: Copy + From<usize> + Sub<T, Output = T> + StorableVecIndex,
    {
        let mut prev_index: Option<I> = None;
        first_index_vec.iter_from(I::from(self.len()), |(i, v)| {
            if let Some(prev_index) = prev_index {
                self.push_if_needed(prev_index, *v - T::from(1))?;
            }
            prev_index.replace(i);
            self.flush_vec_if_needed().map_err(storable_vec::Error::IO)
        })?;
        if let Some(prev_index) = prev_index {
            self.push_if_needed(prev_index, T::from(final_len) - T::from(1))?;
        }
        self.flush()?;
        Ok(())
    }

    pub fn compute_count_from_indexes<T2>(
        &mut self,
        first_indexes: &storable_vec::StorableVec<I, T2>,
        last_indexes: &storable_vec::StorableVec<I, T2>,
    ) -> color_eyre::Result<()>
    where
        T: From<T2>,
        T2: StorableVecType + Copy + Add<usize, Output = T2> + Sub<T2, Output = T2> + TryInto<T>,
        <T2 as TryInto<T>>::Error: error::Error + Send + Sync + 'static,
    {
        let (mut file_last, mut buf_last) = last_indexes.prepare_to_read_at_(self.len())?;
        first_indexes.iter_from(I::from(self.len()), |(i, first_index)| {
            let last_index = last_indexes.read_exact(&mut file_last, &mut buf_last)?;
            let count = *last_index + 1_usize - *first_index;
            self.push_if_needed(i, count.into())?;
            self.flush_vec_if_needed().map_err(storable_vec::Error::IO)
        })?;
        self.flush()?;
        Ok(())
    }
}
