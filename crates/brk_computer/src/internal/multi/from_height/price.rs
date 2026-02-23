//! Generic price wrapper with both USD and sats representations.
//!
//! All prices use this single struct with different USD types.
//! Sats is always lazily derived from USD via DollarsToSatsFract.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, SatsFract, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{BinaryTransform, Database, ReadableCloneableVec, UnaryTransform};

use super::{ComputedFromHeightLast, LazyBinaryFromHeightLast, LazyFromHeightLast};
use crate::{
    indexes,
    internal::{ComputedVecValue, DollarsToSatsFract, NumericValue},
};

/// Generic price metric with both USD and sats representations.
///
/// Derefs to the usd metric, so existing code works unchanged.
/// Access `.sats` for the sats exchange rate version.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct Price<U> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub usd: U,
    pub sats: LazyFromHeightLast<SatsFract, Dollars>,
}

// --- PriceFromHeight ---

pub type PriceFromHeight = Price<ComputedFromHeightLast<Dollars>>;

impl Price<ComputedFromHeightLast<Dollars>> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let usd = ComputedFromHeightLast::forced_import(db, name, version, indexes)?;
        Ok(Self::from_computed(name, version, usd))
    }

    pub(crate) fn from_computed(
        name: &str,
        version: Version,
        usd: ComputedFromHeightLast<Dollars>,
    ) -> Self {
        let sats = LazyFromHeightLast::from_computed::<DollarsToSatsFract>(
            &format!("{name}_sats"),
            version,
            usd.height.read_only_boxed_clone(),
            &usd,
        );
        Self { usd, sats }
    }
}

// --- LazyPriceFromHeight ---

pub type LazyPriceFromHeight<ST> = Price<LazyFromHeightLast<Dollars, ST>>;

impl<ST> Price<LazyFromHeightLast<Dollars, ST>>
where
    ST: ComputedVecValue + NumericValue + JsonSchema + 'static,
{
    pub(crate) fn from_computed<F: UnaryTransform<ST, Dollars>>(
        name: &str,
        version: Version,
        source: &ComputedFromHeightLast<ST>,
    ) -> Self {
        let usd = LazyFromHeightLast::from_computed::<F>(
            name,
            version,
            source.height.read_only_boxed_clone(),
            source,
        );
        let sats = LazyFromHeightLast::from_lazy::<DollarsToSatsFract, ST>(
            &format!("{name}_sats"),
            version,
            &usd,
        );
        Self { usd, sats }
    }
}

// --- LazyPriceFromCents ---

pub type LazyPriceFromCents = Price<LazyFromHeightLast<Dollars, Cents>>;

// --- LazyBinaryPriceFromHeight ---

pub type LazyBinaryPriceFromHeight = Price<LazyBinaryFromHeightLast<Dollars, Dollars, Dollars>>;

impl Price<LazyBinaryFromHeightLast<Dollars, Dollars, Dollars>> {
    /// Create from a PriceFromHeight (source1) and a LazyPriceFromCents (source2).
    pub(crate) fn from_price_and_lazy_price<F: BinaryTransform<Dollars, Dollars, Dollars>>(
        name: &str,
        version: Version,
        source1: &PriceFromHeight,
        source2: &LazyPriceFromCents,
    ) -> Self {
        let usd = LazyBinaryFromHeightLast::from_block_last_and_lazy_block_last::<F, Cents>(
            name,
            version,
            &source1.usd,
            &source2.usd,
        );

        let sats = LazyFromHeightLast::from_binary::<DollarsToSatsFract, _, _>(
            &format!("{name}_sats"),
            version,
            &usd,
        );

        Self { usd, sats }
    }

    /// Create from a LazyPriceFromCents (source1) and a PriceFromHeight (source2).
    pub(crate) fn from_lazy_price_and_price<F: BinaryTransform<Dollars, Dollars, Dollars>>(
        name: &str,
        version: Version,
        source1: &LazyPriceFromCents,
        source2: &PriceFromHeight,
    ) -> Self {
        let usd = LazyBinaryFromHeightLast::from_lazy_block_last_and_block_last::<F, Cents>(
            name,
            version,
            &source1.usd,
            &source2.usd,
        );

        let sats = LazyFromHeightLast::from_binary::<DollarsToSatsFract, _, _>(
            &format!("{name}_sats"),
            version,
            &usd,
        );

        Self { usd, sats }
    }
}

// --- Price bands (for stddev/ratio) ---

impl<S2T> Price<LazyBinaryFromHeightLast<Dollars, Dollars, S2T>>
where
    S2T: ComputedVecValue + NumericValue + JsonSchema,
{
    /// Create a price band from a computed price and a computed band.
    pub(crate) fn from_computed_price_and_band<F: BinaryTransform<Dollars, S2T, Dollars>>(
        name: &str,
        version: Version,
        price: &ComputedFromHeightLast<Dollars>,
        band: &ComputedFromHeightLast<S2T>,
    ) -> Self {
        let usd = LazyBinaryFromHeightLast::from_computed_last::<F>(name, version, price, band);

        let sats = LazyFromHeightLast::from_binary::<DollarsToSatsFract, _, _>(
            &format!("{name}_sats"),
            version,
            &usd,
        );

        Self { usd, sats }
    }

    /// Create a price band from a lazy price and a computed band.
    pub(crate) fn from_lazy_price_and_band<F: BinaryTransform<Dollars, S2T, Dollars>, S1T>(
        name: &str,
        version: Version,
        price: &LazyFromHeightLast<Dollars, S1T>,
        band: &ComputedFromHeightLast<S2T>,
    ) -> Self
    where
        S1T: ComputedVecValue + JsonSchema,
    {
        let usd = LazyBinaryFromHeightLast::from_lazy_block_last_and_block_last::<F, S1T>(
            name, version, price, band,
        );

        let sats = LazyFromHeightLast::from_binary::<DollarsToSatsFract, _, _>(
            &format!("{name}_sats"),
            version,
            &usd,
        );

        Self { usd, sats }
    }
}
