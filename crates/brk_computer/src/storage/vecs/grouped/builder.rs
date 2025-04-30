use std::path::Path;

use brk_core::{CheckedSub, StoredUsize};
use brk_exit::Exit;
use brk_vec::{Compressed, DynamicVec, Result, StoredIndex, StoredType, StoredVec, Version};
use color_eyre::eyre::ContextCompat;

use crate::storage::{ComputedType, EagerVec};

#[derive(Clone, Debug)]
pub struct ComputedVecBuilder<I, T>
where
    I: StoredIndex,
    T: ComputedType,
{
    first: Option<EagerVec<I, T>>,
    average: Option<EagerVec<I, T>>,
    sum: Option<EagerVec<I, T>>,
    max: Option<EagerVec<I, T>>,
    _90p: Option<EagerVec<I, T>>,
    _75p: Option<EagerVec<I, T>>,
    median: Option<EagerVec<I, T>>,
    _25p: Option<EagerVec<I, T>>,
    _10p: Option<EagerVec<I, T>>,
    min: Option<EagerVec<I, T>>,
    last: Option<EagerVec<I, T>>,
    total: Option<EagerVec<I, T>>,
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
        compressed: Compressed,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let key = I::to_string().split("::").last().unwrap().to_lowercase();

        let only_one_active = options.is_only_one_active();

        let default = || path.join(format!("{key}_to_{name}"));

        let prefix = |s: &str| path.join(format!("{key}_to_{s}_{name}"));

        let maybe_prefix = |s: &str| {
            if only_one_active {
                default()
            } else {
                prefix(s)
            }
        };

        let suffix = |s: &str| path.join(format!("{key}_to_{name}_{s}"));

        let maybe_suffix = |s: &str| {
            if only_one_active {
                default()
            } else {
                suffix(s)
            }
        };

        let version = VERSION + version;

        let s = Self {
            first: options.first.then(|| {
                EagerVec::forced_import(&maybe_prefix("first"), version + Version::ZERO, compressed)
                    .unwrap()
            }),
            last: options.last.then(|| {
                EagerVec::forced_import(
                    &path.join(format!("{key}_to_{name}")),
                    version + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            min: options.min.then(|| {
                EagerVec::forced_import(&maybe_suffix("min"), version + Version::ZERO, compressed)
                    .unwrap()
            }),
            max: options.max.then(|| {
                EagerVec::forced_import(&maybe_suffix("max"), version + Version::ZERO, compressed)
                    .unwrap()
            }),
            median: options.median.then(|| {
                EagerVec::forced_import(
                    &maybe_suffix("median"),
                    version + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            average: options.average.then(|| {
                EagerVec::forced_import(
                    &maybe_suffix("average"),
                    version + Version::ZERO,
                    compressed,
                )
                .unwrap()
            }),
            sum: options.sum.then(|| {
                EagerVec::forced_import(&maybe_suffix("sum"), version + Version::ZERO, compressed)
                    .unwrap()
            }),
            total: options.total.then(|| {
                EagerVec::forced_import(&prefix("total"), version + Version::ZERO, compressed)
                    .unwrap()
            }),
            _90p: options._90p.then(|| {
                EagerVec::forced_import(&maybe_suffix("90p"), version + Version::ZERO, compressed)
                    .unwrap()
            }),
            _75p: options._75p.then(|| {
                EagerVec::forced_import(&maybe_suffix("75p"), version + Version::ZERO, compressed)
                    .unwrap()
            }),
            _25p: options._25p.then(|| {
                EagerVec::forced_import(&maybe_suffix("25p"), version + Version::ZERO, compressed)
                    .unwrap()
            }),
            _10p: options._10p.then(|| {
                EagerVec::forced_import(&maybe_suffix("10p"), version + Version::ZERO, compressed)
                    .unwrap()
            }),
        };

        Ok(s)
    }

    pub fn extend(&mut self, max_from: I, source: &StoredVec<I, T>, exit: &Exit) -> Result<()> {
        if self.total.is_none() {
            return Ok(());
        };

        let index = self.starting_index(max_from);

        let total_vec = self.total.as_mut().unwrap();

        let mut total = index.decremented().map_or(T::from(0_usize), |index| {
            total_vec.iter().unwrap_get_inner(index)
        });
        source.iter_at(index).try_for_each(|(i, v)| -> Result<()> {
            total = total.clone() + v.into_inner();
            total_vec.forced_push_at(i, total.clone(), exit)
        })?;

        self.safe_flush(exit)?;

        Ok(())
    }

    pub fn compute<I2>(
        &mut self,
        max_from: I,
        source: &StoredVec<I2, T>,
        first_indexes: &StoredVec<I, I2>,
        count_indexes: &StoredVec<I, StoredUsize>,
        exit: &Exit,
    ) -> Result<()>
    where
        I2: StoredIndex + StoredType + CheckedSub<I2>,
    {
        let index = self.starting_index(max_from);

        let mut count_indexes_iter = count_indexes.iter();
        let mut source_iter = source.iter();

        let total_vec = self.total.as_mut();

        let mut total = total_vec.map(|total_vec| {
            index.decremented().map_or(T::from(0_usize), |index| {
                total_vec.iter().unwrap_get_inner(index)
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

                let needs_sum_or_total = self.sum.is_some() || self.total.is_some();
                let needs_average_sum_or_total = needs_sum_or_total || self.average.is_some();
                let needs_sorted = self.max.is_some()
                    || self._90p.is_some()
                    || self._75p.is_some()
                    || self.median.is_some()
                    || self._25p.is_some()
                    || self._10p.is_some()
                    || self.min.is_some();
                let needs_values = needs_sorted || needs_average_sum_or_total;

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
                                            first_indexes.path(),
                                            first_index,
                                            count_indexes.path(),
                                            count_index,
                                            source.len(),
                                            source.path()
                                        );
                                    })
                                    .unwrap()
                                    .clone(),
                                exit,
                            )?;
                        }

                        if let Some(_90p) = self._90p.as_mut() {
                            _90p.forced_push_at(i, Self::get_percentile(&values, 0.90), exit)?;
                        }

                        if let Some(_75p) = self._75p.as_mut() {
                            _75p.forced_push_at(i, Self::get_percentile(&values, 0.75), exit)?;
                        }

                        if let Some(median) = self.median.as_mut() {
                            median.forced_push_at(i, Self::get_percentile(&values, 0.50), exit)?;
                        }

                        if let Some(_25p) = self._25p.as_mut() {
                            _25p.forced_push_at(i, Self::get_percentile(&values, 0.25), exit)?;
                        }

                        if let Some(_10p) = self._10p.as_mut() {
                            _10p.forced_push_at(i, Self::get_percentile(&values, 0.10), exit)?;
                        }

                        if let Some(min) = self.min.as_mut() {
                            min.forced_push_at(i, values.first().unwrap().clone(), exit)?;
                        }
                    }

                    if needs_average_sum_or_total {
                        let len = values.len();
                        let sum = values.into_iter().fold(T::from(0), |a, b| a + b);

                        if let Some(average) = self.average.as_mut() {
                            let avg = sum.clone() / len;
                            average.forced_push_at(i, avg, exit)?;
                        }

                        if needs_sum_or_total {
                            if let Some(sum_vec) = self.sum.as_mut() {
                                sum_vec.forced_push_at(i, sum.clone(), exit)?;
                            }

                            if let Some(total_vec) = self.total.as_mut() {
                                let t = total.as_ref().unwrap().clone() + sum;
                                total.replace(t.clone());
                                total_vec.forced_push_at(i, t, exit)?;
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
        first_indexes: &StoredVec<I, I2>,
        count_indexes: &StoredVec<I, StoredUsize>,
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

        let index = self.starting_index(max_from);

        let mut count_indexes_iter = count_indexes.iter();

        let mut source_first_iter = source.first.as_ref().map(|f| f.iter());
        let mut source_last_iter = source.last.as_ref().map(|f| f.iter());
        let mut source_max_iter = source.max.as_ref().map(|f| f.iter());
        let mut source_min_iter = source.min.as_ref().map(|f| f.iter());
        let mut source_average_iter = source.average.as_ref().map(|f| f.iter());
        let mut source_sum_iter = source.sum.as_ref().map(|f| f.iter());

        let mut total = self.total.as_mut().map(|total_vec| {
            index.decremented().map_or(T::from(0_usize), |index| {
                total_vec.iter().unwrap_get_inner(index)
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

                let needs_sum_or_total = self.sum.is_some() || self.total.is_some();
                let needs_average_sum_or_total = needs_sum_or_total || self.average.is_some();
                let needs_sorted = self.max.is_some() || self.min.is_some();
                let needs_values = needs_sorted || needs_average_sum_or_total;

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

                    if needs_average_sum_or_total {
                        if let Some(average) = self.average.as_mut() {
                            let source_average_iter = source_average_iter.as_mut().unwrap();
                            source_average_iter.set(first_index);
                            let values = source_average_iter
                                .take(*count_index)
                                .map(|(_, v)| v.into_inner())
                                .collect::<Vec<_>>();

                            let len = values.len();
                            let total = values.into_iter().fold(T::from(0), |a, b| a + b);
                            // TODO: Multiply by count then divide by total
                            // Right now it's not 100% accurate as there could be more or less elements in the lower timeframe (28 days vs 31 days in a month for example)
                            let avg = total / len;
                            average.forced_push_at(i, avg, exit)?;
                        }

                        if needs_sum_or_total {
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

                            if let Some(total_vec) = self.total.as_mut() {
                                let t = total.as_ref().unwrap().clone() + sum;
                                total.replace(t.clone());
                                total_vec.forced_push_at(i, t, exit)?;
                            }
                        }
                    }
                }

                Ok(())
            })?;

        self.safe_flush(exit)?;

        Ok(())
    }

    fn get_percentile(sorted: &[T], percentile: f64) -> T {
        let len = sorted.len();

        if len == 0 {
            panic!();
        } else if len == 1 {
            sorted[0].clone()
        } else {
            let index = (len - 1) as f64 * percentile;

            let fract = index.fract();

            if fract != 0.0 {
                let left = sorted.get(index as usize).unwrap().clone();
                let right = sorted.get(index.ceil() as usize).unwrap().clone();
                left / 2 + right / 2
            } else {
                sorted.get(index as usize).unwrap().clone()
            }
        }
    }

    fn starting_index(&self, max_from: I) -> I {
        max_from.min(I::from(
            self.any_vecs().into_iter().map(|v| v.len()).min().unwrap(),
        ))
    }

    pub fn unwrap_first(&mut self) -> &mut EagerVec<I, T> {
        self.first.as_mut().unwrap()
    }
    pub fn unwrap_average(&mut self) -> &mut EagerVec<I, T> {
        self.average.as_mut().unwrap()
    }
    pub fn unwrap_sum(&mut self) -> &mut EagerVec<I, T> {
        self.sum.as_mut().unwrap()
    }
    pub fn unwrap_max(&mut self) -> &mut EagerVec<I, T> {
        self.max.as_mut().unwrap()
    }
    pub fn unwrap_90p(&mut self) -> &mut EagerVec<I, T> {
        self._90p.as_mut().unwrap()
    }
    pub fn unwrap_75p(&mut self) -> &mut EagerVec<I, T> {
        self._75p.as_mut().unwrap()
    }
    pub fn unwrap_median(&mut self) -> &mut EagerVec<I, T> {
        self.median.as_mut().unwrap()
    }
    pub fn unwrap_25p(&mut self) -> &mut EagerVec<I, T> {
        self._25p.as_mut().unwrap()
    }
    pub fn unwrap_10p(&mut self) -> &mut EagerVec<I, T> {
        self._10p.as_mut().unwrap()
    }
    pub fn unwrap_min(&mut self) -> &mut EagerVec<I, T> {
        self.min.as_mut().unwrap()
    }
    pub fn unwrap_last(&mut self) -> &mut EagerVec<I, T> {
        self.last.as_mut().unwrap()
    }
    pub fn unwrap_total(&mut self) -> &mut EagerVec<I, T> {
        self.total.as_mut().unwrap()
    }

    pub fn any_vecs(&self) -> Vec<&dyn brk_vec::AnyStoredVec> {
        let mut v: Vec<&dyn brk_vec::AnyStoredVec> = vec![];

        if let Some(first) = self.first.as_ref() {
            v.push(first.any_vec());
        }
        if let Some(last) = self.last.as_ref() {
            v.push(last.any_vec());
        }
        if let Some(min) = self.min.as_ref() {
            v.push(min.any_vec());
        }
        if let Some(max) = self.max.as_ref() {
            v.push(max.any_vec());
        }
        if let Some(median) = self.median.as_ref() {
            v.push(median.any_vec());
        }
        if let Some(average) = self.average.as_ref() {
            v.push(average.any_vec());
        }
        if let Some(sum) = self.sum.as_ref() {
            v.push(sum.any_vec());
        }
        if let Some(total) = self.total.as_ref() {
            v.push(total.any_vec());
        }
        if let Some(_90p) = self._90p.as_ref() {
            v.push(_90p.any_vec());
        }
        if let Some(_75p) = self._75p.as_ref() {
            v.push(_75p.any_vec());
        }
        if let Some(_25p) = self._25p.as_ref() {
            v.push(_25p.any_vec());
        }
        if let Some(_10p) = self._10p.as_ref() {
            v.push(_10p.any_vec());
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
        if let Some(total) = self.total.as_mut() {
            total.safe_flush(exit)?;
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
    total: bool,
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

    pub fn add_90p(mut self) -> Self {
        self._90p = true;
        self
    }

    pub fn add_75p(mut self) -> Self {
        self._75p = true;
        self
    }

    pub fn add_25p(mut self) -> Self {
        self._25p = true;
        self
    }

    pub fn add_10p(mut self) -> Self {
        self._10p = true;
        self
    }

    pub fn add_total(mut self) -> Self {
        self.total = true;
        self
    }

    pub fn rm_min(mut self) -> Self {
        self.min = false;
        self
    }

    pub fn rm_max(mut self) -> Self {
        self.max = false;
        self
    }

    pub fn rm_median(mut self) -> Self {
        self.median = false;
        self
    }

    pub fn rm_average(mut self) -> Self {
        self.average = false;
        self
    }

    pub fn rm_sum(mut self) -> Self {
        self.sum = false;
        self
    }

    pub fn rm_90p(mut self) -> Self {
        self._90p = false;
        self
    }

    pub fn rm_75p(mut self) -> Self {
        self._75p = false;
        self
    }

    pub fn rm_25p(mut self) -> Self {
        self._25p = false;
        self
    }

    pub fn rm_10p(mut self) -> Self {
        self._10p = false;
        self
    }

    pub fn rm_total(mut self) -> Self {
        self.total = false;
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
            self.total,
        ]
        .iter()
        .filter(|b| **b)
        .count()
            == 1
    }

    pub fn copy_self_extra(&self) -> Self {
        Self {
            total: self.total,
            ..Self::default()
        }
    }
}
