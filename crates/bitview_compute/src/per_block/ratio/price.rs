use brk_error::Result;

use bitview_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, PriceRatio, SatsFract, StoredF32, Version};
use vecdb::{
    CachedBoxedVec, Database, ReadableBoxedVec, ReadableCloneableVec, Rw, StorageMode, unlikely,
};

use crate::{
    CACHE_BUDGET, IndexSources, LazyIndexedVec, LazyPerBlock, LazyRatioPerBlock, PerBlock, Price,
};

pub(super) const PRICE_RATIO_VERSION: Version = Version::new(5);

#[derive(Traversable)]
pub struct PriceWithRatioPerBlock<M: StorageMode = Rw> {
    /// Reported in USD per BTC.
    pub usd: LazyPerBlock<Dollars, Cents>,
    /// Reported in cents per BTC.
    pub cents: PerBlock<Cents, M>,
    /// Reported in sats per USD: 100,000,000 divided by the price in USD per BTC.
    pub sats: LazyPerBlock<SatsFract, Dollars>,
    /// Spot price divided by this price in parts per million; 1,000,000
    /// represents a ratio of 1.0. Finite ratios saturate at 4,294.967294;
    /// a value at that ceiling means at least that ratio. Undefined values are NaN.
    pub ppm: LazyPerBlock<PriceRatio>,
    /// Spot price divided by this price as a unitless decimal ratio.
    pub ratio: LazyPerBlock<StoredF32, PriceRatio>,
}

impl PriceWithRatioPerBlock {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
        spot_price: &CachedBoxedVec<Height, Cents>,
    ) -> Result<Self> {
        let price = Price::forced_import(db, name, version, indexes)?;
        let ratio = cached_price_ratio(
            name,
            version,
            price.cents.height.read_only_boxed_clone(),
            spot_price,
            indexes,
        );
        Ok(Self {
            usd: price.usd,
            cents: price.cents,
            sats: price.sats,
            ppm: ratio.ppm,
            ratio: ratio.ratio,
        })
    }
}

pub(super) fn cached_price_ratio(
    name: &str,
    version: Version,
    price: ReadableBoxedVec<Height, Cents>,
    spot_price: &CachedBoxedVec<Height, Cents>,
    indexes: &IndexSources,
) -> LazyRatioPerBlock<PriceRatio> {
    let version = version + PRICE_RATIO_VERSION;
    let source = CACHE_BUDGET.wrap(LazyIndexedVec::new(
        &format!("{name}_ratio_ppm_source"),
        version,
        price,
        spot_price.clone(),
        |_, price, spot| price_ratio(spot, price),
    ));
    LazyRatioPerBlock::from_height_source(&format!("{name}_ratio"), version, source, indexes)
}

#[inline]
pub fn price_ratio(close: Cents, price: Cents) -> PriceRatio {
    if unlikely(price == Cents::ZERO) {
        PriceRatio::NAN
    } else {
        PriceRatio::from(f64::from(close) / f64::from(price))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_price_has_no_ratio() {
        assert!(price_ratio(Cents::new(100), Cents::ZERO).is_nan());
        assert_eq!(
            price_ratio(Cents::new(100), Cents::new(50)),
            PriceRatio::from(2.0),
        );
        assert_eq!(
            price_ratio(Cents::new(1_000_000), Cents::new(1)),
            PriceRatio::MAX
        );
        assert!(price_ratio(Cents::NAN, Cents::new(1)).is_nan());
        assert!(price_ratio(Cents::new(1), Cents::NAN).is_nan());
        assert_eq!(price_ratio(Cents::ZERO, Cents::new(1)), PriceRatio::ZERO);
    }
}
