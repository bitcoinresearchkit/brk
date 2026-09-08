use brk_types::{Cents, Day1, Height, Version};
use vecdb::{CacheBudget, ColumnId, LazyVec, PcoVec, ReadOnlyColumnarVec, ReadableCloneableVec};

use crate::{DailyMappings, IndexSources, LazyColumnDailyPrice, LazyRatioPerBlock};

/// One daily price source. Finer price views repeat that day's value; they are
/// not intraday observations. Ratios compare spot with that daily reference.
pub type LazyColumnDailyPriceWithRatio<C> = crate::PriceWithRatio<LazyColumnDailyPrice<C>>;

impl<C: ColumnId> LazyColumnDailyPriceWithRatio<C> {
    pub fn new(
        cache: &'static CacheBudget,
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Day1, Cents>, C>,
        column: C,
        indexes: &IndexSources,
        mappings: &DailyMappings,
        spot: &impl ReadableCloneableVec<Height, Cents>,
    ) -> Self {
        let price = LazyColumnDailyPrice::new(cache, name, version, source, column, mappings);
        let reference = LazyVec::init(
            &format!("{name}_daily_reference"),
            version,
            price.cents.views.height.read_only_boxed_clone(),
            |_, value| value.unwrap_or(Cents::NAN),
        );
        let ratio = LazyRatioPerBlock::from_price_source(name, version, &reference, spot, indexes);
        Self {
            price,
            relative: ratio,
        }
    }
}
