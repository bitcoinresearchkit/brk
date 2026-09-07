use bitview_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, PriceRatio, SatsFract, StoredF32, Version};
use vecdb::{CachedBoxedVec, ReadableBoxedVec, ReadableCloneableVec, ReadableVec, TypedVec};

use crate::{Identity, IndexSources, LazyIndexedVec, LazyPerBlock, LazyRatioPerBlock, Price};

use super::price::{PRICE_RATIO_VERSION, cached_price_ratio, price_ratio};

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
    pub fn from_boxed_height_source(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<Height, Cents>,
        indexes: &IndexSources,
        spot_price: &CachedBoxedVec<Height, Cents>,
    ) -> Self {
        let source = LazyPerBlock::from_boxed_height_source::<Identity<Cents>>(
            &format!("{name}_cents"),
            version,
            source,
            indexes,
        );
        let price = Price::from_lazy_cents_source::<Identity<Cents>, Cents>(name, version, &source);
        let ratio = cached_price_ratio(
            name,
            version,
            price.cents.height.read_only_boxed_clone(),
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

    pub fn from_height_source<V>(
        name: &str,
        version: Version,
        source: V,
        indexes: &IndexSources,
        spot_price: &CachedBoxedVec<Height, Cents>,
    ) -> Self
    where
        V: TypedVec<I = Height, T = Cents> + ReadableVec<Height, Cents> + Clone + 'static,
    {
        let price = Price::from_height_source(name, version, source, indexes);
        let ratio_version = version + PRICE_RATIO_VERSION;
        let ppm_source = LazyIndexedVec::new(
            &format!("{name}_ratio_ppm_source"),
            ratio_version,
            price.cents.height.read_only_boxed_clone(),
            spot_price.clone(),
            |_, price, spot| price_ratio(spot, price),
        );
        let ratio = LazyRatioPerBlock::from_height_source(
            &format!("{name}_ratio"),
            ratio_version,
            ppm_source,
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
