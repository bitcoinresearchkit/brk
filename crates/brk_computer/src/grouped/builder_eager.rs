use brk_error::{Error, Result};
use brk_traversable::Traversable;
use brk_types::{CheckedSub, StoredU64, Version};
use vecdb::{
    AnyStoredVec, Database, EagerVec, Exit, GenericStoredVec, ImportableVec, IterableVec, PcoVec,
    VecIndex, VecValue,
};

use crate::utils::{OptionExt, get_percentile};

use super::ComputedVecValue;

const VERSION: Version = Version::ZERO;

#[derive(Clone, Debug, Traversable)]
pub struct EagerVecsBuilder<I, T>
where
    I: VecIndex,
    T: ComputedVecValue,
{
    pub first: Option<Box<EagerVec<PcoVec<I, T>>>>,
    pub average: Option<Box<EagerVec<PcoVec<I, T>>>>,
    pub sum: Option<Box<EagerVec<PcoVec<I, T>>>>,
    pub max: Option<Box<EagerVec<PcoVec<I, T>>>>,
    pub pct90: Option<Box<EagerVec<PcoVec<I, T>>>>,
    pub pct75: Option<Box<EagerVec<PcoVec<I, T>>>>,
    pub median: Option<Box<EagerVec<PcoVec<I, T>>>>,
    pub pct25: Option<Box<EagerVec<PcoVec<I, T>>>>,
    pub pct10: Option<Box<EagerVec<PcoVec<I, T>>>>,
    pub min: Option<Box<EagerVec<PcoVec<I, T>>>>,
    pub last: Option<Box<EagerVec<PcoVec<I, T>>>>,
    pub cumulative: Option<Box<EagerVec<PcoVec<I, T>>>>,
}

