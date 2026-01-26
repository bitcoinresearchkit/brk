//! Lazy binary price wrapper with both USD and sats representations.
//!
//! For binary operations (e.g., price × ratio) that produce price values.
//! Both dollars and sats are computed lazily from the same sources.

use brk_traversable::Traversable;
use brk_types::{Dollars, SatsFract, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, IterableCloneableVec, LazyVecFrom1};

use super::{ComputedFromDateLast, LazyBinaryFromDateLast};
use crate::internal::{ComputedFromHeightLast, ComputedVecValue, DollarsToSatsFract, LazyTransformLast, NumericValue};

/// Lazy binary price with both USD and sats representations.
///
/// Wraps a binary operation that produces Dollars and lazily converts to sats.
/// Derefs to the dollars metric.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryPrice<S1T, S2T>
where
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dollars: LazyBinaryFromDateLast<Dollars, S1T, S2T>,
    pub sats: LazyUnaryFromBinaryLast<SatsFract, Dollars, S1T, S2T>,
}

/// Lazy unary transform chain on a LazyBinaryFromDateLast output.
#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct LazyUnaryFromBinaryLast<T, ST, S1T, S2T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
    ST: ComputedVecValue,
    S1T: ComputedVecValue,
    S2T: ComputedVecValue,
{
    pub dateindex: LazyVecFrom1<brk_types::DateIndex, T, brk_types::DateIndex, ST>,
    pub weekindex: LazyTransformLast<brk_types::WeekIndex, T, ST>,
    pub monthindex: LazyTransformLast<brk_types::MonthIndex, T, ST>,
    pub quarterindex: LazyTransformLast<brk_types::QuarterIndex, T, ST>,
    pub semesterindex: LazyTransformLast<brk_types::SemesterIndex, T, ST>,
    pub yearindex: LazyTransformLast<brk_types::YearIndex, T, ST>,
    pub decadeindex: LazyTransformLast<brk_types::DecadeIndex, T, ST>,
    _phantom: std::marker::PhantomData<(S1T, S2T)>,
}

impl<S1T, S2T> LazyBinaryPrice<S1T, S2T>
where
    S1T: ComputedVecValue + JsonSchema + 'static,
    S2T: ComputedVecValue + JsonSchema + 'static,
{
    /// Create from height-based price and dateindex-based ratio sources.
    pub fn from_height_and_dateindex_last<F: BinaryTransform<S1T, S2T, Dollars>>(
        name: &str,
        version: Version,
        source1: &ComputedFromHeightLast<S1T>,
        source2: &ComputedFromDateLast<S2T>,
    ) -> Self
    where
        S1T: NumericValue,
    {
        let dollars = LazyBinaryFromDateLast::from_height_and_dateindex_last::<F>(
            name, version, source1, source2,
        );
        Self::from_dollars(name, version, dollars)
    }

    /// Create from two computed dateindex sources.
    pub fn from_computed_both_last<F: BinaryTransform<S1T, S2T, Dollars>>(
        name: &str,
        version: Version,
        source1: &ComputedFromDateLast<S1T>,
        source2: &ComputedFromDateLast<S2T>,
    ) -> Self {
        let dollars = LazyBinaryFromDateLast::from_computed_both_last::<F>(
            name, version, source1, source2,
        );
        Self::from_dollars(name, version, dollars)
    }

    /// Create sats version from dollars.
    fn from_dollars(
        name: &str,
        version: Version,
        dollars: LazyBinaryFromDateLast<Dollars, S1T, S2T>,
    ) -> Self {
        let sats_name = format!("{name}_sats");
        let sats = LazyUnaryFromBinaryLast {
            dateindex: LazyVecFrom1::transformed::<DollarsToSatsFract>(
                &sats_name,
                version,
                dollars.dateindex.boxed_clone(),
            ),
            weekindex: LazyTransformLast::from_boxed::<DollarsToSatsFract>(
                &sats_name,
                version,
                dollars.weekindex.boxed_clone(),
            ),
            monthindex: LazyTransformLast::from_boxed::<DollarsToSatsFract>(
                &sats_name,
                version,
                dollars.monthindex.boxed_clone(),
            ),
            quarterindex: LazyTransformLast::from_boxed::<DollarsToSatsFract>(
                &sats_name,
                version,
                dollars.quarterindex.boxed_clone(),
            ),
            semesterindex: LazyTransformLast::from_boxed::<DollarsToSatsFract>(
                &sats_name,
                version,
                dollars.semesterindex.boxed_clone(),
            ),
            yearindex: LazyTransformLast::from_boxed::<DollarsToSatsFract>(
                &sats_name,
                version,
                dollars.yearindex.boxed_clone(),
            ),
            decadeindex: LazyTransformLast::from_boxed::<DollarsToSatsFract>(
                &sats_name,
                version,
                dollars.decadeindex.boxed_clone(),
            ),
            _phantom: std::marker::PhantomData,
        };

        Self { dollars, sats }
    }
}
