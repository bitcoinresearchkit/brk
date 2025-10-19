use allocative::Allocative;
use brk_structs::Version;
use brk_traversable::Traversable;
use vecdb::{
    AnyBoxedIterableVec, AnyCloneableIterableVec, FromCoarserIndex, LazyVecFrom2, StoredIndex,
};

use crate::grouped::{EagerVecsBuilder, VecBuilderOptions};

use super::ComputedType;

#[allow(clippy::type_complexity)]
#[derive(Clone, Traversable, Allocative)]
pub struct LazyVecsBuilder<I, T, S1I, S2T>
where
    I: StoredIndex,
    T: ComputedType,
    S1I: StoredIndex,
    S2T: ComputedType,
{
    pub first: Option<Box<LazyVecFrom2<I, T, S1I, T, I, S2T>>>,
    pub average: Option<Box<LazyVecFrom2<I, T, S1I, T, I, S2T>>>,
    pub sum: Option<Box<LazyVecFrom2<I, T, S1I, T, I, S2T>>>,
    pub max: Option<Box<LazyVecFrom2<I, T, S1I, T, I, S2T>>>,
    pub min: Option<Box<LazyVecFrom2<I, T, S1I, T, I, S2T>>>,
    pub last: Option<Box<LazyVecFrom2<I, T, S1I, T, I, S2T>>>,
    pub cumulative: Option<Box<LazyVecFrom2<I, T, S1I, T, I, S2T>>>,
}

const VERSION: Version = Version::ZERO;

