use brk_error::Result;

use brk_structs::Version;
use vecdb::{
    AnyBoxedIterableVec, AnyCloneableIterableVec, AnyCollectableVec, AnyIterableVec, Computation,
    ComputedVec, ComputedVecFrom2, Database, Exit, Format, FromCoarserIndex, StoredIndex,
};

use crate::grouped::{EagerVecBuilder, VecBuilderOptions};

use super::ComputedType;

#[allow(clippy::type_complexity)]
#[derive(Clone)]
pub struct ComputedVecBuilder<I, T, S1I, S2T>
where
    I: StoredIndex,
    T: ComputedType,
    S2T: ComputedType,
{
    pub first: Option<Box<ComputedVecFrom2<I, T, S1I, T, I, S2T>>>,
    pub average: Option<Box<ComputedVecFrom2<I, T, S1I, T, I, S2T>>>,
    pub sum: Option<Box<ComputedVecFrom2<I, T, S1I, T, I, S2T>>>,
    pub max: Option<Box<ComputedVecFrom2<I, T, S1I, T, I, S2T>>>,
    pub min: Option<Box<ComputedVecFrom2<I, T, S1I, T, I, S2T>>>,
    pub last: Option<Box<ComputedVecFrom2<I, T, S1I, T, I, S2T>>>,
    pub cumulative: Option<Box<ComputedVecFrom2<I, T, S1I, T, I, S2T>>>,
}

const VERSION: Version = Version::ZERO;

