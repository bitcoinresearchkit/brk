use std::{borrow::Cow, sync::Arc};

use crate::{
    AnyBoxedIterableVec, AnyCollectableVec, AnyIterableVec, AnyVec, BaseVecIterator,
    BoxedVecIterator, CollectableVec, Exit, File, Format, Result, StoredCompressed, StoredIndex,
    StoredRaw, Version,
};

use super::{
    ComputeFrom1, ComputeFrom2, ComputeFrom3, EagerVec, LazyVecFrom1, LazyVecFrom1Iterator,
    LazyVecFrom2, LazyVecFrom2Iterator, LazyVecFrom3, LazyVecFrom3Iterator, StoredVecIterator,
};

mod computation;

pub use computation::*;

#[derive(Clone)]
pub enum Dependencies<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    S1T: Clone,
    S2T: Clone,
    S3T: Clone,
{
    From1(AnyBoxedIterableVec<S1I, S1T>, ComputeFrom1<I, T, S1I, S1T>),
    From2(
        (AnyBoxedIterableVec<S1I, S1T>, AnyBoxedIterableVec<S2I, S2T>),
        ComputeFrom2<I, T, S1I, S1T, S2I, S2T>,
    ),
    From3(
        (
            AnyBoxedIterableVec<S1I, S1T>,
            AnyBoxedIterableVec<S2I, S2T>,
            AnyBoxedIterableVec<S3I, S3T>,
        ),
        ComputeFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>,
    ),
}

pub type ComputedVecFrom1<I, T, S1I, S1T> = ComputedVec<I, T, S1I, S1T, usize, (), usize, ()>;
pub type ComputedVecFrom2<I, T, S1I, S1T, S2I, S2T> =
    ComputedVec<I, T, S1I, S1T, S2I, S2T, usize, ()>;
pub type ComputedVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T> =
    ComputedVec<I, T, S1I, S1T, S2I, S2T, S3I, S3T>;