impl<I, T, S1I, S2T> LazyVecsBuilder<I, T, S1I, S2T>
where
    I: StoredIndex,
    T: ComputedType + 'static,
    S1I: StoredIndex + 'static + FromCoarserIndex<I>,
    S2T: ComputedType,
{
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        name: &str,
        version: Version,
        source: Option<AnyBoxedIterableVec<S1I, T>>,
        source_extra: &EagerVecsBuilder<S1I, T>,
        len_source: AnyBoxedIterableVec<I, S2T>,
        options: LazyVecBuilderOptions,
    ) -> Self {
        let only_one_active = options.is_only_one_active();

        let suffix = |s: &str| format!("{name}_{s}");

        let maybe_suffix = |s: &str| {
            if only_one_active {
                name.to_string()
            } else {
                suffix(s)
            }
        };

        Self {
            first: options.first.then(|| {
                Box::new(LazyVecFrom2::init(
                    &maybe_suffix("first"),
                    version + VERSION + Version::ZERO,
                    source_extra
                        .first
                        .as_ref()
                        .map_or_else(|| source.as_ref().unwrap().clone(), |v| v.clone()),
                    len_source.clone(),
                    |i: I, source, len_source| {
                        if i.to_usize() >= len_source.len() {
                            return None;
                        }
                        source
                            .next_at(S1I::min_from(i))
                            .map(|(_, cow)| cow.into_owned())
                    },
                ))
            }),
            last: options.last.then(|| {
                Box::new(LazyVecFrom2::init(
                    name,
                    version + VERSION + Version::ZERO,
                    source_extra.last.as_ref().map_or_else(
                        || {
                            source
                                .as_ref()
                                .unwrap_or_else(|| {
                                    dbg!(name, I::to_string());
                                    panic!()
                                })
                                .clone()
                        },
                        |v| v.clone(),
                    ),
                    len_source.clone(),
                    |i: I, source, len_source| {
                        if i.to_usize() >= len_source.len() {
                            return None;
                        }
                        source
                            .next_at(S1I::max_from(i, source.len()))
                            .map(|(_, cow)| cow.into_owned())
                    },
                ))
            }),
            min: options.min.then(|| {
                Box::new(LazyVecFrom2::init(
                    &maybe_suffix("min"),
                    version + VERSION + Version::ZERO,
                    source_extra
                        .min
                        .as_ref()
                        .map_or_else(|| source.as_ref().unwrap().clone(), |v| v.clone()),
                    len_source.clone(),
                    |i: I, source, len_source| {
                        if i.to_usize() >= len_source.len() {
                            return None;
                        }
                        S1I::inclusive_range_from(i, source.len())
                            .flat_map(|i| source.next_at(i).map(|(_, cow)| cow.into_owned()))
                            .min()
                    },
                ))
            }),
            max: options.max.then(|| {
                Box::new(LazyVecFrom2::init(
                    &maybe_suffix("max"),
                    version + VERSION + Version::ZERO,
                    source_extra
                        .max
                        .as_ref()
                        .map_or_else(|| source.as_ref().unwrap().clone(), |v| v.clone()),
                    len_source.clone(),
                    |i: I, source, len_source| {
                        if i.to_usize() >= len_source.len() {
                            return None;
                        }
                        S1I::inclusive_range_from(i, source.len())
                            .flat_map(|i| source.next_at(i).map(|(_, cow)| cow.into_owned()))
                            .max()
                    },
                ))
            }),
            average: options.average.then(|| {
                Box::new(LazyVecFrom2::init(
                    &maybe_suffix("avg"),
                    version + VERSION + Version::ZERO,
                    source_extra
                        .average
                        .as_ref()
                        .map_or_else(|| source.as_ref().unwrap().clone(), |v| v.clone()),
                    len_source.clone(),
                    |i: I, source, len_source| {
                        if i.to_usize() >= len_source.len() {
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
                ))
            }),
            sum: options.sum.then(|| {
                Box::new(LazyVecFrom2::init(
                    &(if !options.last && !options.average && !options.min && !options.max {
                        name.to_string()
                    } else {
                        maybe_suffix("sum")
                    }),
                    version + VERSION + Version::ZERO,
                    source_extra
                        .sum
                        .as_ref()
                        .map_or_else(|| source.as_ref().unwrap().clone(), |v| v.clone()),
                    len_source.clone(),
                    |i: I, source, len_source| {
                        if i.to_usize() >= len_source.len() {
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
                ))
            }),
            cumulative: options.cumulative.then(|| {
                Box::new(LazyVecFrom2::init(
                    &suffix("cumulative"),
                    version + VERSION + Version::ZERO,
                    source_extra.cumulative.as_ref().unwrap().boxed_clone(),
                    len_source.clone(),
                    |i: I, source, len_source| {
                        if i.to_usize() >= len_source.len() {
                            return None;
                        }
                        source
                            .next_at(S1I::max_from(i, source.len()))
                            .map(|(_, cow)| cow.into_owned())
                    },
                ))
            }),
        }
    }

    pub fn starting_index(&self, max_from: I) -> I {
        max_from.min(I::from(
            self.iter_any_collectable().map(|v| v.len()).min().unwrap(),
        ))
    }

    pub fn unwrap_first(&self) -> &LazyVecFrom2<I, T, S1I, T, I, S2T> {
        self.first.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_average(&self) -> &LazyVecFrom2<I, T, S1I, T, I, S2T> {
        self.average.as_ref().unwrap()
    }
    pub fn unwrap_sum(&self) -> &LazyVecFrom2<I, T, S1I, T, I, S2T> {
        self.sum.as_ref().unwrap()
    }
    pub fn unwrap_max(&self) -> &LazyVecFrom2<I, T, S1I, T, I, S2T> {
        self.max.as_ref().unwrap()
    }
    pub fn unwrap_min(&self) -> &LazyVecFrom2<I, T, S1I, T, I, S2T> {
        self.min.as_ref().unwrap()
    }
    pub fn unwrap_last(&self) -> &LazyVecFrom2<I, T, S1I, T, I, S2T> {
        self.last.as_ref().unwrap()
    }
    #[allow(unused)]
    pub fn unwrap_cumulative(&self) -> &LazyVecFrom2<I, T, S1I, T, I, S2T> {
        self.cumulative.as_ref().unwrap()
    }
}

#[derive(Default, Clone, Copy)]
pub struct LazyVecBuilderOptions {
    average: bool,
    sum: bool,
    max: bool,
    min: bool,
    first: bool,
    last: bool,
    cumulative: bool,
}

impl From<VecBuilderOptions> for LazyVecBuilderOptions {
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

impl LazyVecBuilderOptions {
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
