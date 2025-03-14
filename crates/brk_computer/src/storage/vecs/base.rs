use core::error;
use std::{
    cmp::Ordering,
    fmt::Debug,
    io,
    ops::{Add, Deref, DerefMut, Sub},
    path::{Path, PathBuf},
};

use brk_core::CheckedSub;
use brk_exit::Exit;
use brk_vec::{Compressed, Error, Result, StoredIndex, StoredType, Version};

const FLUSH_EVERY: usize = 10_000;

#[derive(Debug)]
pub struct StorableVec<I, T> {
    computed_version: Option<Version>,
    vec: brk_vec::StorableVec<I, T>,
}

impl<I, T> StorableVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    pub fn import(path: &Path, version: Version) -> brk_vec::Result<Self> {
        let vec = brk_vec::StorableVec::forced_import(path, version, Compressed::YES)?;

        Ok(Self {
            computed_version: None,
            vec,
        })
    }

    #[inline]
    pub fn i_to_usize(index: I) -> Result<usize> {
        index.try_into().map_err(|_| Error::FailedKeyTryIntoUsize)
    }

    fn safe_truncate_if_needed(&mut self, index: I, exit: &Exit) -> Result<()> {
        if exit.triggered() {
            return Ok(());
        }
        exit.block();
        self.truncate_if_needed(index)?;
        exit.release();
        Ok(())
    }

    #[inline]
    fn push_and_flush_if_needed(&mut self, index: I, value: T, exit: &Exit) -> Result<()> {
        match self.len().cmp(&Self::i_to_usize(index)?) {
            Ordering::Less => {
                return Err(Error::IndexTooHigh);
            }
            ord => {
                if ord == Ordering::Greater {
                    self.safe_truncate_if_needed(index, exit)?;
                }
                self.push(value);
            }
        }

        if self.pushed_len() >= FLUSH_EVERY {
            Ok(self.safe_flush(exit)?)
        } else {
            Ok(())
        }
    }

    pub fn safe_flush(&mut self, exit: &Exit) -> io::Result<()> {
        if exit.triggered() {
            return Ok(());
        }
        exit.block();
        self.flush()?;
        exit.release();
        Ok(())
    }

    #[inline]
    fn path_computed_version(&self) -> PathBuf {
        self.path().join("computed_version")
    }

    fn validate_computed_version_or_reset_file(&mut self, version: Version) -> Result<()> {
        let path = self.path_computed_version();
        if version.validate(path.as_ref()).is_err() {
            self.reset()?;
        }
        version.write(path.as_ref())?;
        Ok(())
    }

    pub fn compute_transform<A, B, F>(
        &mut self,
        max_from: A,
        other: &mut brk_vec::StorableVec<A, B>,
        mut t: F,
        exit: &Exit,
    ) -> Result<()>
    where
        A: StoredIndex,
        B: StoredType,
        F: FnMut((A, B, &mut Self, &mut brk_vec::StorableVec<A, B>)) -> (I, T),
    {
        self.validate_computed_version_or_reset_file(
            Version::from(0) + self.version() + other.version(),
        )?;

        let index = max_from.min(A::from(self.len()));
        other.iter_from_cloned(index, |(a, b, other)| {
            let (i, v) = t((a, b, self, other));
            self.push_and_flush_if_needed(i, v, exit)
        })?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_inverse_more_to_less(
        &mut self,
        max_from: T,
        other: &mut brk_vec::StorableVec<T, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType + StoredIndex,
        T: StoredIndex,
    {
        self.validate_computed_version_or_reset_file(
            Version::from(0) + self.version() + other.version(),
        )?;

        let index = max_from.min(self.read_last()?.cloned().unwrap_or_default());
        other.iter_from(index, |(v, i, ..)| {
            let i = *i;
            if self.read(i).unwrap().is_none_or(|old_v| *old_v > v) {
                self.push_and_flush_if_needed(i, v, exit)
            } else {
                Ok(())
            }
        })?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_inverse_less_to_more(
        &mut self,
        max_from: T,
        first_indexes: &mut brk_vec::StorableVec<T, I>,
        last_indexes: &mut brk_vec::StorableVec<T, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType,
        T: StoredIndex,
    {
        self.validate_computed_version_or_reset_file(
            Version::from(0) + self.version() + first_indexes.version() + last_indexes.version(),
        )?;

        let index = max_from.min(T::from(self.len()));
        first_indexes.iter_from(index, |(value, first_index, ..)| {
            let first_index = Self::i_to_usize(*first_index)?;
            let last_index = Self::i_to_usize(*last_indexes.read(value)?.unwrap())?;
            (first_index..last_index)
                .try_for_each(|index| self.push_and_flush_if_needed(I::from(index), value, exit))
        })?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_last_index_from_first(
        &mut self,
        max_from: I,
        first_indexes: &mut brk_vec::StorableVec<I, T>,
        final_len: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Copy + From<usize> + CheckedSub<T> + StoredIndex,
    {
        self.validate_computed_version_or_reset_file(
            Version::from(0) + self.version() + first_indexes.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let one = T::from(1);
        let mut prev_index: Option<I> = None;
        first_indexes.iter_from(index, |(i, v, ..)| {
            if let Some(prev_index) = prev_index.take() {
                self.push_and_flush_if_needed(prev_index, v.checked_sub(one).unwrap(), exit)?;
            }
            prev_index.replace(i);
            Ok(())
        })?;
        if let Some(prev_index) = prev_index {
            self.push_and_flush_if_needed(
                prev_index,
                T::from(final_len).checked_sub(one).unwrap(),
                exit,
            )?;
        }

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_count_from_indexes<T2>(
        &mut self,
        max_from: I,
        first_indexes: &mut brk_vec::StorableVec<I, T2>,
        last_indexes: &mut brk_vec::StorableVec<I, T2>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<T2>,
        T2: StoredType + Copy + Add<usize, Output = T2> + Sub<T2, Output = T2> + TryInto<T>,
        <T2 as TryInto<T>>::Error: error::Error + 'static,
    {
        self.validate_computed_version_or_reset_file(
            Version::from(0) + self.version() + first_indexes.version() + last_indexes.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        first_indexes.iter_from(index, |(i, first_index, ..)| {
            let last_index = last_indexes.read(i)?.unwrap();
            let count = *last_index + 1_usize - *first_index;
            self.push_and_flush_if_needed(i, count.into(), exit)
        })?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_is_first_ordered<A>(
        &mut self,
        max_from: I,
        self_to_other: &mut brk_vec::StorableVec<I, A>,
        other_to_self: &mut brk_vec::StorableVec<A, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType,
        T: From<bool>,
        A: StoredIndex + StoredType,
    {
        self.validate_computed_version_or_reset_file(
            Version::from(0) + self.version() + self_to_other.version() + other_to_self.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        self_to_other.iter_from(index, |(i, other, ..)| {
            self.push_and_flush_if_needed(
                i,
                T::from(other_to_self.read(*other)?.unwrap() == &i),
                exit,
            )
        })?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_sum_from_indexes<T2>(
        &mut self,
        max_from: I,
        first_indexes: &mut brk_vec::StorableVec<I, T2>,
        last_indexes: &mut brk_vec::StorableVec<I, T2>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<T2>,
        T2: StoredType + Copy + Add<usize, Output = T2> + Sub<T2, Output = T2> + TryInto<T>,
        <T2 as TryInto<T>>::Error: error::Error + 'static,
    {
        self.validate_computed_version_or_reset_file(
            Version::from(0) + self.version() + first_indexes.version() + last_indexes.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        first_indexes.iter_from(index, |(index, first_index, ..)| {
            let last_index = last_indexes.read(index)?.unwrap();
            let count = *last_index + 1_usize - *first_index;
            self.push_and_flush_if_needed(index, count.into(), exit)
        })?;

        Ok(self.safe_flush(exit)?)
    }
}

impl<I, T> Deref for StorableVec<I, T> {
    type Target = brk_vec::StorableVec<I, T>;
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
impl<I, T> DerefMut for StorableVec<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}
impl<I, T> Clone for StorableVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn clone(&self) -> Self {
        Self {
            computed_version: self.computed_version,
            vec: self.vec.clone(),
        }
    }
}
