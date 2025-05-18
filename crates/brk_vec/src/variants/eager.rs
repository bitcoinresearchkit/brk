use core::error;
use std::{
    cmp::Ordering,
    f32,
    fmt::Debug,
    ops::{Add, Div, Mul},
    path::{Path, PathBuf},
    time::Duration,
};

use brk_core::{
    Bitcoin, CheckedSub, Close, Date, DateIndex, Dollars, Height, Sats, StoredUsize, TxIndex,
};
use brk_exit::Exit;
use log::info;

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BoxedVecIterator, CollectableVec, Compressed, Error,
    GenericStoredVec, Result, StoredIndex, StoredType, StoredVec, StoredVecIterator, Value,
    VecIterator, Version,
};

const ONE_KIB: usize = 1024;
const ONE_MIB: usize = ONE_KIB * ONE_KIB;
const MAX_CACHE_SIZE: usize = 210 * ONE_MIB;
const DCA_AMOUNT: Dollars = Dollars::mint(100.0);

#[derive(Debug, Clone)]
pub struct EagerVec<I, T> {
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
        value_name: &str,
        version: Version,
        compressed: Compressed,
    ) -> Result<Self> {
        let inner = StoredVec::forced_import(path, value_name, version, compressed)?;

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

    pub fn path(&self) -> &Path {
        self.inner.path()
    }

    pub fn inner_version(&self) -> Version {
        self.inner.version()
    }

    #[inline]
    fn path_computed_version(&self) -> PathBuf {
        self.inner.path().join("computed_version")
    }

    pub fn validate_computed_version_or_reset_file(&mut self, version: Version) -> Result<()> {
        let path = self.path_computed_version();
        if version.validate(path.as_ref()).is_err() {
            self.inner.reset()?;
        }
        version.write(path.as_ref())?;
        self.computed_version = Some(version);

        if self.is_empty() {
            info!("Computing {}...", self.name())
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
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.inner.version() + version,
        )?;

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
            Version::ZERO + self.inner.version() + other.version(),
        )?;

        let index = max_from.min(A::from(self.len()));
        other.iter_at(index).try_for_each(|(a, b)| {
            let (i, v) = t((a, b.into_inner(), self));
            self.forced_push_at(i, v, exit)
        })?;

        self.safe_flush(exit)
    }

    pub fn compute_divide<T2, T3, T4>(
        &mut self,
        max_from: I,
        divided: &impl AnyIterableVec<I, T2>,
        divider: &impl AnyIterableVec<I, T3>,
        exit: &Exit,
    ) -> Result<()>
    where
        T2: StoredType + Div<T3, Output = T4>,
        T3: StoredType,
        T4: Mul<usize, Output = T4> + CheckedSub<usize>,
        T: From<T4>,
    {
        self.compute_divide_(max_from, divided, divider, exit, false, false)
    }

    pub fn compute_percentage<T2, T3, T4>(
        &mut self,
        max_from: I,
        divided: &impl AnyIterableVec<I, T2>,
        divider: &impl AnyIterableVec<I, T3>,
        exit: &Exit,
    ) -> Result<()>
    where
        T2: StoredType + Div<T3, Output = T4>,
        T3: StoredType,
        T4: Mul<usize, Output = T4> + CheckedSub<usize>,
        T: From<T4>,
    {
        self.compute_divide_(max_from, divided, divider, exit, true, false)
    }

    pub fn compute_percentage_difference<T2, T3, T4>(
        &mut self,
        max_from: I,
        divided: &impl AnyIterableVec<I, T2>,
        divider: &impl AnyIterableVec<I, T3>,
        exit: &Exit,
    ) -> Result<()>
    where
        T2: StoredType + Div<T3, Output = T4>,
        T3: StoredType,
        T4: Mul<usize, Output = T4> + CheckedSub<usize>,
        T: From<T4>,
    {
        self.compute_divide_(max_from, divided, divider, exit, true, true)
    }

    pub fn compute_divide_<T2, T3, T4>(
        &mut self,
        max_from: I,
        divided: &impl AnyIterableVec<I, T2>,
        divider: &impl AnyIterableVec<I, T3>,
        exit: &Exit,
        as_percentage: bool,
        as_difference: bool,
    ) -> Result<()>
    where
        T2: StoredType + Div<T3, Output = T4>,
        T3: StoredType,
        T4: Mul<usize, Output = T4> + CheckedSub<usize>,
        T: From<T4>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.inner.version() + divided.version() + divider.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let multiplier = if as_percentage { 100 } else { 1 };
        let subtract = if as_difference { multiplier } else { 0 };

        let mut divider_iter = divider.iter();
        divided.iter_at(index).try_for_each(|(i, divided)| {
            let v = (divided.into_inner() / divider_iter.unwrap_get_inner(i) * multiplier)
                .checked_sub(subtract)
                .unwrap();
            self.forced_push_at(i, T::from(v), exit)
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
            Version::ZERO + self.inner.version() + other.version(),
        )?;

        let index = max_from.min(
            VecIterator::last(self.inner.into_iter())
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
        first_indexes: &impl AnyIterableVec<T, I>,
        indexes_count: &impl AnyIterableVec<T, StoredUsize>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType,
        T: StoredIndex,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO
                + self.inner.version()
                + first_indexes.version()
                + indexes_count.version(),
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
            Version::ZERO + self.inner.version() + first_indexes.version(),
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
            Version::ZERO
                + self.inner.version()
                + self_to_other.version()
                + other_to_self.version(),
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
            Version::ZERO
                + self.inner.version()
                + first_indexes.version()
                + indexes_count.version(),
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
            Version::ZERO + self.inner.version() + source.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let mut prev = None;
        let min_prev_i = min_i.unwrap_or_default().unwrap_to_usize();
        source.iter_at(index).try_for_each(|(i, value)| {
            let value = value.into_inner();

            if min_i.is_none() || min_i.is_some_and(|min_i| min_i <= i) {
                if prev.is_none() {
                    let i = i.unwrap_to_usize();
                    prev.replace(if i > min_prev_i {
                        self.into_iter().unwrap_get_inner_(i - 1)
                    } else {
                        T::from(0.0)
                    });
                }
                let len = (i.unwrap_to_usize() - min_prev_i + 1).min(sma);
                let sma = T::from(
                    (f32::from(prev.clone().unwrap()) * (len - 1) as f32 + f32::from(value))
                        / len as f32,
                );

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
            Version::ZERO + self.inner.version() + source.version(),
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
            Version::ZERO + self.inner.version() + source.version(),
        )?;

        let index = max_from.min(I::from(self.len()));
        let mut source_iter = source.iter();
        source.iter_at(index).try_for_each(|(i, b)| {
            let previous_value = f32::from(
                i.checked_sub(I::from(len))
                    .map(|prev_i| source_iter.unwrap_get_inner(prev_i))
                    .unwrap_or_default(),
            );

            let last_value = f32::from(b.into_inner());

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
            Version::ZERO + self.inner.version() + percentage_returns.version(),
        )?;

        if days % 365 != 0 {
            panic!("bad days");
        }

        let years = days / 365;
        let index = max_from.min(I::from(self.len()));
        percentage_returns
            .iter_at(index)
            .try_for_each(|(i, percentage)| {
                let percentage = percentage.into_inner();

                let cagr = (((f32::from(percentage) / 100.0 + 1.0).powf(1.0 / years as f32)) - 1.0)
                    * 100.0;

                self.forced_push_at(i, T::from(cagr), exit)
            })?;

        self.safe_flush(exit)
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
            Version::new(5) + self.inner.version() + closes.version(),
        )?;

        let mut other_iter = closes.iter();
        let mut prev = None;

        let index = max_from.min(DateIndex::from(self.len()));
        closes.iter_at(index).try_for_each(|(i, closes)| {
            let price = *closes.into_inner();
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
            Version::ZERO + self.inner.version() + closes.version(),
        )?;

        let mut prev = None;

        let index = max_from.min(DateIndex::from(self.len()));
        closes.iter_at(index).try_for_each(|(i, closes)| {
            let price = *closes.into_inner();
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
            Version::TWO + self.inner.version() + stacks.version(),
        )?;

        let index = max_from.min(DateIndex::from(self.len()));

        let first_price_date = DateIndex::try_from(Date::new(2010, 8, 16)).unwrap();

        stacks.iter_at(index).try_for_each(|(i, stack)| {
            let stack = stack.into_inner();
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
            Version::ZERO + self.inner.version() + stacks.version(),
        )?;

        let index = max_from.min(DateIndex::from(self.len()));

        let from_usize = from.unwrap_to_usize();

        stacks.iter_at(index).try_for_each(|(i, stack)| {
            let stack = stack.into_inner();
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
            Version::ZERO + self.inner.version() + sats.version(),
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
        bitcoin: &impl AnyIterableVec<Height, Bitcoin>,
        price: &impl AnyIterableVec<Height, Close<Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.inner.version() + bitcoin.version(),
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
        bitcoin: &impl AnyIterableVec<TxIndex, Bitcoin>,
        i_to_height: &impl AnyIterableVec<TxIndex, Height>,
        price: &impl AnyIterableVec<Height, Close<Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.inner.version() + bitcoin.version(),
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

impl<I, T> AnyVec for EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn version(&self) -> Version {
        self.computed_version
            .or_else(|| {
                dbg!(self.path());
                None
            })
            .unwrap()
    }

    #[inline]
    fn name(&self) -> String {
        self.inner.name()
    }

    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    fn modified_time(&self) -> Result<Duration> {
        self.inner.modified_time()
    }

    #[inline]
    fn index_type_to_string(&self) -> String {
        I::to_string()
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
        Box::new(self.inner.into_iter())
    }
}

impl<I, T> AnyCollectableVec for EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn collect_range_serde_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
