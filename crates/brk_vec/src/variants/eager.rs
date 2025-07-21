use core::error;
use std::{
    borrow::Cow,
    cmp::Ordering,
    f32,
    fmt::Debug,
    ops::{Add, Div, Mul},
    path::{Path, PathBuf},
};

use brk_core::{
    Bitcoin, CheckedSub, Close, Date, DateIndex, Dollars, Error, Result, Sats, StoredF32,
    StoredUsize, Version,
};
use brk_exit::Exit;
use log::info;
use memmap2::Mmap;

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BoxedVecIterator, CollectableVec, Format,
    GenericStoredVec, StoredIndex, StoredType, StoredVec, StoredVecIterator, VecIterator,
};

const ONE_KIB: usize = 1024;
const ONE_MIB: usize = ONE_KIB * ONE_KIB;
const MAX_CACHE_SIZE: usize = 256 * ONE_MIB;
const DCA_AMOUNT: Dollars = Dollars::mint(100.0);

#[derive(Debug, Clone)]
pub struct EagerVec<I, T>(StoredVec<I, T>);
// computed_version: Arc<ArcSwap<Option<Version>>>,

impl<I, T> EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    const SIZE_OF: usize = size_of::<T>();

    pub fn forced_import(
        path: &Path,
        value_name: &str,
        version: Version,
        format: Format,
    ) -> Result<Self> {
        Ok(Self(StoredVec::forced_import(
            path, value_name, version, format,
        )?))
    }

    fn safe_truncate_if_needed(&mut self, index: I, exit: &Exit) -> Result<()> {
        let _lock = exit.lock();
        self.0.truncate_if_needed(index)?;
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
                self.0.push(value);
            }
        }

        if self.0.pushed_len() * Self::SIZE_OF >= MAX_CACHE_SIZE {
            self.safe_flush(exit)
        } else {
            Ok(())
        }
    }

    pub fn safe_flush(&mut self, exit: &Exit) -> Result<()> {
        let _lock = exit.lock();
        self.0.flush()?;
        Ok(())
    }

    pub fn path(&self) -> PathBuf {
        self.0.path()
    }

    pub fn get_or_read(&self, index: I, mmap: &Mmap) -> Result<Option<Cow<T>>> {
        self.0.get_or_read(index, mmap)
    }

    pub fn inner_version(&self) -> Version {
        self.0.version()
    }

    fn update_computed_version(&mut self, computed_version: Version) {
        self.0
            .mut_header()
            .update_computed_version(computed_version);
    }

    pub fn validate_computed_version_or_reset_file(&mut self, version: Version) -> Result<()> {
        if version != self.0.header().computed_version() {
            self.update_computed_version(version);
            if !self.is_empty() {
                self.0.reset()?;
            }
        }

        if self.is_empty() {
            info!(
                "Computing {}_to_{}...",
                self.index_type_to_string(),
                self.name()
            )
        }

        Ok(())
    }

    pub fn compute_to<F>(
        &mut self,
        max_from: I,
        to: usize,
        version: Version,
        mut t: F,
        exit: &Exit,
    ) -> Result<()>
    where
        F: FnMut(I) -> (I, T),
    {
        self.validate_computed_version_or_reset_file(Version::ZERO + self.0.version() + version)?;

        let index = max_from.min(I::from(self.len()));
        (index.to_usize()?..to).try_for_each(|i| {
            let (i, v) = t(I::from(i));
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_range<A, F>(
        &mut self,
        max_from: I,
        other: &impl AnyIterableVec<I, A>,
        t: F,
        exit: &Exit,
    ) -> Result<()>
    where
        A: StoredType,
        F: FnMut(I) -> (I, T),
    {
        self.compute_to(max_from, other.len(), other.version(), t, exit)
    }

    pub fn compute_from_index<T2>(
        &mut self,
        max_from: I,
        other: &impl AnyIterableVec<I, T2>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<I>,
        T2: StoredType,
    {
        self.compute_to(
            max_from,
            other.len(),
            other.version(),
            |i| (i, T::from(i)),
            exit,
        )
    }

    pub fn compute_transform<A, B, F>(
        &mut self,
        max_from: A,
        other: &impl AnyIterableVec<A, B>,
        mut t: F,
        exit: &Exit,
    ) -> Result<()>
    where
        A: StoredIndex,
        B: StoredType,
        F: FnMut((A, B, &Self)) -> (I, T),
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + other.version(),
        )?;

        let index = max_from.min(A::from(self.len()));
        other.iter_at(index).try_for_each(|(a, b)| {
            let (i, v) = t((a, b.into_owned(), self));
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_add(
        &mut self,
        max_from: I,
        added: &impl AnyIterableVec<I, T>,
        adder: &impl AnyIterableVec<I, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Add<Output = T>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + added.version() + adder.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let mut adder_iter = adder.iter();

        added.iter_at(index).try_for_each(|(i, v)| {
            let v = v.into_owned() + adder_iter.unwrap_get_inner(i);

            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_subtract(
        &mut self,
        max_from: I,
        subtracted: &impl AnyIterableVec<I, T>,
        subtracter: &impl AnyIterableVec<I, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: CheckedSub,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + subtracted.version() + subtracter.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let mut subtracter_iter = subtracter.iter();

        subtracted.iter_at(index).try_for_each(|(i, v)| {
            let v = v
                .into_owned()
                .checked_sub(subtracter_iter.unwrap_get_inner(i))
                .unwrap();

            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_max<T2>(
        &mut self,
        max_from: I,
        source: &impl AnyIterableVec<I, T2>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<T2> + Ord,
        T2: StoredType,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + source.version(),
        )?;

        let index = max_from.min(I::from(self.len()));

        let mut prev = None;

        source.iter_at(index).try_for_each(|(i, v)| {
            if prev.is_none() {
                let i = i.unwrap_to_usize();
                prev.replace(if i > 0 {
                    self.into_iter().unwrap_get_inner_(i - 1)
                } else {
                    T::from(source.iter().unwrap_get_inner_(0))
                });
            }
            let max = prev.clone().unwrap().max(T::from(v.into_owned()));
            prev.replace(max.clone());

            self.forced_push_at(i, max, exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_multiply<T2, T3, T4>(
        &mut self,
        max_from: I,
        multiplied: &impl AnyIterableVec<I, T2>,
        multiplier: &impl AnyIterableVec<I, T3>,
        exit: &Exit,
    ) -> Result<()>
    where
        T2: StoredType + Mul<T3, Output = T4>,
        T3: StoredType,
        T4: StoredType,
        T: From<T4>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + multiplied.version() + multiplier.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let mut multiplier_iter = multiplier.iter();

        multiplied.iter_at(index).try_for_each(|(i, v)| {
            let v = v.into_owned() * multiplier_iter.unwrap_get_inner(i);

            self.forced_push_at(i, v.into(), exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_divide<T2, T3, T4, T5>(
        &mut self,
        max_from: I,
        divided: &impl AnyIterableVec<I, T2>,
        divider: &impl AnyIterableVec<I, T3>,
        exit: &Exit,
    ) -> Result<()>
    where
        T2: StoredType + Mul<usize, Output = T4>,
        T3: StoredType,
        T4: Div<T3, Output = T5> + From<T2>,
        T5: CheckedSub<usize>,
        T: From<T5>,
    {
        self.compute_divide_(max_from, divided, divider, exit, false, false)
    }

    pub fn compute_percentage<T2, T3, T4, T5>(
        &mut self,
        max_from: I,
        divided: &impl AnyIterableVec<I, T2>,
        divider: &impl AnyIterableVec<I, T3>,
        exit: &Exit,
    ) -> Result<()>
    where
        T2: StoredType + Mul<usize, Output = T4>,
        T3: StoredType,
        T4: Div<T3, Output = T5> + From<T2>,
        T5: CheckedSub<usize>,
        T: From<T5>,
    {
        self.compute_divide_(max_from, divided, divider, exit, true, false)
    }

    pub fn compute_percentage_difference<T2, T3, T4, T5>(
        &mut self,
        max_from: I,
        divided: &impl AnyIterableVec<I, T2>,
        divider: &impl AnyIterableVec<I, T3>,
        exit: &Exit,
    ) -> Result<()>
    where
        T2: StoredType + Mul<usize, Output = T4>,
        T3: StoredType,
        T4: Div<T3, Output = T5> + From<T2>,
        T5: CheckedSub<usize>,
        T: From<T5>,
    {
        self.compute_divide_(max_from, divided, divider, exit, true, true)
    }

    pub fn compute_divide_<T2, T3, T4, T5>(
        &mut self,
        max_from: I,
        divided: &impl AnyIterableVec<I, T2>,
        divider: &impl AnyIterableVec<I, T3>,
        exit: &Exit,
        as_percentage: bool,
        as_difference: bool,
    ) -> Result<()>
    where
        T2: StoredType + Mul<usize, Output = T4>,
        T3: StoredType,
        T4: Div<T3, Output = T5> + From<T2>,
        T5: CheckedSub<usize>,
        T: From<T5>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ONE + self.0.version() + divided.version() + divider.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let multiplier = if as_percentage { 100 } else { 1 };

        let mut divider_iter = divider.iter();
        divided.iter_at(index).try_for_each(|(i, divided)| {
            let divided = divided.into_owned();
            let divider = divider_iter.unwrap_get_inner(i);

            let v = if as_percentage {
                divided * multiplier
            } else {
                T4::from(divided)
            };
            let mut v = v / divider;
            if as_difference {
                v = v.checked_sub(multiplier).unwrap();
            }
            self.forced_push_at(i, T::from(v), exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_drawdown(
        &mut self,
        max_from: I,
        close: &impl AnyIterableVec<I, Close<Dollars>>,
        ath: &impl AnyIterableVec<I, Dollars>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<StoredF32>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + ath.version() + close.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let mut close_iter = close.iter();
        ath.iter_at(index).try_for_each(|(i, ath)| {
            let ath = ath.into_owned();
            if ath == Dollars::ZERO {
                self.forced_push_at(i, T::from(StoredF32::default()), exit)
            } else {
                let close = *close_iter.unwrap_get_inner(i);
                let drawdown = StoredF32::from((*ath - *close) / *ath * -100.0);
                self.forced_push_at(i, T::from(drawdown), exit)
            }
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_inverse_more_to_less(
        &mut self,
        max_from: T,
        other: &impl AnyIterableVec<T, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType + StoredIndex,
        T: StoredIndex,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + other.version(),
        )?;

        let index = max_from.min(
            VecIterator::last(self.0.into_iter()).map_or_else(T::default, |(_, v)| v.into_owned()),
        );
        let mut prev_i = None;
        other.iter_at(index).try_for_each(|(v, i)| -> Result<()> {
            let i = i.into_owned();
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
        first_indexes: &impl AnyIterableVec<T, I>,
        indexes_count: &impl AnyIterableVec<T, StoredUsize>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType,
        T: StoredIndex,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + first_indexes.version() + indexes_count.version(),
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
        first_indexes: &impl AnyIterableVec<I, T2>,
        other_to_else: &impl AnyIterableVec<T2, T3>,
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
        first_indexes: &impl AnyIterableVec<I, T2>,
        other_to_else: &impl AnyIterableVec<T2, T3>,
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
        first_indexes: &impl AnyIterableVec<I, T2>,
        other_to_else: &impl AnyIterableVec<T2, T3>,
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
            Version::ZERO + self.0.version() + first_indexes.version() + other_to_else.version(),
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
        self_to_other: &impl AnyIterableVec<I, A>,
        other_to_self: &impl AnyIterableVec<A, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType,
        T: From<bool>,
        A: StoredIndex + StoredType,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + self_to_other.version() + other_to_self.version(),
        )?;

        let mut other_to_self_iter = other_to_self.iter();
        let index = max_from.min(I::from(self.len()));
        self_to_other.iter_at(index).try_for_each(|(i, other)| {
            self.forced_push_at(
                i,
                T::from(other_to_self_iter.unwrap_get_inner(other.into_owned()) == i),
                exit,
            )
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_sum_from_indexes<T2>(
        &mut self,
        max_from: I,
        first_indexes: &impl AnyIterableVec<I, T2>,
        indexes_count: &impl AnyIterableVec<I, StoredUsize>,
        source: &impl AnyIterableVec<T2, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<usize> + Add<T, Output = T>,
        T2: StoredIndex + StoredType,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + first_indexes.version() + indexes_count.version(),
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

    pub fn compute_sum_of_others(
        &mut self,
        max_from: I,
        others: &[&impl AnyIterableVec<I, T>],
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<usize> + Add<T, Output = T>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + others.iter().map(|v| v.version()).sum(),
        )?;

        if others.is_empty() {
            unreachable!("others should've length of 1 at least");
        }

        let mut others_iter = others[1..].iter().map(|v| v.iter()).collect::<Vec<_>>();

        let index = max_from.min(I::from(self.len()));
        others
            .first()
            .unwrap()
            .iter_at(index)
            .try_for_each(|(i, v)| {
                let mut sum = v.into_owned();
                others_iter.iter_mut().for_each(|iter| {
                    sum = sum.clone() + iter.unwrap_get_inner(i);
                });
                self.forced_push_at(i, sum, exit)
            })?;

        self.safe_flush(exit)
    }

    pub fn compute_min_of_others(
        &mut self,
        max_from: I,
        others: &[&impl AnyIterableVec<I, T>],
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<usize> + Add<T, Output = T> + Ord,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + others.iter().map(|v| v.version()).sum(),
        )?;

        if others.is_empty() {
            unreachable!("others should've length of 1 at least");
        }

        let mut others_iter = others[1..].iter().map(|v| v.iter()).collect::<Vec<_>>();

        let index = max_from.min(I::from(self.len()));
        others
            .first()
            .unwrap()
            .iter_at(index)
            .try_for_each(|(i, v)| {
                let min = v.into_owned();
                let min = others_iter
                    .iter_mut()
                    .map(|iter| iter.unwrap_get_inner(i))
                    .min()
                    .map_or(min.clone(), |min2| min.min(min2));
                self.forced_push_at(i, min, exit)
            })?;

        self.safe_flush(exit)
    }

    pub fn compute_max_of_others(
        &mut self,
        max_from: I,
        others: &[&impl AnyIterableVec<I, T>],
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<usize> + Add<T, Output = T> + Ord,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + others.iter().map(|v| v.version()).sum(),
        )?;

        if others.is_empty() {
            unreachable!("others should've length of 1 at least");
        }

        let mut others_iter = others[1..].iter().map(|v| v.iter()).collect::<Vec<_>>();

        let index = max_from.min(I::from(self.len()));
        others
            .first()
            .unwrap()
            .iter_at(index)
            .try_for_each(|(i, v)| {
                let max = v.into_owned();
                let max = others_iter
                    .iter_mut()
                    .map(|iter| iter.unwrap_get_inner(i))
                    .max()
                    .map_or(max.clone(), |max2| max.max(max2));
                self.forced_push_at(i, max, exit)
            })?;

        self.safe_flush(exit)
    }

    pub fn compute_sma<T2>(
        &mut self,
        max_from: I,
        source: &impl AnyIterableVec<I, T2>,
        sma: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Add<T, Output = T> + From<T2> + Div<usize, Output = T> + From<f32>,
        T2: StoredType,
        f32: From<T> + From<T2>,
    {
        self.compute_sma_(max_from, source, sma, exit, None)
    }

    pub fn compute_sma_<T2>(
        &mut self,
        max_from: I,
        source: &impl AnyIterableVec<I, T2>,
        sma: usize,
        exit: &Exit,
        min_i: Option<I>,
    ) -> Result<()>
    where
        T: Add<T, Output = T> + From<T2> + Div<usize, Output = T> + From<f32>,
        T2: StoredType,
        f32: From<T> + From<T2>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ONE + self.0.version() + source.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let mut prev = None;
        let min_prev_i = min_i.unwrap_or_default().unwrap_to_usize();
        let mut other_iter = source.iter();
        source.iter_at(index).try_for_each(|(i, value)| {
            let value = value.into_owned();

            if min_i.is_none() || min_i.is_some_and(|min_i| min_i <= i) {
                if prev.is_none() {
                    let i = i.unwrap_to_usize();
                    prev.replace(if i > min_prev_i {
                        self.into_iter().unwrap_get_inner_(i - 1)
                    } else {
                        T::from(0.0)
                    });
                }

                let processed_values_count = i.unwrap_to_usize() - min_prev_i + 1;
                let len = (processed_values_count).min(sma);

                let value = f32::from(value);

                let sma = T::from(if processed_values_count > sma {
                    let prev_sum = f32::from(prev.clone().unwrap()) * len as f32;
                    let value_to_subtract = f32::from(
                        other_iter.unwrap_get_inner_(i.unwrap_to_usize().checked_sub(sma).unwrap()),
                    );
                    (prev_sum - value_to_subtract + value) / len as f32
                } else {
                    (f32::from(prev.clone().unwrap()) * (len - 1) as f32 + value) / len as f32
                });

                prev.replace(sma.clone());
                self.forced_push_at(i, sma, exit)
            } else {
                self.forced_push_at(i, T::from(f32::NAN), exit)
            }
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_previous_value<T2>(
        &mut self,
        max_from: I,
        source: &impl AnyIterableVec<I, T2>,
        len: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        I: CheckedSub,
        T2: StoredType + Default,
        f32: From<T2>,
        T: From<f32>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + source.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let mut source_iter = source.iter();
        (index.to_usize()?..source.len()).try_for_each(|i| {
            let i = I::from(i);

            let previous_value = i
                .checked_sub(I::from(len))
                .map(|prev_i| f32::from(source_iter.unwrap_get_inner(prev_i)))
                .unwrap_or(f32::NAN);

            self.forced_push_at(i, T::from(previous_value), exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_change(
        &mut self,
        max_from: I,
        source: &impl AnyIterableVec<I, T>,
        len: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        I: CheckedSub,
        T: CheckedSub + Default,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + source.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let mut source_iter = source.iter();
        source.iter_at(index).try_for_each(|(i, current)| {
            let current = current.into_owned();

            let prev = i
                .checked_sub(I::from(len))
                .map(|prev_i| source_iter.unwrap_get_inner(prev_i))
                .unwrap_or_default();

            self.forced_push_at(i, current.checked_sub(prev).unwrap(), exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_percentage_change<T2>(
        &mut self,
        max_from: I,
        source: &impl AnyIterableVec<I, T2>,
        len: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        I: CheckedSub,
        T2: StoredType + Default,
        f32: From<T2>,
        T: From<f32>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + source.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let mut source_iter = source.iter();
        source.iter_at(index).try_for_each(|(i, b)| {
            let previous_value = f32::from(
                i.checked_sub(I::from(len))
                    .map(|prev_i| source_iter.unwrap_get_inner(prev_i))
                    .unwrap_or_default(),
            );

            let last_value = f32::from(b.into_owned());

            let percentage_change = ((last_value / previous_value) - 1.0) * 100.0;

            self.forced_push_at(i, T::from(percentage_change), exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_cagr<T2>(
        &mut self,
        max_from: I,
        percentage_returns: &impl AnyIterableVec<I, T2>,
        days: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        I: CheckedSub,
        T2: StoredType + Default,
        f32: From<T2>,
        T: From<f32>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + percentage_returns.version(),
        )?;

        if days % 365 != 0 {
            panic!("bad days");
        }

        let years = days / 365;
        let index = max_from.min(I::from(self.len()));
        percentage_returns
            .iter_at(index)
            .try_for_each(|(i, percentage)| {
                let percentage = percentage.into_owned();

                let cagr = (((f32::from(percentage) / 100.0 + 1.0).powf(1.0 / years as f32)) - 1.0)
                    * 100.0;

                self.forced_push_at(i, T::from(cagr), exit)
            })?;

        self.safe_flush(exit)
    }

    pub fn compute_zscore(
        &mut self,
        max_from: I,
        ratio: &impl AnyIterableVec<I, StoredF32>,
        sma: &impl AnyIterableVec<I, StoredF32>,
        sd: &impl AnyIterableVec<I, StoredF32>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<StoredF32>,
    {
        let mut sma_iter = sma.iter();
        let mut sd_iter = sd.iter();

        self.compute_transform(
            max_from,
            ratio,
            |(i, ratio, ..)| {
                let sma = sma_iter.unwrap_get_inner(i);
                let sd = sd_iter.unwrap_get_inner(i);
                (i, T::from((ratio - sma) / sd))
            },
            exit,
        )
    }
}

impl EagerVec<DateIndex, Sats> {
    pub fn compute_dca_stack_via_len(
        &mut self,
        max_from: DateIndex,
        closes: &impl AnyIterableVec<DateIndex, Close<Dollars>>,
        len: usize,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + closes.version(),
        )?;

        let mut other_iter = closes.iter();
        let mut prev = None;

        let index = max_from.min(DateIndex::from(self.len()));
        closes.iter_at(index).try_for_each(|(i, closes)| {
            let price = *closes.into_owned();
            let i_usize = i.unwrap_to_usize();
            if prev.is_none() {
                if i_usize == 0 {
                    prev.replace(Sats::ZERO);
                } else {
                    prev.replace(self.into_iter().unwrap_get_inner_(i_usize - 1));
                }
            }

            let mut stack = Sats::ZERO;

            if price != Dollars::ZERO {
                stack = prev.unwrap() + Sats::from(Bitcoin::from(DCA_AMOUNT / price));

                if i_usize >= len {
                    let prev_price = *other_iter.unwrap_get_inner_(i_usize - len);
                    if prev_price != Dollars::ZERO {
                        stack = stack
                            .checked_sub(Sats::from(Bitcoin::from(DCA_AMOUNT / prev_price)))
                            .unwrap();
                    }
                }
            }

            prev.replace(stack);

            self.forced_push_at(i, stack, exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_dca_stack_via_from(
        &mut self,
        max_from: DateIndex,
        closes: &impl AnyIterableVec<DateIndex, Close<Dollars>>,
        from: DateIndex,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + closes.version(),
        )?;

        let mut prev = None;

        let index = max_from.min(DateIndex::from(self.len()));
        closes.iter_at(index).try_for_each(|(i, closes)| {
            let price = *closes.into_owned();
            let i_usize = i.unwrap_to_usize();
            if prev.is_none() {
                if i_usize == 0 {
                    prev.replace(Sats::ZERO);
                } else {
                    prev.replace(self.into_iter().unwrap_get_inner_(i_usize - 1));
                }
            }

            let mut stack = Sats::ZERO;

            if price != Dollars::ZERO && i >= from {
                stack = prev.unwrap() + Sats::from(Bitcoin::from(DCA_AMOUNT / price));
            }

            prev.replace(stack);

            self.forced_push_at(i, stack, exit)
        })?;

        self.safe_flush(exit)
    }
}

impl EagerVec<DateIndex, Dollars> {
    pub fn compute_dca_avg_price_via_len(
        &mut self,
        max_from: DateIndex,
        stacks: &impl AnyIterableVec<DateIndex, Sats>,
        len: usize,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ONE + self.0.version() + stacks.version(),
        )?;

        let index = max_from.min(DateIndex::from(self.len()));

        let first_price_date = DateIndex::try_from(Date::new(2010, 7, 12)).unwrap();

        stacks.iter_at(index).try_for_each(|(i, stack)| {
            let stack = stack.into_owned();
            let mut avg_price = Dollars::from(f64::NAN);
            if i > first_price_date {
                avg_price = DCA_AMOUNT
                    * len
                        .min(i.unwrap_to_usize() + 1)
                        .min(i.checked_sub(first_price_date).unwrap().unwrap_to_usize() + 1)
                    / Bitcoin::from(stack);
            }
            self.forced_push_at(i, avg_price, exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_dca_avg_price_via_from(
        &mut self,
        max_from: DateIndex,
        stacks: &impl AnyIterableVec<DateIndex, Sats>,
        from: DateIndex,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + stacks.version(),
        )?;

        let index = max_from.min(DateIndex::from(self.len()));

        let from_usize = from.unwrap_to_usize();

        stacks.iter_at(index).try_for_each(|(i, stack)| {
            let stack = stack.into_owned();
            let mut avg_price = Dollars::from(f64::NAN);
            if i >= from {
                avg_price =
                    DCA_AMOUNT * (i.unwrap_to_usize() + 1 - from_usize) / Bitcoin::from(stack);
            }
            self.forced_push_at(i, avg_price, exit)
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
        sats: &impl AnyIterableVec<I, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + sats.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        sats.iter_at(index).try_for_each(|(i, sats)| {
            let (i, v) = (i, Bitcoin::from(sats.into_owned()));
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }
}

impl<I> EagerVec<I, Dollars>
where
    I: StoredIndex,
{
    pub fn compute_from_bitcoin(
        &mut self,
        max_from: I,
        bitcoin: &impl AnyIterableVec<I, Bitcoin>,
        price: &impl AnyIterableVec<I, Close<Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.0.version() + bitcoin.version(),
        )?;

        let mut price_iter = price.iter();
        let index = max_from.min(I::from(self.len()));
        bitcoin.iter_at(index).try_for_each(|(i, bitcoin)| {
            let dollars = price_iter.unwrap_get_inner(i);
            let (i, v) = (i, *dollars * bitcoin.into_owned());
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }
}

// impl EagerVec<TxIndex, Dollars> {
//     pub fn compute_txindex_from_bitcoin(
//         &mut self,
//         max_from: TxIndex,
//         bitcoin: &impl AnyIterableVec<TxIndex, Bitcoin>,
//         i_to_height: &impl AnyIterableVec<TxIndex, Height>,
//         price: &impl AnyIterableVec<Height, Close<Dollars>>,
//         exit: &Exit,
//     ) -> Result<()> {
//         self.validate_computed_version_or_reset_file(
//             Version::ZERO
//                 + self.0.version()
//                 + bitcoin.version()
//                 + i_to_height.version()
//                 + price.version(),
//         )?;

//         let mut i_to_height_iter = i_to_height.iter();
//         let mut price_iter = price.iter();
//         let index = max_from.min(TxIndex::from(self.len()));
//         bitcoin.iter_at(index).try_for_each(|(i, bitcoin, ..)| {
//             let height = i_to_height_iter.unwrap_get_inner(i);
//             let dollars = price_iter.unwrap_get_inner(height);
//             let (i, v) = (i, *dollars * bitcoin.into_owned());
//             self.forced_push_at(i, v, exit)
//         })?;

//         self.safe_flush(exit)
//     }
// }

impl<'a, I, T> IntoIterator for &'a EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type Item = (I, Cow<'a, T>);
    type IntoIter = StoredVecIterator<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<I, T> AnyVec for EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn version(&self) -> Version {
        self.0.header().computed_version()
    }

    #[inline]
    fn name(&self) -> &str {
        self.0.name()
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn index_type_to_string(&self) -> &'static str {
        I::to_string()
    }

    #[inline]
    fn value_type_to_size_of(&self) -> usize {
        size_of::<T>()
    }
}

impl<I, T> AnyIterableVec<I, T> for EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn boxed_iter<'a>(&'a self) -> BoxedVecIterator<'a, I, T>
    where
        I: StoredIndex,
        T: StoredType + 'a,
    {
        Box::new(self.0.into_iter())
    }
}

impl<I, T> AnyCollectableVec for EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn collect_range_serde_json(
        &self,
        from: Option<usize>,
        to: Option<usize>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
