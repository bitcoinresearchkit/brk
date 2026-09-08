use brk_types::{Cents, Height, Version};
use vecdb::{CacheBudget, ColumnId, PcoVec, ReadOnlyColumnarVec, ReadableCloneableVec};

use crate::{IndexSources, LazyColumnPerBlock, LazyRatioPerBlock, Price};

pub type LazyColumnPriceWithRatioPerBlock<C> =
    crate::PriceWithRatio<Price<LazyColumnPerBlock<Cents, C>>>;

impl<C> LazyColumnPriceWithRatioPerBlock<C>
where
    C: ColumnId,
{
    pub fn new(
        cache: &'static CacheBudget,
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, Cents>, C>,
        column: C,
        indexes: &IndexSources,
        spot_price: &impl ReadableCloneableVec<Height, Cents>,
    ) -> Self {
        let price = Price::from_columnar_source(cache, name, version, source, column, indexes);
        let ratio = LazyRatioPerBlock::from_price_source(
            name,
            version,
            price.cents.resolutions.height_source(),
            spot_price,
            indexes,
        );

        Self {
            price,
            relative: ratio,
        }
    }
}
