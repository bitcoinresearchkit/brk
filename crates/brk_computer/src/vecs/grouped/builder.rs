use std::path::Path;

use brk_core::{CheckedSub, Result, StoredUsize, Version};
use brk_exit::Exit;
use brk_vec::{AnyCollectableVec, AnyIterableVec, EagerVec, Format, StoredIndex, StoredType};
use color_eyre::eyre::ContextCompat;

use crate::utils::get_percentile;

use super::ComputedType;

#[derive(Clone, Debug)]
pub struct ComputedVecBuilder<I, T>
where
    I: StoredIndex,
    T: ComputedType,
{
    pub first: Option<Box<EagerVec<I, T>>>,
    pub average: Option<Box<EagerVec<I, T>>>,
    pub sum: Option<Box<EagerVec<I, T>>>,
    pub max: Option<Box<EagerVec<I, T>>>,
    pub _90p: Option<Box<EagerVec<I, T>>>,
    pub _75p: Option<Box<EagerVec<I, T>>>,
    pub median: Option<Box<EagerVec<I, T>>>,
    pub _25p: Option<Box<EagerVec<I, T>>>,
    pub _10p: Option<Box<EagerVec<I, T>>>,
    pub min: Option<Box<EagerVec<I, T>>>,
    pub last: Option<Box<EagerVec<I, T>>>,
    pub cumulative: Option<Box<EagerVec<I, T>>>,
}

const VERSION: Version = Version::ZERO;

