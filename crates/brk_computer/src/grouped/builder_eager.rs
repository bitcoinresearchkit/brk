use allocative::Allocative;
use brk_error::{Error, Result};
use brk_traversable::Traversable;
use brk_types::{CheckedSub, StoredU64, Version};
use vecdb::{
    AnyIterableVec, AnyStoredVec, AnyVec, Database, EagerVec, Exit, Format, GenericStoredVec,
    StoredIndex, StoredRaw,
};

use crate::utils::get_percentile;

use super::ComputedType;

#[derive(Clone, Debug, Traversable, Allocative)]
pub struct EagerVecsBuilder<I, T>
where
    I: StoredIndex,
    T: ComputedType,
{
    pub first: Option<Box<EagerVec<I, T>>>,
    pub average: Option<Box<EagerVec<I, T>>>,
    pub sum: Option<Box<EagerVec<I, T>>>,
    pub max: Option<Box<EagerVec<I, T>>>,
    pub pct90: Option<Box<EagerVec<I, T>>>,
    pub pct75: Option<Box<EagerVec<I, T>>>,
    pub median: Option<Box<EagerVec<I, T>>>,
    pub pct25: Option<Box<EagerVec<I, T>>>,
    pub pct10: Option<Box<EagerVec<I, T>>>,
    pub min: Option<Box<EagerVec<I, T>>>,
    pub last: Option<Box<EagerVec<I, T>>>,
    pub cumulative: Option<Box<EagerVec<I, T>>>,
}

const VERSION: Version = Version::ZERO;

