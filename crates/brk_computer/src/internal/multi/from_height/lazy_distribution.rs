//! LazyFromHeightDistribution - lazy height + derived distribution (avg/min/max) for indexes.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{ComputeFrom1, Database, Exit, IterableCloneableVec, LazyVecFrom1, UnaryTransform};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedVecValue, ComputedHeightDerivedDistribution, NumericValue},
};

const VERSION: Version = Version::ZERO;

/// Lazy height + derived distribution for indexes.
///
/// Height is a lazy transform from a source.
/// Indexes (dateindex + periods + difficultyepoch) store distribution stats (avg/min/max).
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyFromHeightDistribution<T, S = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S: ComputedVecValue,
{
    #[traversable(rename = "base")]
    pub height: LazyVecFrom1<Height, T, Height, S>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: ComputedHeightDerivedDistribution<T>,
}

impl<T, S> LazyFromHeightDistribution<T, S>
where
    T: NumericValue + JsonSchema,
    S: ComputedVecValue + JsonSchema,
{
    pub fn forced_import<F: UnaryTransform<S, T>>(
        db: &Database,
        name: &str,
        version: Version,
        source: impl IterableCloneableVec<Height, S>,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height = LazyVecFrom1::transformed::<F>(name, v, source.boxed_clone());

        let rest = ComputedHeightDerivedDistribution::forced_import(
            db,
            name,
            height.boxed_clone(),
            v,
            indexes,
        )?;

        Ok(Self { height, rest })
    }

    pub fn forced_import_with_init(
        db: &Database,
        name: &str,
        version: Version,
        source: impl IterableCloneableVec<Height, S>,
        indexes: &indexes::Vecs,
        init_fn: ComputeFrom1<Height, T, Height, S>,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height = LazyVecFrom1::init(name, v, source.boxed_clone(), init_fn);

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
