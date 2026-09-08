use bitview_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, PriceRatio, SatsFract, StoredF32, Version};
use vecdb::ReadableCloneableVec;

use crate::{IndexSources, LazyPerBlock, LazyRatioPerBlock, Price};

#[derive(Clone, Traversable)]
pub struct LazyPriceWithRatioPerBlock {
    /// Reported in USD per BTC.
    pub usd: LazyPerBlock<Dollars, Cents>,
    /// Reported in cents per BTC.
    pub cents: LazyPerBlock<Cents>,
    /// Reported in sats per USD: 100,000,000 divided by the price in USD per BTC.
    pub sats: LazyPerBlock<SatsFract, Dollars>,
    /// Spot price divided by this price in parts per million; 1,000,000
    /// represents a ratio of 1.0. Finite ratios saturate at 4,294.967294;
    /// a value at that ceiling means at least that ratio. Undefined values are NaN.
    pub ppm: LazyPerBlock<PriceRatio>,
    /// Spot price divided by this price as a unitless decimal ratio.
    pub ratio: LazyPerBlock<StoredF32, PriceRatio>,
}

impl LazyPriceWithRatioPerBlock {
    pub fn from_height_source<V>(
        name: &str,
        version: Version,
        source: &V,
        indexes: &IndexSources,
        spot_price: &impl ReadableCloneableVec<Height, Cents>,
    ) -> Self
    where
        V: ReadableCloneableVec<Height, Cents> + ?Sized,
    {
        let price = Price::from_height_source(name, version, source, indexes);
        let ratio = LazyRatioPerBlock::from_price_source(
            name,
            version,
            &price.cents.height,
            spot_price,
            indexes,
        );

        Self {
            usd: price.usd,
            cents: price.cents,
            sats: price.sats,
            ppm: ratio.ppm,
            ratio: ratio.ratio,
        }
    }
}
