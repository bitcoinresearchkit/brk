//! LazyComputedFromHeightFull - block full with lazy height transform.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableCloneableVec, LazyVecFrom1, UnaryTransform, Rw, StorageMode};

use crate::{
    ComputeIndexes,
    indexes,
    internal::{ComputedVecValue, ComputedHeightDerivedFull, NumericValue},
};

const VERSION: Version = Version::ZERO;

/// Block full aggregation with lazy height transform + computed derived indexes.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyComputedFromHeightFull<T, S = T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S: ComputedVecValue,
{
    #[traversable(rename = "base")]
    pub height: LazyVecFrom1<Height, T, Height, S>,
    #[deref]
    #[deref_mut]
    pub rest: Box<ComputedHeightDerivedFull<T, M>>,
}

impl<T, S> LazyComputedFromHeightFull<T, S>
where
    T: NumericValue + JsonSchema,
    S: ComputedVecValue + JsonSchema,
{
    pub(crate) fn forced_import<F: UnaryTransform<S, T>>(
        db: &Database,
        name: &str,
        version: Version,
        source: &impl ReadableCloneableVec<Height, S>,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height = LazyVecFrom1::transformed::<F>(name, v, source.read_only_boxed_clone());

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
