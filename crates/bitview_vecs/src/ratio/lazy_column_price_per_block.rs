use brk_types::{Cents, Height, Version};
use vecdb::{ColumnId, PcoVec, ReadOnlyColumnarVec, ReadableCloneableVec};

use crate::{IndexSources, LazyColumnPerBlock, LazyRatioPerBlock, Price, PriceWithRatio};

pub type LazyColumnPriceWithRatioPerBlock<C> = PriceWithRatio<Price<LazyColumnPerBlock<Cents, C>>>;

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
            price,
            relative: ratio,
        }
    }
}
