use bitview_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, PriceRatio, SatsFract, StoredF32, Version};
use vecdb::{ColumnId, PcoVec, ReadOnlyColumnarVec, ReadableCloneableVec};

use crate::{IndexSources, LazyColumnPerBlock, LazyPerBlock, LazyRatioPerBlock, Price};

#[derive(Clone, Traversable)]
pub struct LazyColumnPriceWithRatioPerBlock<C>
where
    C: ColumnId,
{
    /// Reported in USD per BTC.
    pub usd: LazyPerBlock<Dollars, Cents>,
    /// Reported in cents per BTC.
    pub cents: LazyColumnPerBlock<Cents, C>,
    /// Reported in sats per USD: 100,000,000 divided by the price in USD per BTC.
    pub sats: LazyPerBlock<SatsFract, Dollars>,
    /// Spot price divided by this price in parts per million; 1,000,000
    /// represents a ratio of 1.0. Finite ratios saturate at 4,294.967294;
    /// a value at that ceiling means at least that ratio. Undefined values are NaN.
    pub ppm: LazyPerBlock<PriceRatio>,
    /// Spot price divided by this price as a unitless decimal ratio.
    pub ratio: LazyPerBlock<StoredF32, PriceRatio>,
}

impl<C> LazyColumnPriceWithRatioPerBlock<C>
where
    C: ColumnId,
{
    pub fn new(
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, Cents>, C>,
        column: C,
        indexes: &IndexSources,
        spot_price: &impl ReadableCloneableVec<Height, Cents>,
    ) -> Self {
        let price = Price::from_columnar_source(name, version, source, column, indexes);
        let ratio = LazyRatioPerBlock::from_price_source(
            name,
            version,
            price.cents.resolutions.height_source(),
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
