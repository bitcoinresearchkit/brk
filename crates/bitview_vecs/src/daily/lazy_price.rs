use bitview_transforms::{CentsUnsignedToDollars, DollarsToSatsFract};
use brk_types::{Cents, Day1, Dollars, SatsFract, Version};
use vecdb::{Ident, ReadableCloneableVec};

use crate::{DailyMappings, LazyDailyMetric, Price};

pub type LazyDailyPrice = Price<
    LazyDailyMetric<Cents, Cents>,
    LazyDailyMetric<Dollars, Cents>,
    LazyDailyMetric<SatsFract, Dollars>,
>;

impl LazyDailyPrice {
    pub fn from_day1_source(
        name: &str,
        version: Version,
        source: &(impl ReadableCloneableVec<Day1, Cents> + ?Sized),
        mappings: &DailyMappings,
    ) -> Self {
        let cents = LazyDailyMetric::from_source::<Ident>(
            &format!("{name}_cents"),
            version,
            source.read_only_boxed_clone(),
            mappings,
        );
        let usd = LazyDailyMetric::from_source::<CentsUnsignedToDollars>(
            name,
            version,
            cents.day1.read_only_boxed_clone(),
            mappings,
        );
        let sats = LazyDailyMetric::from_source::<DollarsToSatsFract>(
            &format!("{name}_sats"),
            version,
            usd.day1.read_only_boxed_clone(),
            mappings,
        );
        Self { usd, cents, sats }
    }
}
