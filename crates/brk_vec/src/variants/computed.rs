use std::{fs, path::Path, time::Duration};

use brk_exit::Exit;
use clap_derive::ValueEnum;
use serde::{Deserialize, Serialize};

use brk_core::{Result, StoredPhantom, Value, Version};

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BaseVecIterator, BoxedAnyIterableVec,
    BoxedVecIterator, CollectableVec, Format, StoredIndex, StoredType,
};

use super::{
    ComputeFrom1, ComputeFrom2, ComputeFrom3, EagerVec, LazyVecFrom1, LazyVecFrom1Iterator,
    LazyVecFrom2, LazyVecFrom2Iterator, LazyVecFrom3, LazyVecFrom3Iterator, StoredVecIterator,
};

#[derive(
    Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Serialize, Deserialize, ValueEnum,
)]
pub enum Computation {
    Eager,
    #[default]
    Lazy,
}

impl Computation {
    pub fn eager(&self) -> bool {
        *self == Self::Eager
    }

    pub fn lazy(&self) -> bool {
        *self == Self::Lazy
    }
}

#[derive(Clone)]
pub enum Dependencies<I, T, S1I, S1T, S2I, S2T, S3I, S3T> {
    From1(BoxedAnyIterableVec<S1I, S1T>, ComputeFrom1<I, T, S1I, S1T>),
    From2(
        (BoxedAnyIterableVec<S1I, S1T>, BoxedAnyIterableVec<S2I, S2T>),
        ComputeFrom2<I, T, S1I, S1T, S2I, S2T>,
    ),
    From3(
        (
            BoxedAnyIterableVec<S1I, S1T>,
            BoxedAnyIterableVec<S2I, S2T>,
            BoxedAnyIterableVec<S3I, S3T>,
        ),
        ComputeFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>,
    ),
}

pub type ComputedVecFrom1<I, T, S1I, S1T> =
    ComputedVec<I, T, S1I, S1T, StoredPhantom, StoredPhantom, StoredPhantom, StoredPhantom>;
pub type ComputedVecFrom2<I, T, S1I, S1T, S2I, S2T> =
    ComputedVec<I, T, S1I, S1T, S2I, S2T, StoredPhantom, StoredPhantom>;
pub type ComputedVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T> =
    ComputedVec<I, T, S1I, S1T, S2I, S2T, S3I, S3T>;

#[derive(Clone)]
pub enum ComputedVec<I, T, S1I, S1T, S2I, S2T, S3I, S3T> {
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
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
{
    pub fn forced_import_or_init_from_1(
        mode: Computation,
        path: &Path,
        value_name: &str,
        version: Version,
        format: Format,
        source: BoxedAnyIterableVec<S1I, S1T>,
        compute: ComputeFrom1<I, T, S1I, S1T>,
    ) -> Result<Self> {
        Ok(match mode {
            Computation::Eager => Self::Eager {
                vec: EagerVec::forced_import(path, value_name, version, format)?,
                deps: Dependencies::From1(source, compute),
            },
            Computation::Lazy => {
                let _ = fs::remove_dir_all(I::path(path, value_name));
                Self::LazyFrom1(LazyVecFrom1::init(value_name, version, source, compute))
            }
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn forced_import_or_init_from_2(
        mode: Computation,
        path: &Path,
        value_name: &str,
        version: Version,
        format: Format,
        source1: BoxedAnyIterableVec<S1I, S1T>,
        source2: BoxedAnyIterableVec<S2I, S2T>,
        compute: ComputeFrom2<I, T, S1I, S1T, S2I, S2T>,
    ) -> Result<Self> {
        Ok(match mode {
            Computation::Eager => Self::Eager {
                vec: EagerVec::forced_import(path, value_name, version, format)?,
                deps: Dependencies::From2((source1, source2), compute),
            },
            Computation::Lazy => {
                let _ = fs::remove_dir_all(I::path(path, value_name));
                Self::LazyFrom2(LazyVecFrom2::init(
                    value_name, version, source1, source2, compute,
                ))
            }
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn forced_import_or_init_from_3(
        mode: Computation,
        path: &Path,
        value_name: &str,
        version: Version,
        format: Format,
        source1: BoxedAnyIterableVec<S1I, S1T>,
        source2: BoxedAnyIterableVec<S2I, S2T>,
        source3: BoxedAnyIterableVec<S3I, S3T>,
        compute: ComputeFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>,
    ) -> Result<Self> {
        Ok(match mode {
            Computation::Eager => Self::Eager {
                vec: EagerVec::forced_import(path, value_name, version, format)?,
                deps: Dependencies::From3((source1, source2, source3), compute),
            },
            Computation::Lazy => {
                let _ = fs::remove_dir_all(I::path(path, value_name));
                Self::LazyFrom3(LazyVecFrom3::init(
                    value_name, version, source1, source2, source3, compute,
                ))
            }
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
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
{
    fn version(&self) -> Version {
        match self {
            ComputedVec::Eager { vec, .. } => vec.version(),
            ComputedVec::LazyFrom1(v) => v.version(),
            ComputedVec::LazyFrom2(v) => v.version(),
            ComputedVec::LazyFrom3(v) => v.version(),
        }
    }

    fn name(&self) -> String {
        match self {
            ComputedVec::Eager { vec, .. } => vec.name(),
            ComputedVec::LazyFrom1(v) => v.name(),
            ComputedVec::LazyFrom2(v) => v.name(),
            ComputedVec::LazyFrom3(v) => v.name(),
        }
    }

    fn index_type_to_string(&self) -> String {
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

    fn modified_time(&self) -> Result<Duration> {
        match self {
            ComputedVec::Eager { vec, .. } => vec.modified_time(),
            ComputedVec::LazyFrom1(v) => v.modified_time(),
            ComputedVec::LazyFrom2(v) => v.modified_time(),
            ComputedVec::LazyFrom3(v) => v.modified_time(),
        }
    }
}

pub enum ComputedVecIterator<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T> {
    Eager(StoredVecIterator<'a, I, T>),
    LazyFrom1(LazyVecFrom1Iterator<'a, I, T, S1I, S1T>),
    LazyFrom2(LazyVecFrom2Iterator<'a, I, T, S1I, S1T, S2I, S2T>),
    LazyFrom3(LazyVecFrom3Iterator<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T>),
}

impl<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T> Iterator
    for ComputedVecIterator<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
{
    type Item = (I, Value<'a, T>);
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
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
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
    fn path(&self) -> &Path {
        match self {
            Self::Eager(i) => i.path(),
            Self::LazyFrom1(i) => i.path(),
            Self::LazyFrom2(i) => i.path(),
            Self::LazyFrom3(i) => i.path(),
        }
    }
}

impl<'a, I, T, S1I, S1T, S2I, S2T, S3I, S3T> IntoIterator
    for &'a ComputedVec<I, T, S1I, S1T, S2I, S2T, S3I, S3T>
where
    I: StoredIndex,
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
{
    type Item = (I, Value<'a, T>);
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
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
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
    T: StoredType,
    S1I: StoredIndex,
    S1T: StoredType,
    S2I: StoredIndex,
    S2T: StoredType,
    S3I: StoredIndex,
    S3T: StoredType,
{
    fn collect_range_serde_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
