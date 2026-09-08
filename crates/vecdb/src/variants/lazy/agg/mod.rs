use std::{marker::PhantomData, sync::Arc};

pub mod any_vec;
pub mod clone;
pub mod fold;
pub mod readable;
pub mod sparse;
pub mod typed;

pub use fold::*;
use sparse::Sparse;

use crate::{ReadableBoxedVec, VecIndex, VecValue, Version};

/// Lazy aggregation vector that maps coarser output indices to ranges in a finer source.
///
/// Values are computed on-the-fly using cursor-based sequential access.
/// The mapping is pulled via a caller-provided closure on each read.
pub struct LazyAggVec<I, O, S1I, S2T, S1T = O, Strat = Sparse>
where
    I: VecIndex,
    O: VecValue,
    S1I: VecIndex,
    S2T: VecValue,
    S1T: VecValue,
{
    name: Arc<str>,
    version: Version,
    mapping_version: Version,
    source: ReadableBoxedVec<S1I, S1T>,
    #[allow(clippy::type_complexity)]
    mapping: Arc<dyn Fn() -> Arc<Vec<S2T>> + Send + Sync>,
    #[allow(clippy::type_complexity)]
    _phantom: PhantomData<fn() -> (I, O, Strat)>,
}

impl<I, O, S1I, S2T, S1T, Strat> LazyAggVec<I, O, S1I, S2T, S1T, Strat>
where
    I: VecIndex,
    O: VecValue,
    S1I: VecIndex,
    S2T: VecValue,
    S1T: VecValue,
{
    /// The shared source read by this aggregation view.
    pub fn source(&self) -> &ReadableBoxedVec<S1I, S1T> {
        &self.source
    }

    pub fn new(
        name: &str,
        version: Version,
        mapping_version: Version,
        source: ReadableBoxedVec<S1I, S1T>,
        mapping: impl Fn() -> Arc<Vec<S2T>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: Arc::from(name),
            version,
            mapping_version,
            source,
            mapping: Arc::new(mapping),
            _phantom: PhantomData,
        }
    }
}
