//! LazyComputedFromHeightCumulativeFull - block full with lazy height transform + cumulative + rolling.

use std::ops::SubAssign;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, Exit, LazyVecFrom1, ReadableCloneableVec, Rw, StorageMode, UnaryTransform};

use crate::{
    indexes,
    internal::{ComputedHeightDerivedCumulativeFull, ComputedVecValue, NumericValue, WindowStarts},
};

const VERSION: Version = Version::ZERO;

/// Block full aggregation with lazy height transform + cumulative + rolling windows.
#[derive(Deref, DerefMut, Traversable)]
pub struct LazyComputedFromHeightFull<T, S = T, M: StorageMode = Rw>
where
    T: NumericValue + JsonSchema,
    S: ComputedVecValue,
{
    #[traversable(rename = "base")]
    pub height: LazyVecFrom1<Height, T, Height, S>,
    #[deref]
    #[deref_mut]
    pub rest: Box<ComputedHeightDerivedCumulativeFull<T, M>>,
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

        let rest = ComputedHeightDerivedCumulativeFull::forced_import(db, name, v, indexes)?;

        Ok(Self {
            height,
            rest: Box::new(rest),
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<f64> + Default + SubAssign + Copy + Ord,
        f64: From<T>,
    {
        self.rest.compute(max_from, windows, &self.height, exit)
    }
}
