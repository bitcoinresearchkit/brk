use core::error;
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    f32,
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
    sync::Arc,
};

use log::info;

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyStoredVec, AnyVec, BoxedVecIterator, CheckedSub,
    CollectableVec, Exit, File, Format, GenericStoredVec, Reader, Result, StoredCompressed,
    StoredIndex, StoredRaw, StoredVec, StoredVecIterator, VecIterator, Version, variants::Header,
};

#[derive(Debug, Clone)]
pub struct EagerVec<I, T>(StoredVec<I, T>);

impl<I, T> EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredCompressed,
{
    pub fn forced_import(
        file: &Arc<File>,
        value_name: &str,
        version: Version,
        format: Format,
    ) -> Result<Self> {
        Ok(Self(StoredVec::forced_import(
            file, value_name, version, format,
        )?))
    }

    #[inline]
    pub fn get_or_read<'a, 'b>(&'a self, index: I, reader: &'b Reader) -> Result<Option<Cow<'b, T>>>
    where
        'a: 'b,
    {
        self.0.get_or_read(index, reader)
    }

    pub fn inner_version(&self) -> Version {
        self.0.header().vec_version()
    }

    fn update_computed_version(&mut self, computed_version: Version) {
        self.mut_header().update_computed_version(computed_version);
    }

