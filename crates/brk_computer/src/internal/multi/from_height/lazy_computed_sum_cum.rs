//! LazyComputedFromHeightSumCum - block sum+cumulative with lazy height transform.
//!
//! Use this when you need:
//! - Lazy height (binary transform from two sources)
//! - Stored cumulative and dateindex aggregates
//! - Lazy coarser period lookups

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, Exit, IterableCloneableVec, LazyVecFrom2};

use crate::{indexes, ComputeIndexes};

use crate::internal::{ComputedVecValue, ComputedHeightDerivedSumCum, NumericValue};

/// Block sum+cumulative with lazy binary height transform + computed derived indexes.
///
/// Height is a lazy binary transform (e.g., mask × source, or price × sats).
/// Cumulative and dateindex are stored (computed from lazy height).
/// Coarser periods are lazy lookups.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyComputedFromHeightSumCum<T, S1T = T, S2T = T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[traversable(rename = "sum")]
    pub height: LazyVecFrom2<Height, T, Height, S1T, Height, S2T>,
    #[deref]
    #[deref_mut]
    pub rest: ComputedHeightDerivedSumCum<T>,
}

const VERSION: Version = Version::ZERO;

impl<T, S1T, S2T> LazyComputedFromHeightSumCum<T, S1T, S2T>
where
    T: NumericValue + JsonSchema,
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn forced_import(
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
            height.boxed_clone(),
            v,
            indexes,
        )?;

        Ok(Self { height, rest })
    }

    /// Derive aggregates from the lazy height source.
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