impl<I, T> EagerVecsBuilder<I, T>
where
    I: StoredIndex,
    T: ComputedType,
{
    pub fn forced_import_compressed(
        db: &Database,
        name: &str,
        version: Version,
        options: VecBuilderOptions,
    ) -> Result<Self> {
        Self::forced_import(db, name, version, Format::Compressed, options)
    }

    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        format: Format,
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

        let s = Self {
            first: options.first.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        db,
                        &maybe_suffix("first"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            last: options.last.then(|| {
                Box::new(
                    EagerVec::forced_import(db, name, version + Version::ZERO, format).unwrap(),
                )
            }),
            min: options.min.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        db,
                        &maybe_suffix("min"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            max: options.max.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        db,
                        &maybe_suffix("max"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            median: options.median.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        db,
                        &maybe_suffix("median"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            average: options.average.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        db,
                        &maybe_suffix("avg"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            sum: options.sum.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        db,
                        &(if !options.last && !options.average && !options.min && !options.max {
                            name.to_string()
                        } else {
                            maybe_suffix("sum")
                        }),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            cumulative: options.cumulative.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        db,
                        &suffix("cumulative"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            pct90: options.pct90.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        db,
                        &maybe_suffix("pct90"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            pct75: options.pct75.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        db,
                        &maybe_suffix("pct75"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            pct25: options.pct25.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        db,
                        &maybe_suffix("pct25"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            pct10: options.pct10.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        db,
                        &maybe_suffix("pct10"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
        };

        Ok(s)
    }

    pub fn extend(
        &mut self,
        max_from: I,
        source: &impl AnyIterableVec<I, T>,
        exit: &Exit,
    ) -> Result<()> {
        if self.cumulative.is_none() {
            return Ok(());
        };

        self.validate_computed_version_or_reset(source.version())?;

        let index = self.starting_index(max_from);

        let cumulative_vec = self.cumulative.as_mut().unwrap();

        let mut cumulative = index.decremented().map_or(T::from(0_usize), |index| {
            cumulative_vec.iter().unwrap_get_inner(index)
        });
        source.iter_at(index).try_for_each(|(i, v)| -> Result<()> {
            cumulative += v;
            cumulative_vec.forced_push_at(i, cumulative, exit)?;
            Ok(())
        })?;

        self.safe_flush(exit)?;

        Ok(())
    }

    pub fn compute<I2>(
        &mut self,
        max_from: I,
        source: &impl AnyIterableVec<I2, T>,
        first_indexes: &impl AnyIterableVec<I, I2>,
        count_indexes: &impl AnyIterableVec<I, StoredU64>,
        exit: &Exit,
    ) -> Result<()>
    where
        I2: StoredIndex + StoredRaw + CheckedSub<I2>,
    {
        self.validate_computed_version_or_reset(
            source.version() + first_indexes.version() + count_indexes.version(),
        )?;

        let index = self.starting_index(max_from);

        let mut count_indexes_iter = count_indexes.iter();
        let mut source_iter = source.iter();

        let cumulative_vec = self.cumulative.as_mut();

        let mut cumulative = cumulative_vec.map(|cumulative_vec| {
            index.decremented().map_or(T::from(0_usize), |index| {
                cumulative_vec.iter().unwrap_get_inner(index)
            })
        });

        first_indexes
            .iter_at(index)
            .try_for_each(|(index, first_index)| -> Result<()> {
                let count_index = count_indexes_iter.unwrap_get_inner(index);

                if let Some(first) = self.first.as_mut() {
                    let f = source_iter
                        .get_inner(first_index)
                        .unwrap_or_else(|| T::from(0_usize));
                    first.forced_push_at(index, f, exit)?;
                }

                if let Some(last) = self.last.as_mut() {
                    let count_index = *count_index as usize;
                    if count_index == 0 {
                        panic!("should compute last if count can be 0")
                    }
                    let last_index = first_index + (count_index - 1);
                    let v = source_iter.unwrap_get_inner(last_index);
                    // .context("to work")
                    // .inspect_err(|_| {
                    //     dbg!(first_index, count_index, last_index);
                    // })
                    // .unwrap()
                    // ;
                    last.forced_push_at(index, v, exit)?;
                }

                let needs_sum_or_cumulative = self.sum.is_some() || self.cumulative.is_some();
                let needs_average_sum_or_cumulative =
                    needs_sum_or_cumulative || self.average.is_some();
                let needs_sorted = self.max.is_some()
                    || self.pct90.is_some()
                    || self.pct75.is_some()
                    || self.median.is_some()
                    || self.pct25.is_some()
                    || self.pct10.is_some()
                    || self.min.is_some();
                let needs_values = needs_sorted || needs_average_sum_or_cumulative;

                if needs_values {
                    source_iter.set(first_index);
                    let mut values = (&mut source_iter)
                        .take(*count_index as usize)
                        .map(|(_, v)| v)
                        .collect::<Vec<_>>();

                    if needs_sorted {
                        values.sort_unstable();

                        if let Some(max) = self.max.as_mut() {
                            max.forced_push_at(
                                index,
                                *values
                                    .last()
                                    .ok_or(Error::Str("expect some"))
                                    .inspect_err(|_| {
                                        dbg!(
                                            &values,
                                            max.name(),
                                            first_indexes.name(),
                                            first_index,
                                            count_indexes.name(),
                                            count_index,
                                            source.len(),
                                            source.name()
                                        );
                                    })
                                    .unwrap(),
                                exit,
                            )?;
                        }

                        if let Some(pct90) = self.pct90.as_mut() {
                            pct90.forced_push_at(index, get_percentile(&values, 0.90), exit)?;
                        }

                        if let Some(pct75) = self.pct75.as_mut() {
                            pct75.forced_push_at(index, get_percentile(&values, 0.75), exit)?;
                        }

                        if let Some(median) = self.median.as_mut() {
                            median.forced_push_at(index, get_percentile(&values, 0.50), exit)?;
                        }

                        if let Some(pct25) = self.pct25.as_mut() {
                            pct25.forced_push_at(index, get_percentile(&values, 0.25), exit)?;
                        }

                        if let Some(pct10) = self.pct10.as_mut() {
                            pct10.forced_push_at(index, get_percentile(&values, 0.10), exit)?;
                        }

                        if let Some(min) = self.min.as_mut() {
                            min.forced_push_at(index, *values.first().unwrap(), exit)?;
                        }
                    }

                    if needs_average_sum_or_cumulative {
                        let len = values.len();
                        let sum = values.into_iter().fold(T::from(0), |a, b| a + b);

                        if let Some(average) = self.average.as_mut() {
                            let avg = sum / len;
                            average.forced_push_at(index, avg, exit)?;
                        }

                        if needs_sum_or_cumulative {
                            if let Some(sum_vec) = self.sum.as_mut() {
                                sum_vec.forced_push_at(index, sum, exit)?;
                            }

                            if let Some(cumulative_vec) = self.cumulative.as_mut() {
                                let t = cumulative.unwrap() + sum;
                                cumulative.replace(t);
                                cumulative_vec.forced_push_at(index, t, exit)?;
                            }
                        }
                    }
                }

                Ok(())
            })?;

        self.safe_flush(exit)?;

        Ok(())
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_aligned<I2>(
        &mut self,
        max_from: I,
        source: &EagerVecsBuilder<I2, T>,
        first_indexes: &impl AnyIterableVec<I, I2>,
        count_indexes: &impl AnyIterableVec<I, StoredU64>,
        exit: &Exit,
    ) -> Result<()>
    where
        I2: StoredIndex + StoredRaw + CheckedSub<I2>,
    {
        if self.pct90.is_some()
            || self.pct75.is_some()
            || self.median.is_some()
            || self.pct25.is_some()
            || self.pct10.is_some()
        {
            panic!("unsupported");
        }

        self.validate_computed_version_or_reset(
            VERSION + first_indexes.version() + count_indexes.version(),
        )?;

        let index = self.starting_index(max_from);

        let mut count_indexes_iter = count_indexes.iter();

        let mut source_first_iter = source.first.as_ref().map(|f| f.iter());
        let mut source_last_iter = source.last.as_ref().map(|f| f.iter());
        let mut source_max_iter = source.max.as_ref().map(|f| f.iter());
        let mut source_min_iter = source.min.as_ref().map(|f| f.iter());
        let mut source_average_iter = source.average.as_ref().map(|f| f.iter());
        let mut source_sum_iter = source.sum.as_ref().map(|f| f.iter());

        let mut cumulative = self.cumulative.as_mut().map(|cumulative_vec| {
            index.decremented().map_or(T::from(0_usize), |index| {
                cumulative_vec.iter().unwrap_get_inner(index)
            })
        });

        first_indexes
            .iter_at(index)
            .try_for_each(|(index, first_index, ..)| -> Result<()> {
                let count_index = count_indexes_iter.unwrap_get_inner(index);

                if let Some(first) = self.first.as_mut() {
                    let v = source_first_iter
                        .as_mut()
                        .unwrap()
                        .unwrap_get_inner(first_index);
                    first.forced_push_at(index, v, exit)?;
                }

                if let Some(last) = self.last.as_mut() {
                    let count_index = *count_index as usize;
                    if count_index == 0 {
                        panic!("should compute last if count can be 0")
                    }
                    let last_index = first_index + (count_index - 1);
                    let v = source_last_iter
                        .as_mut()
                        .unwrap()
                        .unwrap_get_inner(last_index);
                    last.forced_push_at(index, v, exit)?;
                }

                let needs_sum_or_cumulative = self.sum.is_some() || self.cumulative.is_some();
                let needs_average_sum_or_cumulative =
                    needs_sum_or_cumulative || self.average.is_some();
                let needs_sorted = self.max.is_some() || self.min.is_some();
                let needs_values = needs_sorted || needs_average_sum_or_cumulative;

                if needs_values {
                    if needs_sorted {
                        if let Some(max) = self.max.as_mut() {
                            let source_max_iter = source_max_iter.as_mut().unwrap();
                            source_max_iter.set(first_index);
                            let mut values = source_max_iter
                                .take(*count_index as usize)
                                .map(|(_, v)| v)
                                .collect::<Vec<_>>();
                            values.sort_unstable();
                            max.forced_push_at(index, *values.last().unwrap(), exit)?;
                        }

                        if let Some(min) = self.min.as_mut() {
                            let source_min_iter = source_min_iter.as_mut().unwrap();
                            source_min_iter.set(first_index);
                            let mut values = source_min_iter
                                .take(*count_index as usize)
                                .map(|(_, v)| v)
                                .collect::<Vec<_>>();
                            values.sort_unstable();
                            min.forced_push_at(index, *values.first().unwrap(), exit)?;
                        }
                    }

                    if needs_average_sum_or_cumulative {
                        if let Some(average) = self.average.as_mut() {
                            let source_average_iter = source_average_iter.as_mut().unwrap();
                            source_average_iter.set(first_index);
                            let values = source_average_iter
                                .take(*count_index as usize)
                                .map(|(_, v)| v)
                                .collect::<Vec<_>>();

                            let len = values.len();
                            let cumulative = values.into_iter().fold(T::from(0), |a, b| a + b);
                            // TODO: Multiply by count then divide by cumulative
                            // Right now it's not 100% accurate as there could be more or less elements in the lower timeframe (28 days vs 31 days in a month for example)
                            let avg = cumulative / len;
                            average.forced_push_at(index, avg, exit)?;
                        }

                        if needs_sum_or_cumulative {
                            let source_sum_iter = source_sum_iter.as_mut().unwrap();
                            source_sum_iter.set(first_index);
                            let values = source_sum_iter
                                .take(*count_index as usize)
                                .map(|(_, v)| v)
                                .collect::<Vec<_>>();

                            let sum = values.into_iter().fold(T::from(0), |a, b| a + b);

                            if let Some(sum_vec) = self.sum.as_mut() {
                                sum_vec.forced_push_at(index, sum, exit)?;
                            }

                            if let Some(cumulative_vec) = self.cumulative.as_mut() {
                                let t = cumulative.unwrap() + sum;
                                cumulative.replace(t);
                                cumulative_vec.forced_push_at(index, t, exit)?;
                            }
                        }
                    }
                }

                Ok(())
            })?;

        self.safe_flush(exit)?;

        Ok(())
    }

    pub fn starting_index(&self, max_from: I) -> I {
        max_from.min(I::from(
            self.iter_any_collectable().map(|v| v.len()).min().unwrap(),
        ))
    }

    pub fn unwrap_first(&self) -> &EagerVec<I, T> {
        self.first.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_average(&self) -> &EagerVec<I, T> {
        self.average.as_ref().unwrap()
    }
    pub fn unwrap_sum(&self) -> &EagerVec<I, T> {
        self.sum.as_ref().unwrap()
    }
    pub fn unwrap_max(&self) -> &EagerVec<I, T> {
        self.max.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_pct90(&self) -> &EagerVec<I, T> {
        self.pct90.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_pct75(&self) -> &EagerVec<I, T> {
        self.pct75.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_median(&self) -> &EagerVec<I, T> {
        self.median.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_pct25(&self) -> &EagerVec<I, T> {
        self.pct25.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_pct10(&self) -> &EagerVec<I, T> {
        self.pct10.as_ref().unwrap()
    }
    pub fn unwrap_min(&self) -> &EagerVec<I, T> {
        self.min.as_ref().unwrap()
    }
    pub fn unwrap_last(&self) -> &EagerVec<I, T> {
        self.last.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_cumulative(&self) -> &EagerVec<I, T> {
        self.cumulative.as_ref().unwrap()
    }

    pub fn safe_flush(&mut self, exit: &Exit) -> Result<()> {
        if let Some(first) = self.first.as_mut() {
            first.safe_flush(exit)?;
        }
        if let Some(last) = self.last.as_mut() {
            last.safe_flush(exit)?;
        }
        if let Some(min) = self.min.as_mut() {
            min.safe_flush(exit)?;
        }
        if let Some(max) = self.max.as_mut() {
            max.safe_flush(exit)?;
        }
        if let Some(median) = self.median.as_mut() {
            median.safe_flush(exit)?;
        }
        if let Some(average) = self.average.as_mut() {
            average.safe_flush(exit)?;
        }
        if let Some(sum) = self.sum.as_mut() {
            sum.safe_flush(exit)?;
        }
        if let Some(cumulative) = self.cumulative.as_mut() {
            cumulative.safe_flush(exit)?;
        }
        if let Some(pct90) = self.pct90.as_mut() {
            pct90.safe_flush(exit)?;
        }
        if let Some(pct75) = self.pct75.as_mut() {
            pct75.safe_flush(exit)?;
        }
        if let Some(pct25) = self.pct25.as_mut() {
            pct25.safe_flush(exit)?;
        }
        if let Some(pct10) = self.pct10.as_mut() {
            pct10.safe_flush(exit)?;
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