impl<I, T> ComputedVecBuilder<I, T>
where
    I: StoredIndex,
    T: ComputedType,
{
    pub fn forced_import(
        path: &Path,
        name: &str,
        version: Version,
        format: Format,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let only_one_active = options.is_only_one_active();

        let prefix = |s: &str| format!("{s}_{name}");

        let maybe_prefix = |s: &str| {
            if only_one_active {
                name.to_string()
            } else {
                prefix(s)
            }
        };

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
                        path,
                        &maybe_prefix("first"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            last: options.last.then(|| {
                Box::new(
                    EagerVec::forced_import(path, name, version + Version::ZERO, format).unwrap(),
                )
            }),
            min: options.min.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        path,
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
                        path,
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
                        path,
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
                        path,
                        &maybe_suffix("average"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            sum: options.sum.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        path,
                        &maybe_suffix("sum"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            cumulative: options.cumulative.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        path,
                        &prefix("cumulative"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            _90p: options._90p.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        path,
                        &maybe_suffix("90p"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            _75p: options._75p.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        path,
                        &maybe_suffix("75p"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            _25p: options._25p.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        path,
                        &maybe_suffix("25p"),
                        version + VERSION + Version::ZERO,
                        format,
                    )
                    .unwrap(),
                )
            }),
            _10p: options._10p.then(|| {
                Box::new(
                    EagerVec::forced_import(
                        path,
                        &maybe_suffix("10p"),
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

        let index = self.starting_index(max_from);

        let cumulative_vec = self.cumulative.as_mut().unwrap();

        let mut cumulative = index.decremented().map_or(T::from(0_usize), |index| {
            cumulative_vec.iter().unwrap_get_inner(index)
        });
        source.iter_at(index).try_for_each(|(i, v)| -> Result<()> {
            cumulative = cumulative.clone() + v.into_inner();
            cumulative_vec.forced_push_at(i, cumulative.clone(), exit)
        })?;

        self.safe_flush(exit)?;

        Ok(())
    }

    pub fn compute<I2>(
        &mut self,
        max_from: I,
        source: &impl AnyIterableVec<I2, T>,
        first_indexes: &impl AnyIterableVec<I, I2>,
        count_indexes: &impl AnyIterableVec<I, StoredUsize>,
        exit: &Exit,
    ) -> Result<()>
    where
        I2: StoredIndex + StoredType + CheckedSub<I2>,
    {
        self.validate_computed_version_or_reset_file(
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
            .try_for_each(|(i, first_index)| -> Result<()> {
                let first_index = first_index.into_inner();

                let count_index = count_indexes_iter.unwrap_get_inner(i);

                if let Some(first) = self.first.as_mut() {
                    let f = source_iter
                        .get_inner(first_index)
                        .unwrap_or_else(|| T::from(0_usize));
                    first.forced_push_at(index, f, exit)?;
                }

                if let Some(last) = self.last.as_mut() {
                    let count_index = *count_index;
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
                    // .into_inner();
                    last.forced_push_at(index, v, exit)?;
                }

                let needs_sum_or_cumulative = self.sum.is_some() || self.cumulative.is_some();
                let needs_average_sum_or_cumulative =
                    needs_sum_or_cumulative || self.average.is_some();
                let needs_sorted = self.max.is_some()
                    || self._90p.is_some()
                    || self._75p.is_some()
                    || self.median.is_some()
                    || self._25p.is_some()
                    || self._10p.is_some()
                    || self.min.is_some();
                let needs_values = needs_sorted || needs_average_sum_or_cumulative;

                if needs_values {
                    source_iter.set(first_index);
                    let mut values = (&mut source_iter)
                        .take(*count_index)
                        .map(|(_, v)| v.into_inner())
                        .collect::<Vec<_>>();

                    if needs_sorted {
                        values.sort_unstable();

                        if let Some(max) = self.max.as_mut() {
                            max.forced_push_at(
                                i,
                                values
                                    .last()
                                    .context("expect some")
                                    .inspect_err(|_| {
                                        dbg!(
                                            &values,
                                            max.path(),
                                            first_indexes.name(),
                                            first_index,
                                            count_indexes.name(),
                                            count_index,
                                            source.len(),
                                            source.name()
                                        );
                                    })
                                    .unwrap()
                                    .clone(),
                                exit,
                            )?;
                        }

                        if let Some(_90p) = self._90p.as_mut() {
                            _90p.forced_push_at(i, get_percentile(&values, 0.90), exit)?;
                        }

                        if let Some(_75p) = self._75p.as_mut() {
                            _75p.forced_push_at(i, get_percentile(&values, 0.75), exit)?;
                        }

                        if let Some(median) = self.median.as_mut() {
                            median.forced_push_at(i, get_percentile(&values, 0.50), exit)?;
                        }

                        if let Some(_25p) = self._25p.as_mut() {
                            _25p.forced_push_at(i, get_percentile(&values, 0.25), exit)?;
                        }

                        if let Some(_10p) = self._10p.as_mut() {
                            _10p.forced_push_at(i, get_percentile(&values, 0.10), exit)?;
                        }

                        if let Some(min) = self.min.as_mut() {
                            min.forced_push_at(i, values.first().unwrap().clone(), exit)?;
                        }
                    }

                    if needs_average_sum_or_cumulative {
                        let len = values.len();
                        let sum = values.into_iter().fold(T::from(0), |a, b| a + b);

                        if let Some(average) = self.average.as_mut() {
                            let avg = sum.clone() / len;
                            average.forced_push_at(i, avg, exit)?;
                        }

                        if needs_sum_or_cumulative {
                            if let Some(sum_vec) = self.sum.as_mut() {
                                sum_vec.forced_push_at(i, sum.clone(), exit)?;
                            }

                            if let Some(cumulative_vec) = self.cumulative.as_mut() {
                                let t = cumulative.as_ref().unwrap().clone() + sum;
                                cumulative.replace(t.clone());
                                cumulative_vec.forced_push_at(i, t, exit)?;
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
        source: &ComputedVecBuilder<I2, T>,
        first_indexes: &impl AnyIterableVec<I, I2>,
        count_indexes: &impl AnyIterableVec<I, StoredUsize>,
        exit: &Exit,
    ) -> Result<()>
    where
        I2: StoredIndex + StoredType + CheckedSub<I2>,
    {
        if self._90p.is_some()
            || self._75p.is_some()
            || self.median.is_some()
            || self._25p.is_some()
            || self._10p.is_some()
        {
            panic!("unsupported");
        }

        self.validate_computed_version_or_reset_file(
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
            .try_for_each(|(i, first_index, ..)| -> Result<()> {
                let first_index = first_index.into_inner();

                let count_index = count_indexes_iter.unwrap_get_inner(i);

                if let Some(first) = self.first.as_mut() {
                    let v = source_first_iter
                        .as_mut()
                        .unwrap()
                        .unwrap_get_inner(first_index);
                    first.forced_push_at(index, v, exit)?;
                }

                if let Some(last) = self.last.as_mut() {
                    let count_index = *count_index;
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
                                .take(*count_index)
                                .map(|(_, v)| v.into_inner())
                                .collect::<Vec<_>>();
                            values.sort_unstable();
                            max.forced_push_at(i, values.last().unwrap().clone(), exit)?;
                        }

                        if let Some(min) = self.min.as_mut() {
                            let source_min_iter = source_min_iter.as_mut().unwrap();
                            source_min_iter.set(first_index);
                            let mut values = source_min_iter
                                .take(*count_index)
                                .map(|(_, v)| v.into_inner())
                                .collect::<Vec<_>>();
                            values.sort_unstable();
                            min.forced_push_at(i, values.first().unwrap().clone(), exit)?;
                        }
                    }

                    if needs_average_sum_or_cumulative {
                        if let Some(average) = self.average.as_mut() {
                            let source_average_iter = source_average_iter.as_mut().unwrap();
                            source_average_iter.set(first_index);
                            let values = source_average_iter
                                .take(*count_index)
                                .map(|(_, v)| v.into_inner())
                                .collect::<Vec<_>>();

                            let len = values.len();
                            let cumulative = values.into_iter().fold(T::from(0), |a, b| a + b);
                            // TODO: Multiply by count then divide by cumulative
                            // Right now it's not 100% accurate as there could be more or less elements in the lower timeframe (28 days vs 31 days in a month for example)
                            let avg = cumulative / len;
                            average.forced_push_at(i, avg, exit)?;
                        }

                        if needs_sum_or_cumulative {
                            let source_sum_iter = source_sum_iter.as_mut().unwrap();
                            source_sum_iter.set(first_index);
                            let values = source_sum_iter
                                .take(*count_index)
                                .map(|(_, v)| v.into_inner())
                                .collect::<Vec<_>>();

                            let sum = values.into_iter().fold(T::from(0), |a, b| a + b);

                            if let Some(sum_vec) = self.sum.as_mut() {
                                sum_vec.forced_push_at(i, sum.clone(), exit)?;
                            }

                            if let Some(cumulative_vec) = self.cumulative.as_mut() {
                                let t = cumulative.as_ref().unwrap().clone() + sum;
                                cumulative.replace(t.clone());
                                cumulative_vec.forced_push_at(i, t, exit)?;
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
            self.vecs().into_iter().map(|v| v.len()).min().unwrap(),
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
    pub fn unwrap_90p(&self) -> &EagerVec<I, T> {
        self._90p.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_75p(&self) -> &EagerVec<I, T> {
        self._75p.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_median(&self) -> &EagerVec<I, T> {
        self.median.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_25p(&self) -> &EagerVec<I, T> {
        self._25p.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_10p(&self) -> &EagerVec<I, T> {
        self._10p.as_ref().unwrap()
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

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        let mut v: Vec<&dyn AnyCollectableVec> = vec![];

        if let Some(first) = self.first.as_ref() {
            v.push(first.as_ref());
        }
        if let Some(last) = self.last.as_ref() {
            v.push(last.as_ref());
        }
        if let Some(min) = self.min.as_ref() {
            v.push(min.as_ref());
        }
        if let Some(max) = self.max.as_ref() {
            v.push(max.as_ref());
        }
        if let Some(median) = self.median.as_ref() {
            v.push(median.as_ref());
        }
        if let Some(average) = self.average.as_ref() {
            v.push(average.as_ref());
        }
        if let Some(sum) = self.sum.as_ref() {
            v.push(sum.as_ref());
        }
        if let Some(cumulative) = self.cumulative.as_ref() {
            v.push(cumulative.as_ref());
        }
        if let Some(_90p) = self._90p.as_ref() {
            v.push(_90p.as_ref());
        }
        if let Some(_75p) = self._75p.as_ref() {
            v.push(_75p.as_ref());
        }
        if let Some(_25p) = self._25p.as_ref() {
            v.push(_25p.as_ref());
        }
        if let Some(_10p) = self._10p.as_ref() {
            v.push(_10p.as_ref());
        }

        v
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
        if let Some(_90p) = self._90p.as_mut() {
            _90p.safe_flush(exit)?;
        }
        if let Some(_75p) = self._75p.as_mut() {
            _75p.safe_flush(exit)?;
        }
        if let Some(_25p) = self._25p.as_mut() {
            _25p.safe_flush(exit)?;
        }
        if let Some(_10p) = self._10p.as_mut() {
            _10p.safe_flush(exit)?;
        }

        Ok(())
    }

    pub fn validate_computed_version_or_reset_file(&mut self, version: Version) -> Result<()> {
        if let Some(first) = self.first.as_mut() {
            first.validate_computed_version_or_reset_file(Version::ZERO + version)?;
        }
        if let Some(last) = self.last.as_mut() {
            last.validate_computed_version_or_reset_file(Version::ZERO + version)?;
        }
        if let Some(min) = self.min.as_mut() {
            min.validate_computed_version_or_reset_file(Version::ZERO + version)?;
        }
        if let Some(max) = self.max.as_mut() {
            max.validate_computed_version_or_reset_file(Version::ZERO + version)?;
        }
        if let Some(median) = self.median.as_mut() {
            median.validate_computed_version_or_reset_file(Version::ZERO + version)?;
        }
        if let Some(average) = self.average.as_mut() {
            average.validate_computed_version_or_reset_file(Version::ZERO + version)?;
        }
        if let Some(sum) = self.sum.as_mut() {
            sum.validate_computed_version_or_reset_file(Version::ZERO + version)?;
        }
        if let Some(cumulative) = self.cumulative.as_mut() {
            cumulative.validate_computed_version_or_reset_file(Version::ZERO + version)?;
        }
        if let Some(_90p) = self._90p.as_mut() {
            _90p.validate_computed_version_or_reset_file(Version::ZERO + version)?;
        }
        if let Some(_75p) = self._75p.as_mut() {
            _75p.validate_computed_version_or_reset_file(Version::ZERO + version)?;
        }
        if let Some(_25p) = self._25p.as_mut() {
            _25p.validate_computed_version_or_reset_file(Version::ZERO + version)?;
        }
        if let Some(_10p) = self._10p.as_mut() {
            _10p.validate_computed_version_or_reset_file(Version::ZERO + version)?;
        }

        Ok(())
    }
}

#[derive(Default, Clone, Copy)]
pub struct StorableVecGeneatorOptions {
    average: bool,
    sum: bool,
    max: bool,
    _90p: bool,
    _75p: bool,
    median: bool,
    _25p: bool,
    _10p: bool,
    min: bool,
    first: bool,
    last: bool,
    cumulative: bool,
}

impl StorableVecGeneatorOptions {
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
    pub fn add_90p(mut self) -> Self {
        self._90p = true;
        self
    }

    #[allow(unused)]
    pub fn add_75p(mut self) -> Self {
        self._75p = true;
        self
    }

    #[allow(unused)]
    pub fn add_25p(mut self) -> Self {
        self._25p = true;
        self
    }

    #[allow(unused)]
    pub fn add_10p(mut self) -> Self {
        self._10p = true;
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
    pub fn rm_90p(mut self) -> Self {
        self._90p = false;
        self
    }

    #[allow(unused)]
    pub fn rm_75p(mut self) -> Self {
        self._75p = false;
        self
    }

    #[allow(unused)]
    pub fn rm_25p(mut self) -> Self {
        self._25p = false;
        self
    }

    #[allow(unused)]
    pub fn rm_10p(mut self) -> Self {
        self._10p = false;
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
        self._90p = true;
        self._75p = true;
        self.median = true;
        self._25p = true;
        self._10p = true;
        self
    }

    pub fn remove_percentiles(mut self) -> Self {
        self._90p = false;
        self._75p = false;
        self.median = false;
        self._25p = false;
        self._10p = false;
        self
    }

    pub fn is_only_one_active(&self) -> bool {
        [
            self.average,
            self.sum,
            self.max,
            self._90p,
            self._75p,
            self.median,
            self._25p,
            self._10p,
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
