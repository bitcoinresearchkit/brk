use core::error;
use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::Add,
    path::{Path, PathBuf},
};

use brk_core::{Bitcoin, CheckedSub, Close, Dollars, Height, Sats, StoredUsize, TxIndex};
use brk_exit::Exit;
use brk_vec::{
    Compressed, DynamicVec, Error, GenericVec, Result, StoredIndex, StoredType, StoredVec,
    StoredVecIterator, Value, Version,
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

    pub fn mut_vec(&mut self) -> &StoredVec<I, T> {
        &mut self.inner
    }

    pub fn any_vec(&self) -> &dyn brk_vec::AnyStoredVec {
        &self.inner
    }

    pub fn mut_any_vec(&mut self) -> &mut dyn brk_vec::AnyStoredVec {
        &mut self.inner
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

    pub fn iter(&self) -> StoredVecIterator<I, T> {
        self.into_iter()
    }

    pub fn compute_range<A, F>(
        &mut self,
        max_from: I,
        other: &StoredVec<I, A>,
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
        other: &StoredVec<A, B>,
        mut t: F,
        exit: &Exit,
    ) -> Result<()>
    where
        A: StoredIndex,
        B: StoredType,
        F: FnMut((A, B, &Self)) -> (I, T),
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + other.version(),
        )?;

        let index = max_from.min(A::from(self.len()));
        other.iter_at(index).try_for_each(|(a, b)| {
            let (i, v) = t((a, b.into_inner(), self));
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_inverse_more_to_less(
        &mut self,
        max_from: T,
        other: &StoredVec<T, I>,
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
                .iter()
                .last()
                .map_or_else(T::default, |(_, v)| v.into_inner()),
        );
        let mut prev_i = None;
        other.iter_at(index).try_for_each(|(v, i)| -> Result<()> {
            let i = i.into_inner();
            if prev_i.is_some_and(|prev_i| prev_i == i) {
                return Ok(());
            }
            if self.iter().get_inner(i).is_none_or(|old_v| old_v > v) {
                self.forced_push_at(i, v, exit)?;
            }
            prev_i.replace(i);
            Ok(())
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_inverse_less_to_more(
        &mut self,
        max_from: T,
        first_indexes: &StoredVec<T, I>,
        indexes_count: &StoredVec<T, StoredUsize>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType,
        T: StoredIndex,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + first_indexes.version() + indexes_count.version(),
        )?;

        let mut indexes_count_iter = indexes_count.iter();

        let index = max_from.min(T::from(self.len()));
        first_indexes
            .iter_at(index)
            .try_for_each(|(value, first_index)| {
                let first_index = (first_index).to_usize()?;
                let count = *indexes_count_iter.unwrap_get_inner(value);
                (first_index..first_index + count)
                    .try_for_each(|index| self.forced_push_at(I::from(index), value, exit))
            })?;

        self.safe_flush(exit)
    }

    pub fn compute_count_from_indexes<T2, T3>(
        &mut self,
        max_from: I,
        first_indexes: &StoredVec<I, T2>,
        other_to_else: &StoredVec<T2, T3>,
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
        T3: StoredType,
    {
        let opt: Option<Box<dyn FnMut(T2) -> bool>> = None;
        self.compute_filtered_count_from_indexes_(max_from, first_indexes, other_to_else, opt, exit)
    }

    pub fn compute_filtered_count_from_indexes<T2, T3, F>(
        &mut self,
        max_from: I,
        first_indexes: &StoredVec<I, T2>,
        other_to_else: &StoredVec<T2, T3>,
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
        T3: StoredType,
        F: FnMut(T2) -> bool,
    {
        self.compute_filtered_count_from_indexes_(
            max_from,
            first_indexes,
            other_to_else,
            Some(Box::new(filter)),
            exit,
        )
    }

    fn compute_filtered_count_from_indexes_<T2, T3>(
        &mut self,
        max_from: I,
        first_indexes: &StoredVec<I, T2>,
        other_to_else: &StoredVec<T2, T3>,
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
        T3: StoredType,
        <T2 as TryInto<T>>::Error: error::Error + 'static,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + first_indexes.version(), // + last_indexes.version(),
        )?;

        let mut other_iter = first_indexes.iter();
        let index = max_from.min(I::from(self.len()));
        first_indexes
            .iter_at(index)
            .try_for_each(|(i, first_index)| {
                let end = other_iter
                    .get_inner(i + 1)
                    .map(|v| v.unwrap_to_usize())
                    .unwrap_or_else(|| other_to_else.len());

                let range = first_index.unwrap_to_usize()..end;
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
        self_to_other: &StoredVec<I, A>,
        other_to_self: &StoredVec<A, I>,
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

        let mut other_to_self_iter = other_to_self.iter();
        let index = max_from.min(I::from(self.len()));
        self_to_other.iter_at(index).try_for_each(|(i, other)| {
            self.forced_push_at(
                i,
                T::from(other_to_self_iter.unwrap_get_inner(other.into_inner()) == i),
                exit,
            )
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_sum_from_indexes<T2>(
        &mut self,
        max_from: I,
        first_indexes: &StoredVec<I, T2>,
        indexes_count: &StoredVec<I, StoredUsize>,
        source: &StoredVec<T2, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<usize> + Add<T, Output = T>,
        T2: StoredIndex + StoredType,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + first_indexes.version() + indexes_count.version(),
        )?;

        let mut indexes_count_iter = indexes_count.iter();
        let mut source_iter = source.iter();
        let index = max_from.min(I::from(self.len()));
        first_indexes
            .iter_at(index)
            .try_for_each(|(i, first_index)| {
                let count = *indexes_count_iter.unwrap_get_inner(i);
                let first_index = first_index.unwrap_to_usize();
                let range = first_index..first_index + count;
                let mut sum = T::from(0_usize);
                range.into_iter().for_each(|i| {
                    sum = sum.clone() + source_iter.unwrap_get_inner(T2::from(i));
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
        sats: &StoredVec<I, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + sats.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        sats.iter_at(index).try_for_each(|(i, sats)| {
            let (i, v) = (i, Bitcoin::from(sats.into_inner()));
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }
}

impl EagerVec<Height, Dollars> {
    pub fn compute_from_bitcoin(
        &mut self,
        max_from: Height,
        bitcoin: &StoredVec<Height, Bitcoin>,
        price: &StoredVec<Height, Close<Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + bitcoin.version(),
        )?;

        let mut price_iter = price.iter();
        let index = max_from.min(Height::from(self.len()));
        bitcoin.iter_at(index).try_for_each(|(i, bitcoin)| {
            let dollars = price_iter.unwrap_get_inner(i);
            let (i, v) = (i, *dollars * bitcoin.into_inner());
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }
}

impl EagerVec<TxIndex, Dollars> {
    pub fn compute_from_bitcoin(
        &mut self,
        max_from: TxIndex,
        bitcoin: &StoredVec<TxIndex, Bitcoin>,
        i_to_height: &StoredVec<TxIndex, Height>,
        price: &StoredVec<Height, Close<Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.version() + bitcoin.version(),
        )?;

        let mut i_to_height_iter = i_to_height.iter();
        let mut price_iter = price.iter();
        let index = max_from.min(TxIndex::from(self.len()));
        bitcoin.iter_at(index).try_for_each(|(i, bitcoin, ..)| {
            let height = i_to_height_iter.unwrap_get_inner(i);
            let dollars = price_iter.unwrap_get_inner(height);
            let (i, v) = (i, *dollars * bitcoin.into_inner());
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

impl<'a, I, T> IntoIterator for &'a EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type Item = (I, Value<'a, T>);
    type IntoIter = StoredVecIterator<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
