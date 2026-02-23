//! LazyBinaryComputedFromHeightFull - block full with lazy binary transform at height level.
//!
//! Height-level values are lazy: `transform(source1[h], source2[h])`.
//! Cumulative, day1 stats, and difficultyepoch are stored since they
//! require aggregation across heights.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, Database, Exit, ReadableBoxedVec, ReadableCloneableVec, LazyVecFrom2, Rw, StorageMode};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedHeightDerivedFull, ComputedVecValue, NumericValue},
};

const VERSION: Version = Version::ZERO;

/// Block full aggregation with lazy binary transform at height + computed derived indexes.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryComputedFromHeightFull<T, S1T = T, S2T = T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[traversable(rename = "base")]
    pub height: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    #[deref]
    #[deref_mut]
    pub rest: Box<ComputedHeightDerivedFull<T, M>>,
}

impl<T, S1T, S2T> LazyBinaryComputedFromHeightFull<T, S1T, S2T>
where
    T: NumericValue + JsonSchema,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn forced_import<F: BinaryTransform<S1T, S2T, T>>(
        db: &Database,
        name: &str,
        version: Version,
        source1: ReadableBoxedVec<Height, S1T>,
        source2: ReadableBoxedVec<Height, S2T>,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height = LazyVecFrom2::transformed::<F>(name, v, source1, source2);

        let rest =
            ComputedHeightDerivedFull::forced_import(db, name, height.read_only_boxed_clone(), v, indexes)?;

        Ok(Self { height, rest: Box::new(rest) })
    }

    pub(crate) fn compute_cumulative(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.rest
            .compute_cumulative(starting_indexes, &self.height, exit)
    }
}
