//! Lazy binary price wrapper with both USD and sats representations.
//!
//! Height-level value is lazy binary: transform(source1[h], source2[h]).
//! Sats are derived lazily from the dollars output.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, SatsFract, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, Database, IterableBoxedVec, IterableCloneableVec};

use super::LazyBinaryComputedFromHeightLast;
use crate::{
    indexes,
    internal::{ComputedVecValue, DollarsToSatsFract, LazyFromHeightLast, NumericValue},
};

/// Lazy binary price metric with both USD and sats representations.
///
/// Dollars: lazy binary transform at height, stored at dateindex.
/// Sats: lazy unary transform of dollars (fully lazy).
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryPriceFromHeight<S1T = Dollars, S2T = Dollars>
where
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dollars: LazyBinaryComputedFromHeightLast<Dollars, S1T, S2T>,
    pub sats: LazyFromHeightLast<SatsFract, Dollars>,
}

impl<S1T, S2T> LazyBinaryPriceFromHeight<S1T, S2T>
where
    S1T: NumericValue + JsonSchema,
    S2T: NumericValue + JsonSchema,
{
    pub fn forced_import<F: BinaryTransform<S1T, S2T, Dollars>>(
        db: &Database,
        name: &str,
        version: Version,
        source1: IterableBoxedVec<Height, S1T>,
        source2: IterableBoxedVec<Height, S2T>,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let dollars = LazyBinaryComputedFromHeightLast::forced_import::<F>(
            db, name, version, source1, source2, indexes,
        )?;

        let sats = LazyFromHeightLast::from_lazy_binary_computed::<DollarsToSatsFract, S1T, S2T>(
            &format!("{name}_sats"),
            version,
            dollars.height.boxed_clone(),
            &dollars,
        );

        Ok(Self { dollars, sats })
    }
}
