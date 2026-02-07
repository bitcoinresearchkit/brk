//! Fully lazy binary price wrapper with both USD and sats representations.
//!
//! All levels (height, dateindex, date periods, difficultyepoch) are lazy.
//! Derives dateindex from the two source dateindexes rather than storing it.

use brk_traversable::Traversable;
use brk_types::{CentsUnsigned, Dollars, SatsFract, Version};
use derive_more::{Deref, DerefMut};
use vecdb::BinaryTransform;

use crate::internal::{
    DollarsToSatsFract, LazyBinaryFromHeightLast, LazyFromHeightLast, LazyPriceFromCents,
    PriceFromHeight,
};

/// Fully lazy binary price metric with both USD and sats representations.
///
/// Dollars: lazy binary transform at all levels (height, dateindex, date periods, difficultyepoch).
/// Sats: lazy unary transform of dollars.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct LazyBinaryPriceFromHeight {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dollars: LazyBinaryFromHeightLast<Dollars, Dollars, Dollars>,
    pub sats: LazyFromHeightLast<SatsFract, Dollars>,
}

impl LazyBinaryPriceFromHeight {
    /// Create from a PriceFromHeight (source1) and a LazyPriceFromCents (source2).
    pub fn from_price_and_lazy_price<F: BinaryTransform<Dollars, Dollars, Dollars>>(
        name: &str,
        version: Version,
        source1: &PriceFromHeight,
        source2: &LazyPriceFromCents,
    ) -> Self {
        let dollars = LazyBinaryFromHeightLast::from_block_last_and_lazy_block_last::<
            F,
            CentsUnsigned,
        >(name, version, &source1.dollars, &source2.dollars);

        let sats = LazyFromHeightLast::from_binary::<DollarsToSatsFract, _, _>(
            &format!("{name}_sats"),
            version,
            &dollars,
        );

        Self { dollars, sats }
    }

    /// Create from a LazyPriceFromCents (source1) and a PriceFromHeight (source2).
    pub fn from_lazy_price_and_price<F: BinaryTransform<Dollars, Dollars, Dollars>>(
        name: &str,
        version: Version,
        source1: &LazyPriceFromCents,
        source2: &PriceFromHeight,
    ) -> Self {
        let dollars = LazyBinaryFromHeightLast::from_lazy_block_last_and_block_last::<
            F,
            CentsUnsigned,
        >(name, version, &source1.dollars, &source2.dollars);

        let sats = LazyFromHeightLast::from_binary::<DollarsToSatsFract, _, _>(
            &format!("{name}_sats"),
            version,
            &dollars,
        );

        Self { dollars, sats }
    }
}
