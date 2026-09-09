use brk_error::Result;
use brk_types::{Cents, Height, Version};
use vecdb::{CacheBudget, Database, ReadableCloneableVec, Rw};

use crate::{IndexSources, LazyRatioPerBlock, PerBlock, Price, PriceWithRatio};

pub type PriceWithRatioPerBlock<M = Rw> = PriceWithRatio<Price<PerBlock<Cents, M>>>;

impl PriceWithRatioPerBlock {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
        spot_price: &impl ReadableCloneableVec<Height, Cents>,
    ) -> Result<Self> {
        let price = Price::forced_import(cache, db, name, version, indexes)?;
        let ratio = LazyRatioPerBlock::from_price_source(
            name,
            version,
            price.cents.resolutions.height_source(),
            spot_price,
            indexes,
        );
        Ok(Self {
            price,
            relative: ratio,
        })
    }
}
