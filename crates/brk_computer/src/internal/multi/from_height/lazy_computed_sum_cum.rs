//! LazyComputedFromHeightSumCum - block sum+cumulative with lazy height transform.
//!
//! Use this when you need:
//! - Lazy height (binary transform from two sources)
//! - Stored cumulative and day1 aggregates
//! - Lazy coarser period lookups

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableCloneableVec, LazyVecFrom2, Rw, StorageMode};

use crate::{indexes, ComputeIndexes};

use crate::internal::{ComputedVecValue, ComputedHeightDerivedSumCum, NumericValue};

/// Block sum+cumulative with lazy binary height transform + computed derived indexes.
///
/// Height is a lazy binary transform (e.g., mask × source, or price × sats).
/// Cumulative and day1 are stored (computed from lazy height).
/// Coarser periods are lazy lookups.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyComputedFromHeightSumCum<T, S1T = T, S2T = T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[traversable(rename = "sum")]
    pub height: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    #[deref]
    #[deref_mut]
    pub rest: Box<ComputedHeightDerivedSumCum<T, M>>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T, S2T> LazyComputedFromHeightSumCum<T, S1T, S2T>
where
    T: NumericValue + JsonSchema,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        height: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    ) -> Result<Self> {
        let v = version + VERSION;

        let rest = ComputedHeightDerivedSumCum::forced_import(
            db,
            name,
            height.read_only_boxed_clone(),
            v,
            indexes,
        )?;

        Ok(Self { height, rest: Box::new(rest) })
    }

    pub(crate) fn compute_cumulative(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.rest
            .derive_from(starting_indexes, &self.height, exit)
    }
}