    pub fn validate_computed_version_or_reset_file(&mut self, version: Version) -> Result<()> {
        if version != self.header().computed_version() {
            self.update_computed_version(version);
            if !self.is_empty() {
                self.reset()?;
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
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.inner_version() + version,
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
        A: StoredRaw,
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
        T2: StoredRaw,
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
        B: StoredRaw,
        F: FnMut((A, B, &Self)) -> (I, T),
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.inner_version() + other.version(),
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
            Version::ZERO + self.inner_version() + added.version() + adder.version(),
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
            Version::ZERO + self.inner_version() + subtracted.version() + subtracter.version(),
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
        T2: StoredRaw,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.inner_version() + source.version(),
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
            let max = prev.unwrap().max(T::from(v.into_owned()));
            prev.replace(max);

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
        T2: StoredRaw + Mul<T3, Output = T4>,
        T3: StoredRaw,
        T4: StoredRaw,
        T: From<T4>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.inner_version() + multiplied.version() + multiplier.version(),
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
        T2: StoredRaw + Mul<usize, Output = T4>,
        T3: StoredRaw,
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
        T2: StoredRaw + Mul<usize, Output = T4>,
        T3: StoredRaw,
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
        T2: StoredRaw + Mul<usize, Output = T4>,
        T3: StoredRaw,
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
        T2: StoredRaw + Mul<usize, Output = T4>,
        T3: StoredRaw,
        T4: Div<T3, Output = T5> + From<T2>,
        T5: CheckedSub<usize>,
        T: From<T5>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ONE + self.inner_version() + divided.version() + divider.version(),
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

    pub fn compute_inverse_more_to_less(
        &mut self,
        max_from: T,
        other: &impl AnyIterableVec<T, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredRaw + StoredIndex,
        T: StoredIndex,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.inner_version() + other.version(),
        )?;

        let index = max_from.min(
            VecIterator::last(self.into_iter()).map_or_else(T::default, |(_, v)| v.into_owned()),
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

    pub fn compute_inverse_less_to_more<T2>(
        &mut self,
        max_from: T,
        first_indexes: &impl AnyIterableVec<T, I>,
        indexes_count: &impl AnyIterableVec<T, T2>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredRaw,
        T: StoredIndex,
        T2: StoredRaw,
        usize: From<T2>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO
                + self.inner_version()
                + first_indexes.version()
                + indexes_count.version(),
        )?;

        let mut indexes_count_iter = indexes_count.iter();

        let index = max_from.min(T::from(self.len()));
        first_indexes
            .iter_at(index)
            .try_for_each(|(value, first_index)| {
                let first_index = (first_index).to_usize()?;
                let count = usize::from(indexes_count_iter.unwrap_get_inner(value));
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
        T2: StoredRaw
            + StoredIndex
            + Copy
            + Add<usize, Output = T2>
            + CheckedSub<T2>
            + TryInto<T>
            + Default,
        <T2 as TryInto<T>>::Error: error::Error + 'static,
        T3: StoredRaw,
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
        T2: StoredRaw
            + StoredIndex
            + Copy
            + Add<usize, Output = T2>
            + CheckedSub<T2>
            + TryInto<T>
            + Default,
        <T2 as TryInto<T>>::Error: error::Error + 'static,
        T3: StoredRaw,
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
        T2: StoredRaw
            + StoredIndex
            + Copy
            + Add<usize, Output = T2>
            + CheckedSub<T2>
            + TryInto<T>
            + Default,
        T3: StoredRaw,
        <T2 as TryInto<T>>::Error: error::Error + 'static,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO
                + self.inner_version()
                + first_indexes.version()
                + other_to_else.version(),
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
        I: StoredRaw,
        T: From<bool>,
        A: StoredIndex + StoredRaw,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO
                + self.inner_version()
                + self_to_other.version()
                + other_to_self.version(),
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

    pub fn compute_sum_from_indexes<T2, T3>(
        &mut self,
        max_from: I,
        first_indexes: &impl AnyIterableVec<I, T2>,
        indexes_count: &impl AnyIterableVec<I, T3>,
        source: &impl AnyIterableVec<T2, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<usize> + Add<T, Output = T>,
        T2: StoredIndex + StoredRaw,
        T3: StoredRaw,
        usize: From<T3>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO
                + self.inner_version()
                + first_indexes.version()
                + indexes_count.version(),
        )?;

        let mut indexes_count_iter = indexes_count.iter();
        let mut source_iter = source.iter();
        let index = max_from.min(I::from(self.len()));
        first_indexes
            .iter_at(index)
            .try_for_each(|(i, first_index)| {
                let count = usize::from(indexes_count_iter.unwrap_get_inner(i));
                let first_index = first_index.unwrap_to_usize();
                let range = first_index..first_index + count;
                let mut sum = T::from(0_usize);
                range.into_iter().for_each(|i| {
                    sum = sum + source_iter.unwrap_get_inner(T2::from(i));
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
            Version::ZERO + self.inner_version() + others.iter().map(|v| v.version()).sum(),
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
                    sum = sum + iter.unwrap_get_inner(i);
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
            Version::ZERO + self.inner_version() + others.iter().map(|v| v.version()).sum(),
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
                    .map_or(min, |min2| min.min(min2));
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
            Version::ZERO + self.inner_version() + others.iter().map(|v| v.version()).sum(),
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
                    .map_or(max, |max2| max.max(max2));
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
        T2: StoredRaw,
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
        T2: StoredRaw,
        f32: From<T> + From<T2>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ONE + self.inner_version() + source.version(),
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
                    let prev_sum = f32::from(prev.unwrap()) * len as f32;
                    let value_to_subtract = f32::from(
                        other_iter.unwrap_get_inner_(i.unwrap_to_usize().checked_sub(sma).unwrap()),
                    );
                    (prev_sum - value_to_subtract + value) / len as f32
                } else {
                    (f32::from(prev.unwrap()) * (len - 1) as f32 + value) / len as f32
                });

                prev.replace(sma);
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
        T2: StoredRaw + Default,
        f32: From<T2>,
        T: From<f32>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.inner_version() + source.version(),
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
            Version::ZERO + self.inner_version() + source.version(),
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
        T2: StoredRaw + Default,
        f32: From<T2>,
        T: From<f32>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.inner_version() + source.version(),
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
        T2: StoredRaw + Default,
        f32: From<T2>,
        T: From<f32>,
    {
        self.validate_computed_version_or_reset_file(
            Version::ZERO + self.inner_version() + percentage_returns.version(),
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

    pub fn compute_zscore<T2, T3, T4>(
        &mut self,
        max_from: I,
        ratio: &impl AnyIterableVec<I, T2>,
        sma: &impl AnyIterableVec<I, T3>,
        sd: &impl AnyIterableVec<I, T4>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<f32>,
        T2: StoredRaw + Sub<T3, Output = T2> + Div<T4, Output = T>,
        T3: StoredRaw,
        T4: StoredRaw,
        T2: StoredRaw,
        f32: From<T2> + From<T3> + From<T4>,
    {
        let mut sma_iter = sma.iter();
        let mut sd_iter = sd.iter();

        self.compute_transform(
            max_from,
            ratio,
            |(i, ratio, ..)| {
                let sma = sma_iter.unwrap_get_inner(i);
                let sd = sd_iter.unwrap_get_inner(i);
                (i, (ratio - sma) / sd)
            },
            exit,
        )
    }
}

impl<'a, I, T> IntoIterator for &'a EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredCompressed,
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
    T: StoredCompressed,
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

impl<I, T> AnyStoredVec for EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredCompressed,
{
    fn file(&self) -> &File {
        self.0.file()
    }

    fn region_index(&self) -> usize {
        self.0.region_index()
    }

    fn header(&self) -> &Header {
        self.0.header()
    }

    fn mut_header(&mut self) -> &mut Header {
        self.0.mut_header()
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }

    fn stored_len(&self) -> usize {
        self.0.stored_len()
    }
}

impl<I, T> GenericStoredVec<I, T> for EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredCompressed,
{
    #[inline]
    fn read_(&self, index: usize, reader: &Reader) -> Result<T> {
        self.0.read_(index, reader)
    }

    #[inline]
    fn pushed(&self) -> &[T] {
        self.0.pushed()
    }
    #[inline]
    fn mut_pushed(&mut self) -> &mut Vec<T> {
        self.0.mut_pushed()
    }

    #[inline]
    fn holes(&self) -> &BTreeSet<usize> {
        self.0.holes()
    }
    #[inline]
    fn mut_holes(&mut self) -> &mut BTreeSet<usize> {
        self.0.mut_holes()
    }

    #[inline]
    fn updated(&self) -> &BTreeMap<usize, T> {
        self.0.updated()
    }
    #[inline]
    fn mut_updated(&mut self) -> &mut BTreeMap<usize, T> {
        self.0.mut_updated()
    }

    #[inline]
    fn truncate_if_needed(&mut self, index: I) -> Result<()> {
        self.0.truncate_if_needed(index)
    }

    #[inline]
    fn reset(&mut self) -> Result<()> {
        self.0.reset()
    }
}

impl<I, T> AnyIterableVec<I, T> for EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredCompressed,
{
    fn boxed_iter<'a>(&'a self) -> BoxedVecIterator<'a, I, T>
    where
        I: StoredIndex,
        T: StoredRaw + 'a,
    {
        Box::new(self.0.into_iter())
    }
}

impl<I, T> AnyCollectableVec for EagerVec<I, T>
where
    I: StoredIndex,
    T: StoredCompressed,
{
    fn collect_range_serde_json(
        &self,
        from: Option<usize>,
        to: Option<usize>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