#[derive(Clone)]
pub enum ComputedVec<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    S1T: Clone,
    S2T: Clone,
    S3T: Clone,
{
    Eager {
        vec: EagerVec<I, T>,
        deps: Dependencies<I, T, S1I, S1T, S2I, S2T, S3I, S3T>,
    },
    LazyFrom1(LazyVecFrom1<I, T, S1I, S1T>),
    LazyFrom2(LazyVecFrom2<I, T, S1I, S1T, S2I, S2T>),
    LazyFrom3(LazyVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>),
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> ComputedVec<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredCompressed,
    S1I: StoredIndex,
    S1T: StoredRaw,
    S2I: StoredIndex,
    S2T: StoredRaw,
    S3I: StoredIndex,
    S3T: StoredRaw,
{
    pub fn forced_import_or_init_from_1(
        computation: Computation,
        file: &Arc<File>,
        name: &str,
        version: Version,
        format: Format,
        source: AnyBoxedIterableVec<S1I, S1T>,
        compute: ComputeFrom1<I, T, S1I, S1T>,
    ) -> Result<Self> {
        Ok(match computation {
            Computation::Eager => Self::Eager {
                vec: EagerVec::forced_import(file, name, version, format)?,
                deps: Dependencies::From1(source, compute),
            },
            Computation::Lazy => {
                Self::LazyFrom1(LazyVecFrom1::init(name, version, source, compute))
            }
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn forced_import_or_init_from_2(
        computation: Computation,
        file: &Arc<File>,
        name: &str,
        version: Version,
        format: Format,
        source1: AnyBoxedIterableVec<S1I, S1T>,
        source2: AnyBoxedIterableVec<S2I, S2T>,
        compute: ComputeFrom2<I, T, S1I, S1T, S2I, S2T>,
    ) -> Result<Self> {
        Ok(match computation {
            Computation::Eager => Self::Eager {
                vec: EagerVec::forced_import(file, name, version, format)?,
                deps: Dependencies::From2((source1, source2), compute),
            },
            Computation::Lazy => {
                Self::LazyFrom2(LazyVecFrom2::init(name, version, source1, source2, compute))
            }
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn forced_import_or_init_from_3(
        computation: Computation,
        file: &Arc<File>,
        name: &str,
        version: Version,
        format: Format,
        source1: AnyBoxedIterableVec<S1I, S1T>,
        source2: AnyBoxedIterableVec<S2I, S2T>,
        source3: AnyBoxedIterableVec<S3I, S3T>,
        compute: ComputeFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>,
    ) -> Result<Self> {
        Ok(match computation {
            Computation::Eager => Self::Eager {
                vec: EagerVec::forced_import(file, name, version, format)?,
                deps: Dependencies::From3((source1, source2, source3), compute),
            },
            Computation::Lazy => Self::LazyFrom3(LazyVecFrom3::init(
                name, version, source1, source2, source3, compute,
            )),
        })
    }

    pub fn compute_if_necessary<T2>(
        &mut self,
        max_from: I,
        len_source: &impl AnyIterableVec<I, T2>,
        exit: &Exit,
    ) -> Result<()> {
        let (vec, dependencies) = if let ComputedVec::Eager {
            vec,
            deps: dependencies,
        } = self
        {
            (vec, dependencies)
        } else {
            return Ok(());
        };

        let len = len_source.len();

        match dependencies {
            Dependencies::From1(source, compute) => {
                let version = source.version();
                let mut iter = source.iter();
                let t = |i: I| compute(i, &mut *iter).map(|v| (i, v)).unwrap();
                vec.compute_to(max_from, len, version, t, exit)
            }
            Dependencies::From2((source1, source2), compute) => {
                let version = source1.version() + source2.version();
                let mut iter1 = source1.iter();
                let mut iter2 = source2.iter();
                let t = |i: I| {
                    compute(i, &mut *iter1, &mut *iter2)
                        .map(|v| (i, v))
                        .unwrap()
                };
                vec.compute_to(max_from, len, version, t, exit)
            }
            Dependencies::From3((source1, source2, source3), compute) => {
                let version = source1.version() + source2.version() + source3.version();
                let mut iter1 = source1.iter();
                let mut iter2 = source2.iter();
                let mut iter3 = source3.iter();
                let t = |i: I| {
                    compute(i, &mut *iter1, &mut *iter2, &mut *iter3)
                        .map(|v| (i, v))
                        .unwrap()
                };
                vec.compute_to(max_from, len, version, t, exit)
            }
        }
    }
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> AnyVec for ComputedVec<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredCompressed,
    S1I: StoredIndex,
    S1T: StoredRaw,
    S2I: StoredIndex,
    S2T: StoredRaw,
    S3I: StoredIndex,
    S3T: StoredRaw,
{
    fn version(&self) -> Version {
        match self {
            ComputedVec::Eager { vec, .. } => vec.version(),
            ComputedVec::LazyFrom1(v) => v.version(),
            ComputedVec::LazyFrom2(v) => v.version(),
            ComputedVec::LazyFrom3(v) => v.version(),
        }
    }

    fn name(&self) -> &str {
        match self {
            ComputedVec::Eager { vec, .. } => vec.name(),
            ComputedVec::LazyFrom1(v) => v.name(),
            ComputedVec::LazyFrom2(v) => v.name(),
            ComputedVec::LazyFrom3(v) => v.name(),
        }
    }

    fn index_type_to_string(&self) -> &'static str {
        I::to_string()
    }

    fn len(&self) -> usize {
        match self {
            ComputedVec::Eager { vec, .. } => vec.len(),
            ComputedVec::LazyFrom1(v) => v.len(),
            ComputedVec::LazyFrom2(v) => v.len(),
            ComputedVec::LazyFrom3(v) => v.len(),
        }
    }

    #[inline]
    fn value_type_to_size_of(&self) -> usize {
        size_of::<T>()
    }
}

pub enum ComputedVecIterator<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    S1T: Clone,
    S2T: Clone,
    S3T: Clone,
{
    Eager(StoredVecIterator<'a, I, T>),
    LazyFrom1(LazyVecFrom1Iterator<'a, I, T, S1I, S1T>),
    LazyFrom2(LazyVecFrom2Iterator<'a, I, T, S1I, S1T, S2I, S2T>),
    LazyFrom3(LazyVecFrom3Iterator<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T>),
}

impl<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T> Iterator
    for ComputedVecIterator<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredCompressed,
    S1I: StoredIndex,
    S1T: StoredRaw,
    S2I: StoredIndex,
    S2T: StoredRaw,
    S3I: StoredIndex,
    S3T: StoredRaw,
{
    type Item = (I, Cow<'a, T>);
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Eager(i) => i.next(),
            Self::LazyFrom1(i) => i.next(),
            Self::LazyFrom2(i) => i.next(),
            Self::LazyFrom3(i) => i.next(),
        }
    }
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> BaseVecIterator
    for ComputedVecIterator<'_, I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredCompressed,
    S1I: StoredIndex,
    S1T: StoredRaw,
    S2I: StoredIndex,
    S2T: StoredRaw,
    S3I: StoredIndex,
    S3T: StoredRaw,
{
    #[inline]
    fn mut_index(&mut self) -> &mut usize {
        match self {
            Self::Eager(i) => i.mut_index(),
            Self::LazyFrom1(i) => i.mut_index(),
            Self::LazyFrom2(i) => i.mut_index(),
            Self::LazyFrom3(i) => i.mut_index(),
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Eager(i) => i.len(),
            Self::LazyFrom1(i) => i.len(),
            Self::LazyFrom2(i) => i.len(),
            Self::LazyFrom3(i) => i.len(),
        }
    }

    #[inline]
    fn name(&self) -> &str {
        match self {
            Self::Eager(i) => i.name(),
            Self::LazyFrom1(i) => i.name(),
            Self::LazyFrom2(i) => i.name(),
            Self::LazyFrom3(i) => i.name(),
        }
    }
}

impl<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T> IntoIterator
    for &'a ComputedVec<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredCompressed,
    S1I: StoredIndex,
    S1T: StoredRaw,
    S2I: StoredIndex,
    S2T: StoredRaw,
    S3I: StoredIndex,
    S3T: StoredRaw,
{
    type Item = (I, Cow<'a, T>);
    type IntoIter = ComputedVecIterator<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ComputedVec::Eager { vec, .. } => ComputedVecIterator::Eager(vec.into_iter()),
            ComputedVec::LazyFrom1(v) => ComputedVecIterator::LazyFrom1(v.into_iter()),
            ComputedVec::LazyFrom2(v) => ComputedVecIterator::LazyFrom2(v.into_iter()),
            ComputedVec::LazyFrom3(v) => ComputedVecIterator::LazyFrom3(v.into_iter()),
        }
    }
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> AnyIterableVec<I, T>
    for ComputedVec<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredCompressed,
    S1I: StoredIndex,
    S1T: StoredRaw,
    S2I: StoredIndex,
    S2T: StoredRaw,
    S3I: StoredIndex,
    S3T: StoredRaw,
{
    fn boxed_iter<'a>(&'a self) -> BoxedVecIterator<'a, I, T>
    where
        T: 'a,
    {
        Box::new(self.into_iter())
    }
}

impl<I, T, S1I, S1T, S2I, S2T, S3I, S3T> AnyCollectableVec
    for ComputedVec<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredCompressed,
    S1I: StoredIndex,
    S1T: StoredRaw,
    S2I: StoredIndex,
    S2T: StoredRaw,
    S3I: StoredIndex,
    S3T: StoredRaw,
{
    fn collect_range_serde_json(
        &self,
        from: Option<usize>,
        to: Option<usize>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
