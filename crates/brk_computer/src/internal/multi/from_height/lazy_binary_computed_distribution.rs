//! LazyBinaryComputedFromHeightDistribution - lazy binary transform with distribution stats.
//!
//! Height-level values are lazy: `transform(source1[h], source2[h])`.
//! Uses Distribution aggregation (no sum/cumulative) - appropriate for ratios.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, Database, Exit, IterableBoxedVec, IterableCloneableVec, LazyVecFrom2};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedHeightDerivedDistribution, ComputedVecValue, NumericValue},
};

const VERSION: Version = Version::ZERO;

/// Lazy binary transform at height with distribution stats (no sum/cumulative).
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryComputedFromHeightDistribution<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[traversable(rename = "base")]
    pub height: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    #[deref]
    #[deref_mut]
    pub rest: ComputedHeightDerivedDistribution<T>,
}

impl<T, S1T, S2T> LazyBinaryComputedFromHeightDistribution<T, S1T, S2T>
where
    T: NumericValue + JsonSchema,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn forced_import<F: BinaryTransform<S1T, S2T, T>>(
        db: &Database,
        name: &str,
        version: Version,
        source1: IterableBoxedVec<Height, S1T>,
        source2: IterableBoxedVec<Height, S2T>,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height = LazyVecFrom2::transformed::<F>(name, v, source1, source2);

        let rest = ComputedHeightDerivedDistribution::forced_import(
            db,
            name,
            height.boxed_clone(),
            v,
            indexes,
        )?;

        Ok(Self { height, rest })
    }

    pub fn derive_from(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.rest
            .derive_from(indexes, starting_indexes, &self.height, exit)
    }
}
