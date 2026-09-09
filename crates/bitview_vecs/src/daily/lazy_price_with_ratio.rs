use brk_types::{Cents, Day1, Height, Version};
use vecdb::{LazyVec, ReadableCloneableVec};

use crate::{DailyMappings, IndexSources, LazyDailyPrice, LazyRatioPerBlock, PriceWithRatio};

/// Daily prices repeat the day's observation at finer indexes.
pub type LazyDailyPriceWithRatio = PriceWithRatio<LazyDailyPrice>;

impl LazyDailyPriceWithRatio {
    pub fn from_day1_source(
        name: &str,
        version: Version,
        source: &(impl ReadableCloneableVec<Day1, Cents> + ?Sized),
        indexes: &IndexSources,
        mappings: &DailyMappings,
        spot: &impl ReadableCloneableVec<Height, Cents>,
    ) -> Self {
        let price = LazyDailyPrice::from_day1_source(name, version, source, mappings);
        let reference = LazyVec::init(
            &format!("{name}_daily_reference"),
            version,
            price.cents.views.height.read_only_boxed_clone(),
            |_, value| value.unwrap_or(Cents::NAN),
        );
        let relative =
            LazyRatioPerBlock::from_price_source(name, version, &reference, spot, indexes);
        Self { price, relative }
    }
}
