use std::{
    error,
    fmt::Debug,
    io,
    ops::{Add, Sub},
    path::Path,
};

use derive_deref::{Deref, DerefMut};
use storable_vec::{StorableVecIndex, StorableVecType, Version, SINGLE_THREAD};

#[derive(Debug, Deref, DerefMut)]
pub struct StorableVec<I, T, const MODE: u8>(storable_vec::StorableVec<I, T, MODE>);

const FLUSH_EVERY: usize = 10_000;

impl<I, T, const MODE: u8> StorableVec<I, T, MODE>
where
    I: StorableVecIndex,
    T: StorableVecType,
{
    pub fn import(path: &Path, version: Version) -> storable_vec::Result<Self> {
        Ok(Self(storable_vec::StorableVec::forced_import(path, version)?))
    }
}

impl<I, T> StorableVec<I, T, SINGLE_THREAD>
where
    I: StorableVecIndex,
    T: StorableVecType,
{
    fn flush_vec_if_needed(&mut self) -> io::Result<()> {
        if self.pushed_len() == FLUSH_EVERY {
            self.flush()
        } else {
            Ok(())
        }
    }

    pub fn compute_is_first_ordered<A>(
        &mut self,
        self_to_other: &storable_vec::StorableVec<I, A, SINGLE_THREAD>,
        other_to_self: &storable_vec::StorableVec<A, I, SINGLE_THREAD>,
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
        first_index_vec: &storable_vec::StorableVec<I, T, SINGLE_THREAD>,
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
            Ok(self.flush_vec_if_needed()?)
        })?;
        if let Some(prev_index) = prev_index {
            self.push_if_needed(prev_index, T::from(final_len) - T::from(1))?;
        }
        self.flush()?;
        Ok(())
    }

    pub fn compute_count_from_indexes<T2>(
        &mut self,
        first_indexes: &storable_vec::StorableVec<I, T2, SINGLE_THREAD>,
        last_indexes: &storable_vec::StorableVec<I, T2, SINGLE_THREAD>,
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
            Ok(self.flush_vec_if_needed()?)
        })?;
        self.flush()?;
        Ok(())
    }
}
