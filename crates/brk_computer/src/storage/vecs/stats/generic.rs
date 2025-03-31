use std::path::Path;

use brk_exit::Exit;
use brk_vec::{AnyStorableVec, Compressed, Result, StoredIndex, StoredType, Version};

use crate::storage::vecs::base::StorableVec;

use super::ComputedType;

#[derive(Clone, Debug)]
pub struct StorableVecBuilder<I, T>
where
    I: StoredIndex,
    T: ComputedType,
{
    pub first: Option<StorableVec<I, T>>,
    pub average: Option<StorableVec<I, T>>,
    pub sum: Option<StorableVec<I, T>>,
    pub max: Option<StorableVec<I, T>>,
    pub _90p: Option<StorableVec<I, T>>,
    pub _75p: Option<StorableVec<I, T>>,
    pub median: Option<StorableVec<I, T>>,
    pub _25p: Option<StorableVec<I, T>>,
    pub _10p: Option<StorableVec<I, T>>,
    pub min: Option<StorableVec<I, T>>,
    pub last: Option<StorableVec<I, T>>,
}

impl<I, T> StorableVecBuilder<I, T>
where
    I: StoredIndex,
    T: ComputedType,
{
    pub fn forced_import(
        path: &Path,
        compressed: Compressed,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let key = I::to_string().split("::").last().unwrap().to_lowercase();

        let prefix = |s: &str| path.with_file_name(format!("{key}_to_{s}_{name}"));
        let suffix = |s: &str| path.with_file_name(format!("{key}_to_{name}_{s}"));

        let s = Self {
            first: options.first.then(|| {
                StorableVec::forced_import(&prefix("first"), Version::from(1), compressed).unwrap()
            }),
            last: options.last.then(|| {
                StorableVec::forced_import(
                    &path.with_file_name(format!("{key}_to_{name}")),
                    Version::from(1),
                    compressed,
                )
                .unwrap()
            }),
            min: options.min.then(|| {
                StorableVec::forced_import(&suffix("min"), Version::from(1), compressed).unwrap()
            }),
            max: options.max.then(|| {
                StorableVec::forced_import(&suffix("max"), Version::from(1), compressed).unwrap()
            }),
            median: options.median.then(|| {
                StorableVec::forced_import(&suffix("median"), Version::from(1), compressed).unwrap()
            }),
            average: options.average.then(|| {
                StorableVec::forced_import(&suffix("average"), Version::from(1), compressed)
                    .unwrap()
            }),
            sum: options.sum.then(|| {
                StorableVec::forced_import(&suffix("sum"), Version::from(1), compressed).unwrap()
            }),
            _90p: options._90p.then(|| {
                StorableVec::forced_import(&suffix("90p"), Version::from(1), compressed).unwrap()
            }),
            _75p: options._75p.then(|| {
                StorableVec::forced_import(&suffix("75p"), Version::from(1), compressed).unwrap()
            }),
            _25p: options._25p.then(|| {
                StorableVec::forced_import(&suffix("25p"), Version::from(1), compressed).unwrap()
            }),
            _10p: options._10p.then(|| {
                StorableVec::forced_import(&suffix("10p"), Version::from(1), compressed).unwrap()
            }),
        };

        Ok(s)
    }

    pub fn compute<I2>(
        &mut self,
        max_from: I,
        source: &mut StorableVec<I2, T>,
        first_indexes: &mut brk_vec::StorableVec<I, I2>,
        last_indexes: &mut brk_vec::StorableVec<I, I2>,
        exit: &Exit,
    ) -> Result<()>
    where
        I2: StoredIndex + StoredType,
        T: Ord + From<f64>,
        f64: From<T>,
    {
        let index = self.starting_index(max_from);

        first_indexes.iter_from(index, |(i, first_index)| {
            let first_index = *first_index;
            let last_index = *last_indexes.get(i).unwrap().unwrap();

            if let Some(first) = self.first.as_mut() {
                let v = source.get(first_index).unwrap().unwrap();
                first.forced_push_at(index, v.clone(), exit)?;
            }

            if let Some(last) = self.last.as_mut() {
                let v = source.get(last_index).unwrap().unwrap();
                last.forced_push_at(index, v.clone(), exit)?;
            }

            let first_index = first_index.to_usize()?;
            let last_index = last_index.to_usize()?;

            let needs_sum_or_average = self.sum.is_some() || self.average.is_some();
            let needs_sorted = self.max.is_some()
                || self._90p.is_some()
                || self._75p.is_some()
                || self.median.is_some()
                || self._25p.is_some()
                || self._10p.is_some()
                || self.min.is_some();
            let needs_values = needs_sorted || needs_sum_or_average;

            if needs_values {
                let mut values =
                    source.collect_range(Some(first_index as i64), Some(last_index as i64))?;

                if needs_sorted {
                    values.sort_unstable();

                    if let Some(max) = self.max.as_mut() {
                        max.forced_push_at(i, values.last().unwrap().clone(), exit)?;
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

                if needs_sum_or_average {
                    let len = values.len();

                    if let Some(average) = self.average.as_mut() {
                        let len = len as f64;
                        let total = values
                            .iter()
                            .map(|v| f64::from(v.clone()))
                            .fold(0.0, |a, b| a + b);
                        let avg = T::from(total / len);
                        average.forced_push_at(i, avg, exit)?;
                    }

                    if let Some(sum_vec) = self.sum.as_mut() {
                        let sum = values.into_iter().fold(T::from(0), |a, b| a + b);
                        sum_vec.forced_push_at(i, sum, exit)?;
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
        source: &mut StorableVecBuilder<I2, T>,
        first_indexes: &mut brk_vec::StorableVec<I, I2>,
        last_indexes: &mut brk_vec::StorableVec<I, I2>,
        exit: &Exit,
    ) -> Result<()>
    where
        I2: StoredIndex + StoredType,
        T: Ord + From<f64>,
        f64: From<T>,
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

        first_indexes.iter_from(index, |(i, first_index)| {
            let first_index = *first_index;
            let last_index = *last_indexes.get(i).unwrap().unwrap();

            if let Some(first) = self.first.as_mut() {
                let v = source
                    .first
                    .as_mut()
                    .unwrap()
                    .get(first_index)
                    .unwrap()
                    .cloned()
                    .unwrap();
                first.forced_push_at(index, v, exit)?;
            }

            if let Some(last) = self.last.as_mut() {
                let v = source
                    .last
                    .as_mut()
                    .unwrap()
                    .get(last_index)
                    .unwrap()
                    .cloned()
                    .unwrap();
                last.forced_push_at(index, v, exit)?;
            }

            let first_index = Some(first_index.to_usize()? as i64);
            let last_index = Some(last_index.to_usize()? as i64);

            let needs_sum_or_average = self.sum.is_some() || self.average.is_some();
            let needs_sorted = self.max.is_some() || self.min.is_some();
            let needs_values = needs_sorted || needs_sum_or_average;

            if needs_values {
                if needs_sorted {
                    if let Some(max) = self.max.as_mut() {
                        let mut values = source
                            .max
                            .as_ref()
                            .unwrap()
                            .collect_range(first_index, last_index)?;
                        values.sort_unstable();
                        max.forced_push_at(i, values.last().unwrap().clone(), exit)?;
                    }

                    if let Some(min) = self.min.as_mut() {
                        let mut values = source
                            .min
                            .as_ref()
                            .unwrap()
                            .collect_range(first_index, last_index)?;
                        values.sort_unstable();
                        min.forced_push_at(i, values.first().unwrap().clone(), exit)?;
                    }
                }

                if needs_sum_or_average {
                    if let Some(average) = self.average.as_mut() {
                        let values = source
                            .average
                            .as_ref()
                            .unwrap()
                            .collect_range(first_index, last_index)?;
                        let len = values.len() as f64;
                        let total = values
                            .into_iter()
                            .map(|v| f64::from(v))
                            .fold(0.0, |a, b| a + b);
                        // TODO: Multiply by count then divide by total
                        // Right now it's not 100% accurate as there could be more or less elements in the lower timeframe (28 days vs 31 days in a month for example)
                        let avg = T::from(total / len);
                        average.forced_push_at(i, avg, exit)?;
                    }

                    if let Some(sum_vec) = self.sum.as_mut() {
                        let values = source
                            .sum
                            .as_ref()
                            .unwrap()
                            .collect_range(first_index, last_index)?;
                        let sum = values.into_iter().fold(T::from(0), |a, b| a + b);
                        sum_vec.forced_push_at(i, sum, exit)?;
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
            self.as_any_vecs()
                .into_iter()
                .map(|v| v.len())
                .min()
                .unwrap(),
        ))
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        let mut v: Vec<&dyn AnyStorableVec> = vec![];

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
}