impl<I, T> EagerVecsBuilder<I, T>
where
    I: VecIndex,
    T: ComputedVecValue,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        options: VecBuilderOptions,
    ) -> Result<Self> {
        let only_one_active = options.is_only_one_active();
        let suffix = |s: &str| format!("{name}_{s}");
        let maybe_suffix = |s: &str| {
            if only_one_active {
                name.to_string()
            } else {
                suffix(s)
            }
        };
        let v = version + VERSION;

        macro_rules! import {
            ($s:expr) => {
                Box::new(EagerVec::forced_import(db, &maybe_suffix($s), v).unwrap())
            };
        }

        let s = Self {
            first: options.first.then(|| import!("first")),
            last: options
                .last
                .then(|| Box::new(EagerVec::forced_import(db, name, v).unwrap())),
            min: options.min.then(|| import!("min")),
            max: options.max.then(|| import!("max")),
            median: options.median.then(|| import!("median")),
            average: options.average.then(|| import!("avg")),
            sum: options.sum.then(|| {
                let sum_name = if !options.last && !options.average && !options.min && !options.max
                {
                    name.to_string()
                } else {
                    maybe_suffix("sum")
                };
                Box::new(EagerVec::forced_import(db, &sum_name, v).unwrap())
            }),
            cumulative: options
                .cumulative
                .then(|| Box::new(EagerVec::forced_import(db, &suffix("cumulative"), v).unwrap())),
            pct90: options.pct90.then(|| import!("pct90")),
            pct75: options.pct75.then(|| import!("pct75")),
            pct25: options.pct25.then(|| import!("pct25")),
            pct10: options.pct10.then(|| import!("pct10")),
        };

        Ok(s)
    }

    #[inline]
    fn needs_percentiles(&self) -> bool {
        self.pct90.is_some()
            || self.pct75.is_some()
            || self.median.is_some()
            || self.pct25.is_some()
            || self.pct10.is_some()
    }

    #[inline]
    fn needs_minmax(&self) -> bool {
        self.max.is_some() || self.min.is_some()
    }

    #[inline]
    fn needs_sum_or_cumulative(&self) -> bool {
        self.sum.is_some() || self.cumulative.is_some()
    }

    #[inline]
    fn needs_average_sum_or_cumulative(&self) -> bool {
        self.needs_sum_or_cumulative() || self.average.is_some()
    }

    /// Compute min/max in O(n) without sorting or collecting
    #[inline]
    fn compute_minmax_streaming(
        &mut self,
        index: usize,
        iter: impl Iterator<Item = T>,
    ) -> Result<()> {
        let mut min_val: Option<T> = None;
        let mut max_val: Option<T> = None;
        let need_min = self.min.is_some();
        let need_max = self.max.is_some();

        for val in iter {
            if need_min {
                min_val = Some(min_val.map_or(val, |m| if val < m { val } else { m }));
            }
            if need_max {
                max_val = Some(max_val.map_or(val, |m| if val > m { val } else { m }));
            }
        }

        if let Some(min) = self.min.as_mut() {
            min.truncate_push_at(index, min_val.unwrap())?;
        }
        if let Some(max) = self.max.as_mut() {
            max.truncate_push_at(index, max_val.unwrap())?;
        }
        Ok(())
    }

    /// Compute min/max from collected values in O(n) without sorting
    #[inline]
    fn compute_minmax_from_slice(&mut self, index: usize, values: &[T]) -> Result<()> {
        if let Some(min) = self.min.as_mut() {
            min.truncate_push_at(index, *values.iter().min().unwrap())?;
        }
        if let Some(max) = self.max.as_mut() {
            max.truncate_push_at(index, *values.iter().max().unwrap())?;
        }
        Ok(())
    }

    /// Compute percentiles from sorted values (assumes values is already sorted)
    fn compute_percentiles_from_sorted(&mut self, index: usize, values: &[T]) -> Result<()> {
        if let Some(max) = self.max.as_mut() {
            max.truncate_push_at(index, *values.last().ok_or(Error::Internal("Empty values for percentiles"))?)?;
        }
        if let Some(pct90) = self.pct90.as_mut() {
            pct90.truncate_push_at(index, get_percentile(values, 0.90))?;
        }
        if let Some(pct75) = self.pct75.as_mut() {
            pct75.truncate_push_at(index, get_percentile(values, 0.75))?;
        }
        if let Some(median) = self.median.as_mut() {
            median.truncate_push_at(index, get_percentile(values, 0.50))?;
        }
        if let Some(pct25) = self.pct25.as_mut() {
            pct25.truncate_push_at(index, get_percentile(values, 0.25))?;
        }
        if let Some(pct10) = self.pct10.as_mut() {
            pct10.truncate_push_at(index, get_percentile(values, 0.10))?;
        }
        if let Some(min) = self.min.as_mut() {
            min.truncate_push_at(index, *values.first().unwrap())?;
        }
        Ok(())
    }

    /// Compute sum, average, and cumulative from values
    fn compute_aggregates(
        &mut self,
        index: usize,
        values: Vec<T>,
        cumulative: &mut Option<T>,
    ) -> Result<()> {
        let len = values.len();
        let sum = values.into_iter().fold(T::from(0), |a, b| a + b);

        if let Some(average) = self.average.as_mut() {
            average.truncate_push_at(index, sum / len)?;
        }

        if self.needs_sum_or_cumulative() {
            if let Some(sum_vec) = self.sum.as_mut() {
                sum_vec.truncate_push_at(index, sum)?;
            }
            if let Some(cumulative_vec) = self.cumulative.as_mut() {
                let t = cumulative.unwrap() + sum;
                cumulative.replace(t);
                cumulative_vec.truncate_push_at(index, t)?;
            }
        }
        Ok(())
    }

    pub fn extend(
        &mut self,
        max_from: I,
        source: &impl IterableVec<I, T>,
        exit: &Exit,
    ) -> Result<()> {
        if self.cumulative.is_none() {
            return Ok(());
        };

        self.validate_computed_version_or_reset(source.version())?;

        let index = self.starting_index(max_from);

        let cumulative_vec = self.cumulative.um();

        let mut cumulative = index.decremented().map_or(T::from(0_usize), |index| {
            cumulative_vec.iter().get_unwrap(index)
        });
        source
            .iter()
            .enumerate()
            .skip(index.to_usize())
            .try_for_each(|(i, v)| -> Result<()> {
                cumulative += v;
                cumulative_vec.truncate_push_at(i, cumulative)?;
                Ok(())
            })?;

        self.safe_write(exit)?;

        Ok(())
    }

    pub fn compute<A>(
        &mut self,
        max_from: I,
        source: &impl IterableVec<A, T>,
        first_indexes: &impl IterableVec<I, A>,
        count_indexes: &impl IterableVec<I, StoredU64>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecIndex + VecValue + CheckedSub<A>,
    {
        self.validate_computed_version_or_reset(
            source.version() + first_indexes.version() + count_indexes.version(),
        )?;

        let index = self.starting_index(max_from);

        let mut source_iter = source.iter();

        let cumulative_vec = self.cumulative.as_mut();

        let mut cumulative = cumulative_vec.map(|cumulative_vec| {
            index.decremented().map_or(T::from(0_usize), |index| {
                cumulative_vec.iter().get_unwrap(index)
            })
        });

        let mut count_indexes_iter = count_indexes.iter().skip(index.to_usize());

        first_indexes
            .iter()
            .enumerate()
            .skip(index.to_usize())
            .try_for_each(|(index, first_index)| -> Result<()> {
                let count_index = count_indexes_iter.next().unwrap();

                if let Some(first) = self.first.as_mut() {
                    let f = source_iter
                        .get(first_index)
                        .unwrap_or_else(|| T::from(0_usize));
                    first.truncate_push_at(index, f)?;
                }

                if let Some(last) = self.last.as_mut() {
                    let count_index = *count_index as usize;
                    if count_index == 0 {
                        panic!("should compute last if count can be 0")
                    }
                    let last_index = first_index + (count_index - 1);
                    let v = source_iter.get_unwrap(last_index);
                    // .context("to work")
                    // .inspect_err(|_| {
                    //     dbg!(first_index, count_index, last_index);
                    // })
                    // .unwrap()
                    // ;
                    last.truncate_push_at(index, v)?;
                }

                let needs_percentiles = self.needs_percentiles();
                let needs_minmax = self.needs_minmax();
                let needs_aggregates = self.needs_average_sum_or_cumulative();

                // Fast path: only min/max needed, no sorting or allocation required
                if needs_minmax && !needs_percentiles && !needs_aggregates {
                    source_iter.set_position(first_index);
                    self.compute_minmax_streaming(
                        index,
                        (&mut source_iter).take(*count_index as usize),
                    )?;
                } else if needs_percentiles || needs_aggregates {
                    source_iter.set_position(first_index);
                    let mut values = (&mut source_iter)
                        .take(*count_index as usize)
                        .collect::<Vec<_>>();

                    if needs_percentiles {
                        values.sort_unstable();
                        self.compute_percentiles_from_sorted(index, &values)?;
                    } else if needs_minmax {
                        // We have values collected but only need min/max (along with aggregates)
                        self.compute_minmax_from_slice(index, &values)?;
                    }

                    if needs_aggregates {
                        self.compute_aggregates(index, values, &mut cumulative)?;
                    }
                }

                Ok(())
            })?;

        self.safe_write(exit)?;

        Ok(())
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_aligned<A>(
        &mut self,
        max_from: I,
        source: &EagerVecsBuilder<A, T>,
        first_indexes: &impl IterableVec<I, A>,
        count_indexes: &impl IterableVec<I, StoredU64>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecIndex + VecValue + CheckedSub<A>,
    {
        if self.needs_percentiles() {
            panic!("percentiles unsupported in from_aligned");
        }

        self.validate_computed_version_or_reset(
            VERSION + first_indexes.version() + count_indexes.version(),
        )?;

        let index = self.starting_index(max_from);

        let mut source_first_iter = source.first.as_ref().map(|f| f.iter());
        let mut source_last_iter = source.last.as_ref().map(|f| f.iter());
        let mut source_max_iter = source.max.as_ref().map(|f| f.iter());
        let mut source_min_iter = source.min.as_ref().map(|f| f.iter());
        let mut source_average_iter = source.average.as_ref().map(|f| f.iter());
        let mut source_sum_iter = source.sum.as_ref().map(|f| f.iter());

        let mut cumulative = self.cumulative.as_mut().map(|cumulative_vec| {
            index.decremented().map_or(T::from(0_usize), |index| {
                cumulative_vec.iter().get_unwrap(index)
            })
        });

        let mut count_indexes_iter = count_indexes.iter().skip(index.to_usize());

        first_indexes
            .iter()
            .enumerate()
            .skip(index.to_usize())
            .try_for_each(|(index, first_index, ..)| -> Result<()> {
                let count_index = count_indexes_iter.next().unwrap();

                if let Some(first) = self.first.as_mut() {
                    let v = source_first_iter.um().get_unwrap(first_index);
                    first.truncate_push_at(index, v)?;
                }

                if let Some(last) = self.last.as_mut() {
                    let count_index = *count_index as usize;
                    if count_index == 0 {
                        panic!("should compute last if count can be 0")
                    }
                    let last_index = first_index + (count_index - 1);
                    let v = source_last_iter.um().get_unwrap(last_index);
                    last.truncate_push_at(index, v)?;
                }

                let needs_minmax = self.needs_minmax();
                let needs_aggregates = self.needs_average_sum_or_cumulative();

                if needs_minmax || needs_aggregates {
                    // Min/max: use streaming O(n) instead of sort O(n log n)
                    if needs_minmax {
                        if let Some(max) = self.max.as_mut() {
                            let source_max_iter = source_max_iter.um();
                            source_max_iter.set_position(first_index);
                            let max_val =
                                source_max_iter.take(*count_index as usize).max().unwrap();
                            max.truncate_push_at(index, max_val)?;
                        }

                        if let Some(min) = self.min.as_mut() {
                            let source_min_iter = source_min_iter.um();
                            source_min_iter.set_position(first_index);
                            let min_val =
                                source_min_iter.take(*count_index as usize).min().unwrap();
                            min.truncate_push_at(index, min_val)?;
                        }
                    }

                    if needs_aggregates {
                        if let Some(average) = self.average.as_mut() {
                            let source_average_iter = source_average_iter.um();
                            source_average_iter.set_position(first_index);
                            let mut len = 0usize;
                            let sum = (&mut *source_average_iter)
                                .take(*count_index as usize)
                                .inspect(|_| len += 1)
                                .fold(T::from(0), |a, b| a + b);
                            // TODO: Multiply by count then divide by cumulative
                            // Right now it's not 100% accurate as there could be more or less elements in the lower timeframe (28 days vs 31 days in a month for example)
                            let avg = sum / len;
                            average.truncate_push_at(index, avg)?;
                        }

                        if self.needs_sum_or_cumulative() {
                            let source_sum_iter = source_sum_iter.um();
                            source_sum_iter.set_position(first_index);
                            let sum = source_sum_iter
                                .take(*count_index as usize)
                                .fold(T::from(0), |a, b| a + b);

                            if let Some(sum_vec) = self.sum.as_mut() {
                                sum_vec.truncate_push_at(index, sum)?;
                            }

                            if let Some(cumulative_vec) = self.cumulative.as_mut() {
                                let t = cumulative.unwrap() + sum;
                                cumulative.replace(t);
                                cumulative_vec.truncate_push_at(index, t)?;
                            }
                        }
                    }
                }

                Ok(())
            })?;

        self.safe_write(exit)?;

        Ok(())
    }

    pub fn starting_index(&self, max_from: I) -> I {
        max_from.min(I::from(
            self.iter_any_exportable().map(|v| v.len()).min().unwrap(),
        ))
    }

    #[inline]
    pub fn unwrap_first(&self) -> &EagerVec<PcoVec<I, T>> {
        self.first.u()
    }
    #[inline]
    #[allow(unused)]
    pub fn unwrap_average(&self) -> &EagerVec<PcoVec<I, T>> {
        self.average.u()
    }
    #[inline]
    pub fn unwrap_sum(&self) -> &EagerVec<PcoVec<I, T>> {
        self.sum.u()
    }
    #[inline]
    pub fn unwrap_max(&self) -> &EagerVec<PcoVec<I, T>> {
        self.max.u()
    }
    #[inline]
    #[allow(unused)]
    pub fn unwrap_pct90(&self) -> &EagerVec<PcoVec<I, T>> {
        self.pct90.u()
    }
    #[inline]
    #[allow(unused)]
    pub fn unwrap_pct75(&self) -> &EagerVec<PcoVec<I, T>> {
        self.pct75.u()
    }
    #[inline]
    #[allow(unused)]
    pub fn unwrap_median(&self) -> &EagerVec<PcoVec<I, T>> {
        self.median.u()
    }
    #[inline]
    #[allow(unused)]
    pub fn unwrap_pct25(&self) -> &EagerVec<PcoVec<I, T>> {
        self.pct25.u()
    }
    #[inline]
    #[allow(unused)]
    pub fn unwrap_pct10(&self) -> &EagerVec<PcoVec<I, T>> {
        self.pct10.u()
    }
    #[inline]
    pub fn unwrap_min(&self) -> &EagerVec<PcoVec<I, T>> {
        self.min.u()
    }
    #[inline]
    pub fn unwrap_last(&self) -> &EagerVec<PcoVec<I, T>> {
        self.last.u()
    }
    #[inline]
    #[allow(unused)]
    pub fn unwrap_cumulative(&self) -> &EagerVec<PcoVec<I, T>> {
        self.cumulative.u()
    }

    pub fn safe_write(&mut self, exit: &Exit) -> Result<()> {
        if let Some(first) = self.first.as_mut() {
            first.safe_write(exit)?;
        }
        if let Some(last) = self.last.as_mut() {
            last.safe_write(exit)?;
        }
        if let Some(min) = self.min.as_mut() {
            min.safe_write(exit)?;
        }
        if let Some(max) = self.max.as_mut() {
            max.safe_write(exit)?;
        }
        if let Some(median) = self.median.as_mut() {
            median.safe_write(exit)?;
        }
        if let Some(average) = self.average.as_mut() {
            average.safe_write(exit)?;
        }
        if let Some(sum) = self.sum.as_mut() {
            sum.safe_write(exit)?;
        }
        if let Some(cumulative) = self.cumulative.as_mut() {
            cumulative.safe_write(exit)?;
        }
        if let Some(pct90) = self.pct90.as_mut() {
            pct90.safe_write(exit)?;
        }
        if let Some(pct75) = self.pct75.as_mut() {
            pct75.safe_write(exit)?;
        }
        if let Some(pct25) = self.pct25.as_mut() {
            pct25.safe_write(exit)?;
        }
        if let Some(pct10) = self.pct10.as_mut() {
            pct10.safe_write(exit)?;
        }

        Ok(())
    }

    pub fn validate_computed_version_or_reset(&mut self, version: Version) -> Result<()> {
        if let Some(first) = self.first.as_mut() {
            first.validate_computed_version_or_reset(Version::ZERO + version)?;
        }
        if let Some(last) = self.last.as_mut() {
            last.validate_computed_version_or_reset(Version::ZERO + version)?;
        }
        if let Some(min) = self.min.as_mut() {
            min.validate_computed_version_or_reset(Version::ZERO + version)?;
        }
        if let Some(max) = self.max.as_mut() {
            max.validate_computed_version_or_reset(Version::ZERO + version)?;
        }
        if let Some(median) = self.median.as_mut() {
            median.validate_computed_version_or_reset(Version::ZERO + version)?;
        }
        if let Some(average) = self.average.as_mut() {
            average.validate_computed_version_or_reset(Version::ZERO + version)?;
        }
        if let Some(sum) = self.sum.as_mut() {
            sum.validate_computed_version_or_reset(Version::ZERO + version)?;
        }
        if let Some(cumulative) = self.cumulative.as_mut() {
            cumulative.validate_computed_version_or_reset(Version::ZERO + version)?;
        }
        if let Some(pct90) = self.pct90.as_mut() {
            pct90.validate_computed_version_or_reset(Version::ZERO + version)?;
        }
        if let Some(pct75) = self.pct75.as_mut() {
            pct75.validate_computed_version_or_reset(Version::ZERO + version)?;
        }
        if let Some(pct25) = self.pct25.as_mut() {
            pct25.validate_computed_version_or_reset(Version::ZERO + version)?;
        }
        if let Some(pct10) = self.pct10.as_mut() {
            pct10.validate_computed_version_or_reset(Version::ZERO + version)?;
        }

        Ok(())
    }
}

#[derive(Default, Clone, Copy)]
pub struct VecBuilderOptions {
    average: bool,
    sum: bool,
    max: bool,
    pct90: bool,
    pct75: bool,
    median: bool,
    pct25: bool,
    pct10: bool,
    min: bool,
    first: bool,
    last: bool,
    cumulative: bool,
}

impl VecBuilderOptions {
    pub fn average(&self) -> bool {
        self.average
    }

    pub fn sum(&self) -> bool {
        self.sum
    }

    pub fn max(&self) -> bool {
        self.max
    }

    pub fn pct90(&self) -> bool {
        self.pct90
    }

    pub fn pct75(&self) -> bool {
        self.pct75
    }

    pub fn median(&self) -> bool {
        self.median
    }

    pub fn pct25(&self) -> bool {
        self.pct25
    }

    pub fn pct10(&self) -> bool {
        self.pct10
    }

    pub fn min(&self) -> bool {
        self.min
    }

    pub fn first(&self) -> bool {
        self.first
    }

    pub fn last(&self) -> bool {
        self.last
    }

    pub fn cumulative(&self) -> bool {
        self.cumulative
    }

    pub fn add_first(mut self) -> Self {
        self.first = true;
        self
    }

    pub fn add_last(mut self) -> Self {
        self.last = true;
        self
    }

    pub fn add_min(mut self) -> Self {
        self.min = true;
        self
    }

    pub fn add_max(mut self) -> Self {
        self.max = true;
        self
    }

    #[allow(unused)]
    pub fn add_median(mut self) -> Self {
        self.median = true;
        self
    }

    pub fn add_average(mut self) -> Self {
        self.average = true;
        self
    }

    pub fn add_sum(mut self) -> Self {
        self.sum = true;
        self
    }

    #[allow(unused)]
    pub fn add_pct90(mut self) -> Self {
        self.pct90 = true;
        self
    }

    #[allow(unused)]
    pub fn add_pct75(mut self) -> Self {
        self.pct75 = true;
        self
    }

    #[allow(unused)]
    pub fn add_pct25(mut self) -> Self {
        self.pct25 = true;
        self
    }

    #[allow(unused)]
    pub fn add_pct10(mut self) -> Self {
        self.pct10 = true;
        self
    }

    pub fn add_cumulative(mut self) -> Self {
        self.cumulative = true;
        self
    }

    #[allow(unused)]
    pub fn rm_min(mut self) -> Self {
        self.min = false;
        self
    }

    #[allow(unused)]
    pub fn rm_max(mut self) -> Self {
        self.max = false;
        self
    }

    #[allow(unused)]
    pub fn rm_median(mut self) -> Self {
        self.median = false;
        self
    }

    #[allow(unused)]
    pub fn rm_average(mut self) -> Self {
        self.average = false;
        self
    }

    #[allow(unused)]
    pub fn rm_sum(mut self) -> Self {
        self.sum = false;
        self
    }

    #[allow(unused)]
    pub fn rm_pct90(mut self) -> Self {
        self.pct90 = false;
        self
    }

    #[allow(unused)]
    pub fn rm_pct75(mut self) -> Self {
        self.pct75 = false;
        self
    }

    #[allow(unused)]
    pub fn rm_pct25(mut self) -> Self {
        self.pct25 = false;
        self
    }

    #[allow(unused)]
    pub fn rm_pct10(mut self) -> Self {
        self.pct10 = false;
        self
    }

    #[allow(unused)]
    pub fn rm_cumulative(mut self) -> Self {
        self.cumulative = false;
        self
    }

    pub fn add_minmax(mut self) -> Self {
        self.min = true;
        self.max = true;
        self
    }

    pub fn add_percentiles(mut self) -> Self {
        self.pct90 = true;
        self.pct75 = true;
        self.median = true;
        self.pct25 = true;
        self.pct10 = true;
        self
    }

    pub fn remove_percentiles(mut self) -> Self {
        self.pct90 = false;
        self.pct75 = false;
        self.median = false;
        self.pct25 = false;
        self.pct10 = false;
        self
    }

    pub fn is_only_one_active(&self) -> bool {
        [
            self.average,
            self.sum,
            self.max,
            self.pct90,
            self.pct75,
            self.median,
            self.pct25,
            self.pct10,
            self.min,
            self.first,
            self.last,
            self.cumulative,
        ]
        .iter()
        .filter(|b| **b)
        .count()
            == 1
    }

    pub fn copy_self_extra(&self) -> Self {
        Self {
            cumulative: self.cumulative,
            ..Self::default()
        }
    }
}
