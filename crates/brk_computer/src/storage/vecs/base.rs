use core::error;
use std::{
    cmp::Ordering,
    fmt::Debug,
    io,
    ops::{Add, Sub},
    path::{Path, PathBuf},
};

use brk_core::CheckedSub;
use brk_exit::Exit;
use brk_vec::{
    AnyStorableVec, Compressed, Error, Result, StorableVec, StoredIndex, StoredType, Version,
};

const FLUSH_EVERY: usize = 10_000;

#[derive(Debug)]
pub struct ComputedVec<I, T> {
    computed_version: Option<Version>,
    vec: StorableVec<I, T>,
}

impl<I, T> ComputedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    pub fn forced_import(
        path: &Path,
        version: Version,
        compressed: Compressed,
    ) -> brk_vec::Result<Self> {
        let vec = StorableVec::forced_import(path, version, compressed)?;

        Ok(Self {
            computed_version: None,
            vec,
        })
    }

    fn safe_truncate_if_needed(&mut self, index: I, exit: &Exit) -> Result<()> {
        if exit.triggered() {
            return Ok(());
        }
        exit.block();
        self.vec.truncate_if_needed(index)?;
        exit.release();
        Ok(())
    }

    #[inline]
    pub fn forced_push_at(&mut self, index: I, value: T, exit: &Exit) -> Result<()> {
        match self.len().cmp(&index.to_usize()?) {
            Ordering::Less => {
                return Err(Error::IndexTooHigh);
            }
            ord => {
                if ord == Ordering::Greater {
                    self.safe_truncate_if_needed(index, exit)?;
                }
                self.vec.push(value);
            }
        }

        if self.vec.pushed_len() >= FLUSH_EVERY {
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
        self.vec.flush()?;
        exit.release();
        Ok(())
    }

    fn version(&self) -> Version {
        self.vec.version()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn vec(&self) -> &StorableVec<I, T> {
        &self.vec
    }

    pub fn mut_vec(&mut self) -> &mut StorableVec<I, T> {
        &mut self.vec
    }

    pub fn any_vec(&self) -> &dyn AnyStorableVec {
        &self.vec
    }

    pub fn mut_any_vec(&mut self) -> &mut dyn AnyStorableVec {
        &mut self.vec
    }

    pub fn get(&mut self, index: I) -> Result<Option<&T>> {
        self.vec.get(index)
    }

    pub fn collect_range(&self, from: Option<i64>, to: Option<i64>) -> Result<Vec<T>> {
        self.vec.collect_range(from, to)
    }

    #[inline]
    fn path_computed_version(&self) -> PathBuf {
        self.vec.path().join("computed_version")
    }

    fn validate_computed_version_or_reset_file(&mut self, version: Version) -> Result<()> {
        let path = self.path_computed_version();
        if version.validate(path.as_ref()).is_err() {
            self.vec.reset()?;
        }
        version.write(path.as_ref())?;
        Ok(())
    }

    pub fn compute_transform<A, B, F>(
        &mut self,
        max_from: A,
        other: &mut StorableVec<A, B>,
        mut t: F,
        exit: &Exit,
    ) -> Result<()>
    where
        A: StoredIndex,
        B: StoredType,
        F: FnMut((A, B, &mut Self, &mut StorableVec<A, B>)) -> (I, T),
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + other.version(),
        )?;

        let index = max_from.min(A::from(self.len()));
        other.iter_from_cloned(index, |(a, b, other)| {
            let (i, v) = t((a, b, self, other));
            self.forced_push_at(i, v, exit)
        })?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_inverse_more_to_less(
        &mut self,
        max_from: T,
        other: &mut StorableVec<T, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType + StoredIndex,
        T: StoredIndex,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + other.version(),
        )?;

        let index = max_from.min(self.vec.get_last()?.cloned().unwrap_or_default());
        other.iter_from(index, |(v, i, ..)| {
            let i = *i;
            if self.get(i).unwrap().is_none_or(|old_v| *old_v > v) {
                self.forced_push_at(i, v, exit)
            } else {
                Ok(())
            }
        })?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_inverse_less_to_more(
        &mut self,
        max_from: T,
        first_indexes: &mut StorableVec<T, I>,
        last_indexes: &mut StorableVec<T, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType,
        T: StoredIndex,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + first_indexes.version() + last_indexes.version(),
        )?;

        let index = max_from.min(T::from(self.len()));
        first_indexes.iter_from(index, |(value, first_index, ..)| {
            let first_index = (first_index).to_usize()?;
            let last_index = (last_indexes.get(value)?.unwrap()).to_usize()?;
            (first_index..last_index)
                .try_for_each(|index| self.forced_push_at(I::from(index), value, exit))
        })?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_last_index_from_first(
        &mut self,
        max_from: I,
        first_indexes: &mut StorableVec<I, T>,
        final_len: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Copy + From<usize> + CheckedSub<T> + StoredIndex,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + first_indexes.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let one = T::from(1);
        let mut prev_index: Option<I> = None;
        first_indexes.iter_from(index, |(i, v, ..)| {
            if let Some(prev_index) = prev_index.take() {
                self.forced_push_at(prev_index, v.checked_sub(one).unwrap(), exit)?;
            }
            prev_index.replace(i);
            Ok(())
        })?;
        if let Some(prev_index) = prev_index {
            self.forced_push_at(
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
        first_indexes: &mut StorableVec<I, T2>,
        last_indexes: &mut StorableVec<I, T2>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<T2>,
        T2: StoredType + Copy + Add<usize, Output = T2> + CheckedSub<T2> + TryInto<T> + Default,
        <T2 as TryInto<T>>::Error: error::Error + 'static,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + first_indexes.version() + last_indexes.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        first_indexes.iter_from(index, |(i, first_index, ..)| {
            let last_index = last_indexes.get(i)?.unwrap();
            let count = (*last_index + 1_usize)
                .checked_sub(*first_index)
                .unwrap_or_default();
            self.forced_push_at(i, count.into(), exit)
        })?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_is_first_ordered<A>(
        &mut self,
        max_from: I,
        self_to_other: &mut StorableVec<I, A>,
        other_to_self: &mut StorableVec<A, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType,
        T: From<bool>,
        A: StoredIndex + StoredType,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + self_to_other.version() + other_to_self.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        self_to_other.iter_from(index, |(i, other, ..)| {
            self.forced_push_at(i, T::from(other_to_self.get(*other)?.unwrap() == &i), exit)
        })?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_sum_from_indexes<T2>(
        &mut self,
        max_from: I,
        first_indexes: &mut StorableVec<I, T2>,
        last_indexes: &mut StorableVec<I, T2>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<T2>,
        T2: StoredType + Copy + Add<usize, Output = T2> + Sub<T2, Output = T2> + TryInto<T>,
        <T2 as TryInto<T>>::Error: error::Error + 'static,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + first_indexes.version() + last_indexes.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        first_indexes.iter_from(index, |(index, first_index, ..)| {
            let last_index = last_indexes.get(index)?.unwrap();
            let count = *last_index + 1_usize - *first_index;
            self.forced_push_at(index, count.into(), exit)
        })?;

        Ok(self.safe_flush(exit)?)
    }
}

impl<I, T> Clone for ComputedVec<I, T>
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