impl<I, T, S1I, S2T> ComputedVecBuilder<I, T, S1I, S2T>
where
    I: StoredIndex,
    T: ComputedType + 'static,
    S1I: StoredIndex + 'static + FromCoarserIndex<I>,
    S2T: ComputedType,
{
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        format: Format,
        computation: Computation,
        source: Option<AnyBoxedIterableVec<S1I, T>>,
        source_extra: &EagerVecBuilder<S1I, T>,
        len_source: AnyBoxedIterableVec<I, S2T>,
        options: ComputedVecBuilderOptions,
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

        Ok(Self {
            first: options.first.then(|| {
                Box::new(
                    ComputedVec::forced_import_or_init_from_2(
                        computation,
                        db,
                        &maybe_suffix("first"),
                        version + VERSION + Version::ZERO,
                        format,
                        source_extra
                            .first
                            .as_ref()
                            .map_or_else(|| source.as_ref().unwrap().clone(), |v| v.clone()),
                        len_source.clone(),
                        |i: I, source, len_source| {
                            if i.unwrap_to_usize() >= len_source.len() {
                                return None;
                            }
                            source
                                .next_at(S1I::min_from(i))
                                .map(|(_, cow)| cow.into_owned())
                        },
                    )
                    .unwrap(),
                )
            }),
            last: options.last.then(|| {
                Box::new(
                    ComputedVec::forced_import_or_init_from_2(
                        computation,
                        db,
                        name,
                        version + VERSION + Version::ZERO,
                        format,
                        source_extra.last.as_ref().map_or_else(
                            || {
                                source
                                    .as_ref()
                                    .unwrap_or_else(|| {
                                        dbg!(db, name, I::to_string());
                                        panic!()
                                    })
                                    .clone()
                            },
                            |v| v.clone(),
                        ),
                        len_source.clone(),
                        |i: I, source, len_source| {
                            if i.unwrap_to_usize() >= len_source.len() {
                                return None;
                            }
                            source
                                .next_at(S1I::max_from(i, source.len()))
                                .map(|(_, cow)| cow.into_owned())
                        },
                    )
                    .unwrap(),
                )
            }),
            min: options.min.then(|| {
                Box::new(
                    ComputedVec::forced_import_or_init_from_2(
                        computation,
                        db,
                        &maybe_suffix("min"),
                        version + VERSION + Version::ZERO,
                        format,
                        source_extra
                            .min
                            .as_ref()
                            .map_or_else(|| source.as_ref().unwrap().clone(), |v| v.clone()),
                        len_source.clone(),
                        |i: I, source, len_source| {
                            if i.unwrap_to_usize() >= len_source.len() {
                                return None;
                            }
                            S1I::inclusive_range_from(i, source.len())
                                .flat_map(|i| source.next_at(i).map(|(_, cow)| cow.into_owned()))
                                .min()
                        },
                    )
                    .unwrap(),
                )
            }),
            max: options.max.then(|| {
                Box::new(
                    ComputedVec::forced_import_or_init_from_2(
                        computation,
                        db,
                        &maybe_suffix("max"),
                        version + VERSION + Version::ZERO,
                        format,
                        source_extra
                            .max
                            .as_ref()
                            .map_or_else(|| source.as_ref().unwrap().clone(), |v| v.clone()),
                        len_source.clone(),
                        |i: I, source, len_source| {
                            if i.unwrap_to_usize() >= len_source.len() {
                                return None;
                            }
                            S1I::inclusive_range_from(i, source.len())
                                .flat_map(|i| source.next_at(i).map(|(_, cow)| cow.into_owned()))
                                .max()
                        },
                    )
                    .unwrap(),
                )
            }),
            average: options.average.then(|| {
                Box::new(
                    ComputedVec::forced_import_or_init_from_2(
                        computation,
                        db,
                        &maybe_suffix("average"),
                        version + VERSION + Version::ZERO,
                        format,
                        source_extra
                            .average
                            .as_ref()
                            .map_or_else(|| source.as_ref().unwrap().clone(), |v| v.clone()),
                        len_source.clone(),
                        |i: I, source, len_source| {
                            if i.unwrap_to_usize() >= len_source.len() {
                                return None;
                            }
                            let vec = S1I::inclusive_range_from(i, source.len())
                                .flat_map(|i| source.next_at(i).map(|(_, cow)| cow.into_owned()))
                                .collect::<Vec<_>>();
                            if vec.is_empty() {
                                return None;
                            }
                            let mut sum = T::from(0);
                            let len = vec.len();
                            vec.into_iter().for_each(|v| sum += v);
                            Some(sum / len)
                        },
                    )
                    .unwrap(),
                )
            }),
            sum: options.sum.then(|| {
                Box::new(
                    ComputedVec::forced_import_or_init_from_2(
                        computation,
                        db,
                        &(if !options.last && !options.average && !options.min && !options.max {
                            name.to_string()
                        } else {
                            maybe_suffix("sum")
                        }),
                        version + VERSION + Version::ZERO,
                        format,
                        source_extra
                            .sum
                            .as_ref()
                            .map_or_else(|| source.as_ref().unwrap().clone(), |v| v.clone()),
                        len_source.clone(),
                        |i: I, source, len_source| {
                            if i.unwrap_to_usize() >= len_source.len() {
                                return None;
                            }
                            let vec = S1I::inclusive_range_from(i, source.len())
                                .flat_map(|i| source.next_at(i).map(|(_, cow)| cow.into_owned()))
                                .collect::<Vec<_>>();
                            if vec.is_empty() {
                                return None;
                            }
                            let mut sum = T::from(0);
                            vec.into_iter().for_each(|v| sum += v);
                            Some(sum)
                        },
                    )
                    .unwrap(),
                )
            }),
            cumulative: options.cumulative.then(|| {
                Box::new(
                    ComputedVec::forced_import_or_init_from_2(
                        computation,
                        db,
                        &suffix("cumulative"),
                        version + VERSION + Version::ZERO,
                        format,
                        source_extra.cumulative.as_ref().unwrap().boxed_clone(),
                        len_source.clone(),
                        |i: I, source, len_source| {
                            if i.unwrap_to_usize() >= len_source.len() {
                                return None;
                            }
                            source
                                .next_at(S1I::max_from(i, source.len()))
                                .map(|(_, cow)| cow.into_owned())
                        },
                    )
                    .unwrap(),
                )
            }),
        })
    }

    pub fn compute_if_necessary<T2>(
        &mut self,
        max_from: I,
        len_source: &impl AnyIterableVec<I, T2>,
        exit: &Exit,
    ) -> Result<()> {
        if let Some(first) = self.first.as_mut() {
            first.compute_if_necessary(max_from, len_source, exit)?;
        }
        if let Some(last) = self.last.as_mut() {
            last.compute_if_necessary(max_from, len_source, exit)?;
        }
        if let Some(min) = self.min.as_mut() {
            min.compute_if_necessary(max_from, len_source, exit)?;
        }
        if let Some(max) = self.max.as_mut() {
            max.compute_if_necessary(max_from, len_source, exit)?;
        }
        if let Some(average) = self.average.as_mut() {
            average.compute_if_necessary(max_from, len_source, exit)?;
        }
        if let Some(sum) = self.sum.as_mut() {
            sum.compute_if_necessary(max_from, len_source, exit)?;
        }
        if let Some(cumulative) = self.cumulative.as_mut() {
            cumulative.compute_if_necessary(max_from, len_source, exit)?;
        }

        Ok(())
    }

    pub fn starting_index(&self, max_from: I) -> I {
        max_from.min(I::from(
            self.vecs().into_iter().map(|v| v.len()).min().unwrap(),
        ))
    }

    pub fn unwrap_first(&self) -> &ComputedVecFrom2<I, T, S1I, T, I, S2T> {
        self.first.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_average(&self) -> &ComputedVecFrom2<I, T, S1I, T, I, S2T> {
        self.average.as_ref().unwrap()
    }
    pub fn unwrap_sum(&self) -> &ComputedVecFrom2<I, T, S1I, T, I, S2T> {
        self.sum.as_ref().unwrap()
    }
    pub fn unwrap_max(&self) -> &ComputedVecFrom2<I, T, S1I, T, I, S2T> {
        self.max.as_ref().unwrap()
    }
    pub fn unwrap_min(&self) -> &ComputedVecFrom2<I, T, S1I, T, I, S2T> {
        self.min.as_ref().unwrap()
    }
    pub fn unwrap_last(&self) -> &ComputedVecFrom2<I, T, S1I, T, I, S2T> {
        self.last.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_cumulative(&self) -> &ComputedVecFrom2<I, T, S1I, T, I, S2T> {
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
        if let Some(average) = self.average.as_ref() {
            v.push(average.as_ref());
        }
        if let Some(sum) = self.sum.as_ref() {
            v.push(sum.as_ref());
        }
        if let Some(cumulative) = self.cumulative.as_ref() {
            v.push(cumulative.as_ref());
        }

        v
    }
}

#[derive(Default, Clone, Copy)]
pub struct ComputedVecBuilderOptions {
    average: bool,
    sum: bool,
    max: bool,
    min: bool,
    first: bool,
    last: bool,
    cumulative: bool,
}

impl From<VecBuilderOptions> for ComputedVecBuilderOptions {
    fn from(value: VecBuilderOptions) -> Self {
        Self {
            average: value.average(),
            sum: value.sum(),
            max: value.max(),
            min: value.min(),
            first: value.first(),
            last: value.last(),
            cumulative: value.cumulative(),
        }
    }
}

impl ComputedVecBuilderOptions {
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

    pub fn add_average(mut self) -> Self {
        self.average = true;
        self
    }

    pub fn add_sum(mut self) -> Self {
        self.sum = true;
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
    pub fn rm_cumulative(mut self) -> Self {
        self.cumulative = false;
        self
    }

    pub fn add_minmax(mut self) -> Self {
        self.min = true;
        self.max = true;
        self
    }

    pub fn is_only_one_active(&self) -> bool {
        [
            self.average,
            self.sum,
            self.max,
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
