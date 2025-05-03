use std::path::Path;

use brk_exit::Exit;
use clap_derive::ValueEnum;
use serde::{Deserialize, Serialize};

mod _type;
mod eager;
mod lazy1;
mod lazy2;
mod lazy3;

pub use _type::*;
use brk_core::StoredPhantom;
use brk_vec::{Compressed, GenericVec, Result, StoredIndex, StoredType, StoredVec, Version};
pub use eager::*;
pub use lazy1::*;
pub use lazy2::*;
pub use lazy3::*;

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
enum Dependencies<T, S1I, S1T, S2I, S2T, S3I, S3T> {
    From1(StoredVec<S1I, S1T>, ComputeFrom1<T, S1I, S1T>),
    From2(
        (StoredVec<S1I, S1T>, StoredVec<S2I, S2T>),
        ComputeFrom2<T, S1I, S1T, S2I, S2T>,
    ),
    From3(
        (
            StoredVec<S1I, S1T>,
            StoredVec<S2I, S2T>,
            StoredVec<S3I, S3T>,
        ),
        ComputeFrom3<T, S1I, S1T, S2I, S2T, S3I, S3T>,
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
        deps: Dependencies<T, S1I, S1T, S2I, S2T, S3I, S3T>,
    },
    LazyFrom1(LazyVecFrom1<I, T, S1I, S1T>),
    LazyFrom2(LazyVecFrom2<I, T, S1I, S1T, S2I, S2T>),
    LazyFrom3(LazyVecFrom3<I, T, S1I, S1T, S2I, S2T, S3I, S3T>),
    // Lazy4
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
        version: Version,
        compressed: Compressed,
        source: StoredVec<S1I, S1T>,
        compute: ComputeFrom1<T, S1I, S1T>,
    ) -> brk_vec::Result<Self> {
        Ok(match mode {
            Computation::Eager => Self::Eager {
                vec: EagerVec::forced_import(path, version, compressed)?,
                deps: Dependencies::From1(source, compute),
            },
            Computation::Lazy => Self::LazyFrom1(LazyVecFrom1::init(version, source, compute)),
        })
    }

    pub fn forced_import_or_init_from_2(
        mode: Computation,
        path: &Path,
        version: Version,
        compressed: Compressed,
        source1: StoredVec<S1I, S1T>,
        source2: StoredVec<S2I, S2T>,
        compute: ComputeFrom2<T, S1I, S1T, S2I, S2T>,
    ) -> brk_vec::Result<Self> {
        Ok(match mode {
            Computation::Eager => Self::Eager {
                vec: EagerVec::forced_import(path, version, compressed)?,
                deps: Dependencies::From2((source1, source2), compute),
            },
            Computation::Lazy => {
                Self::LazyFrom2(LazyVecFrom2::init(version, source1, source2, compute))
            }
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn forced_import_or_init_from_3(
        mode: Computation,
        path: &Path,
        version: Version,
        compressed: Compressed,
        source1: StoredVec<S1I, S1T>,
        source2: StoredVec<S2I, S2T>,
        source3: StoredVec<S3I, S3T>,
        compute: ComputeFrom3<T, S1I, S1T, S2I, S2T, S3I, S3T>,
    ) -> brk_vec::Result<Self> {
        Ok(match mode {
            Computation::Eager => Self::Eager {
                vec: EagerVec::forced_import(path, version, compressed)?,
                deps: Dependencies::From3((source1, source2, source3), compute),
            },
            Computation::Lazy => Self::LazyFrom3(LazyVecFrom3::init(
                version, source1, source2, source3, compute,
            )),
        })
    }

    pub fn compute_if_necessary(&mut self, max_from: I, exit: &Exit) -> Result<()> {
        let (vec, dependencies) = if let ComputedVec::Eager {
            vec,
            deps: dependencies,
        } = self
        {
            (vec, dependencies)
        } else {
            return Ok(());
        };

        match dependencies {
            Dependencies::From1(source, compute) => {
                let version = source.version();
                let mut iter = source.iter();
                let t = |i: I| {
                    compute(i.unwrap_to_usize(), &mut iter)
                        .map(|v| (i, v))
                        .unwrap()
                };
                vec.compute_to(max_from, 1, version, t, exit)
            }
            Dependencies::From2((source1, source2), compute) => {
                let version = source1.version() + source2.version();
                let mut iter1 = source1.iter();
                let mut iter2 = source2.iter();
                let t = |i: I| {
                    compute(i.unwrap_to_usize(), &mut iter1, &mut iter2)
                        .map(|v| (i, v))
                        .unwrap()
                };
                vec.compute_to(max_from, 1, version, t, exit)
            }
            Dependencies::From3((source1, source2, source3), compute) => {
                let version = source1.version() + source2.version() + source3.version();
                let mut iter1 = source1.iter();
                let mut iter2 = source2.iter();
                let mut iter3 = source3.iter();
                let t = |i: I| {
                    compute(i.unwrap_to_usize(), &mut iter1, &mut iter2, &mut iter3)
                        .map(|v| (i, v))
                        .unwrap()
                };
                vec.compute_to(max_from, 1, version, t, exit)
            }
        }
    }
}
