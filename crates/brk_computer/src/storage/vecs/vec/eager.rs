use core::error;
use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::Add,
    path::{Path, PathBuf},
};

use brk_core::{Bitcoin, CheckedSub, Close, Dollars, Height, Sats, Txindex};
use brk_exit::Exit;
use brk_vec::{
    Compressed, DynamicVec, Error, GenericVec, Result, StoredIndex, StoredType, StoredVec, Version,
};
use log::info;

const ONE_KIB: usize = 1024;
const ONE_MIB: usize = ONE_KIB * ONE_KIB;
const MAX_CACHE_SIZE: usize = 210 * ONE_MIB;

#[derive(Debug)]
pub struct EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    computed_version: Option<Version>,
    inner: StoredVec<I, T>,
}

impl<I, T> EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    const SIZE_OF: usize = size_of::<T>();

    pub fn forced_import(
        path: &Path,
        version: Version,
        compressed: Compressed,
    ) -> brk_vec::Result<Self> {
        let inner = StoredVec::forced_import(path, version, compressed)?;

        Ok(Self {
            computed_version: None,
            inner,
        })
    }

    fn safe_truncate_if_needed(&mut self, index: I, exit: &Exit) -> Result<()> {
        if exit.triggered() {
            return Ok(());
        }
        exit.block();
        self.inner.truncate_if_needed(index)?;
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
                self.inner.push(value);
            }
        }

        if self.inner.pushed_len() * Self::SIZE_OF >= MAX_CACHE_SIZE {
            self.safe_flush(exit)
        } else {
            Ok(())
        }
    }

    pub fn safe_flush(&mut self, exit: &Exit) -> Result<()> {
        if exit.triggered() {
            return Ok(());
        }
        exit.block();
        self.inner.flush()?;
        exit.release();
        Ok(())
    }

    fn version(&self) -> Version {
        self.inner.version()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn file_name(&self) -> String {
        self.inner.file_name()
    }

    pub fn vec(&self) -> &StoredVec<I, T> {
        &self.inner
    }

    pub fn mut_vec(&mut self) -> &mut StoredVec<I, T> {
        &mut self.inner
    }

    pub fn any_vec(&self) -> &dyn brk_vec::AnyStoredVec {
        &self.inner
    }

    pub fn mut_any_vec(&mut self) -> &mut dyn brk_vec::AnyStoredVec {
        &mut self.inner
    }

    pub fn unwrap_cached_get(&mut self, index: I) -> Option<T> {
        self.inner.unwrap_cached_get(index)
    }
    #[inline]
    pub fn double_unwrap_cached_get(&mut self, index: I) -> T {
        self.inner.double_unwrap_cached_get(index)
    }

    pub fn collect_inclusive_range(&self, from: I, to: I) -> Result<Vec<T>> {
        self.inner.collect_inclusive_range(from, to)
    }

    pub fn path(&self) -> &Path {
        self.inner.path()
    }

    #[inline]
    fn path_computed_version(&self) -> PathBuf {
        self.inner.path().join("computed_version")
    }

    fn validate_computed_version_or_reset_file(&mut self, version: Version) -> Result<()> {
        let path = self.path_computed_version();
        if version.validate(path.as_ref()).is_err() {
            self.inner.reset()?;
        }
        version.write(path.as_ref())?;

        if self.is_empty() {
            info!("Computing {}...", self.file_name())
        }

        Ok(())
    }

    pub fn compute_range<A, F>(
        &mut self,
        max_from: I,
        other: &mut StoredVec<I, A>,
        mut t: F,
        exit: &Exit,
    ) -> Result<()>
    where
        A: StoredType,
        F: FnMut(I) -> (I, T),
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + other.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        (index.to_usize()?..other.len()).try_for_each(|i| {
            let (i, v) = t(I::from(i));
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_transform<A, B, F>(
        &mut self,
        max_from: A,
        other: &mut StoredVec<A, B>,
        mut t: F,
        exit: &Exit,
    ) -> Result<()>
    where
        A: StoredIndex,
        B: StoredType,
        F: FnMut((A, B, &mut Self, &mut dyn DynamicVec<I = A, T = B>)) -> (I, T),
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + other.version(),
        )?;

        let index = max_from.min(A::from(self.len()));
        other.iter_from(index, |(a, b, other)| {
            let (i, v) = t((a, b, self, other));
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_inverse_more_to_less(
        &mut self,
        max_from: T,
        other: &mut StoredVec<T, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType + StoredIndex,
        T: StoredIndex,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + other.version(),
        )?;

        let index = max_from.min(
            self.inner
                .cached_get_last()?
                .map_or_else(T::default, |v| v.into_inner()),
        );
        other.iter_from(index, |(v, i, ..)| {
            if self.unwrap_cached_get(i).is_none_or(|old_v| old_v > v) {
                self.forced_push_at(i, v, exit)
            } else {
                Ok(())
            }
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_inverse_less_to_more(
        &mut self,
        max_from: T,
        first_indexes: &mut StoredVec<T, I>,
        last_indexes: &mut StoredVec<T, I>,
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
            let last_index = (last_indexes.double_unwrap_cached_get(value)).to_usize()?;
            (first_index..last_index)
                .try_for_each(|index| self.forced_push_at(I::from(index), value, exit))
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_last_index_from_first(
        &mut self,
        max_from: I,
        first_indexes: &mut StoredVec<I, T>,
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
        first_indexes.iter_from(index, |(index, v, ..)| {
            if let Some(prev_index) = prev_index.take() {
                let value = v.checked_sub(one).unwrap();
                self.forced_push_at(prev_index, value, exit)?;
            }
            prev_index.replace(index);
            Ok(())
        })?;
        if let Some(prev_index) = prev_index {
            self.forced_push_at(
                prev_index,
                T::from(final_len).checked_sub(one).unwrap(),
                exit,
            )?;
        }

        self.safe_flush(exit)
    }

    pub fn compute_count_from_indexes<T2>(
        &mut self,
        max_from: I,
        first_indexes: &mut StoredVec<I, T2>,
        last_indexes: &mut StoredVec<I, T2>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<T2>,
        T2: StoredType
            + StoredIndex
            + Copy
            + Add<usize, Output = T2>
            + CheckedSub<T2>
            + TryInto<T>
            + Default,
        <T2 as TryInto<T>>::Error: error::Error + 'static,
    {
        let opt: Option<Box<dyn FnMut(T2) -> bool>> = None;
        self.compute_filtered_count_from_indexes_(max_from, first_indexes, last_indexes, opt, exit)
    }

    pub fn compute_filtered_count_from_indexes<T2, F>(
        &mut self,
        max_from: I,
        first_indexes: &mut StoredVec<I, T2>,
        last_indexes: &mut StoredVec<I, T2>,
        filter: F,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<T2>,
        T2: StoredType
            + StoredIndex
            + Copy
            + Add<usize, Output = T2>
            + CheckedSub<T2>
            + TryInto<T>
            + Default,
        <T2 as TryInto<T>>::Error: error::Error + 'static,
        F: FnMut(T2) -> bool,
    {
        self.compute_filtered_count_from_indexes_(
            max_from,
            first_indexes,
            last_indexes,
            Some(Box::new(filter)),
            exit,
        )
    }

    fn compute_filtered_count_from_indexes_<T2>(
        &mut self,
        max_from: I,
        first_indexes: &mut StoredVec<I, T2>,
        last_indexes: &mut StoredVec<I, T2>,
        mut filter: Option<Box<dyn FnMut(T2) -> bool + '_>>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<T2>,
        T2: StoredType
            + StoredIndex
            + Copy
            + Add<usize, Output = T2>
            + CheckedSub<T2>
            + TryInto<T>
            + Default,
        <T2 as TryInto<T>>::Error: error::Error + 'static,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + first_indexes.version() + last_indexes.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        first_indexes.iter_from(index, |(i, first_index, ..)| {
            let last_index = last_indexes.double_unwrap_cached_get(i);
            let range = first_index.unwrap_to_usize()..=last_index.unwrap_to_usize();
            let count = if let Some(filter) = filter.as_mut() {
                range.into_iter().filter(|i| filter(T2::from(*i))).count()
            } else {
                range.count()
            };
            self.forced_push_at(i, T::from(T2::from(count)), exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_is_first_ordered<A>(
        &mut self,
        max_from: I,
        self_to_other: &mut StoredVec<I, A>,
        other_to_self: &mut StoredVec<A, I>,
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
            self.forced_push_at(
                i,
                T::from(other_to_self.double_unwrap_cached_get(other) == i),
                exit,
            )
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_sum_from_indexes<T2>(
        &mut self,
        max_from: I,
        first_indexes: &mut StoredVec<I, T2>,
        last_indexes: &mut StoredVec<I, T2>,
        source: &mut StoredVec<T2, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<usize> + Add<T, Output = T>,
        T2: StoredIndex + StoredType,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + first_indexes.version() + last_indexes.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        first_indexes.iter_from(index, |(i, first_index, ..)| {
            let last_index = last_indexes.double_unwrap_cached_get(i);
            let range = first_index.unwrap_to_usize()..=last_index.unwrap_to_usize();
            let mut sum = T::from(0_usize);
            range.into_iter().for_each(|i| {
                sum = sum.clone() + source.double_unwrap_cached_get(T2::from(i));
            });
            self.forced_push_at(i, sum, exit)
        })?;

        self.safe_flush(exit)
    }
}

impl<I> EagerVec<I, Bitcoin>
where
    I: StoredIndex,
{
    pub fn compute_from_sats(
        &mut self,
        max_from: I,
        sats: &mut StoredVec<I, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + sats.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        sats.iter_from(index, |(i, sats, ..)| {
            let (i, v) = (i, Bitcoin::from(sats));
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }
}

impl EagerVec<Height, Dollars> {
    pub fn compute_from_bitcoin(
        &mut self,
        max_from: Height,
        bitcoin: &mut StoredVec<Height, Bitcoin>,
        price: &mut StoredVec<Height, Close<Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + bitcoin.version(),
        )?;

        let index = max_from.min(Height::from(self.len()));
        bitcoin.iter_from(index, |(i, bitcoin, ..)| {
            let dollars = price.double_unwrap_cached_get(i);
            let (i, v) = (i, *dollars * bitcoin);
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }
}

impl EagerVec<Txindex, Dollars> {
    pub fn compute_from_bitcoin(
        &mut self,
        max_from: Txindex,
        bitcoin: &mut StoredVec<Txindex, Bitcoin>,
        i_to_height: &mut StoredVec<Txindex, Height>,
        price: &mut StoredVec<Height, Close<Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + bitcoin.version(),
        )?;

        let index = max_from.min(Txindex::from(self.len()));
        bitcoin.iter_from(index, |(i, bitcoin, ..)| {
            let height = i_to_height.double_unwrap_cached_get(i);
            let dollars = price.double_unwrap_cached_get(height);
            let (i, v) = (i, *dollars * bitcoin);
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }
}

impl<I, T> Clone for EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn clone(&self) -> Self {
        Self {
            computed_version: self.computed_version,
            inner: self.inner.clone(),
        }
    }
}
