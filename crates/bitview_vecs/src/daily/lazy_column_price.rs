use bitview_transforms::{CentsUnsignedToDollars, DollarsToSatsFract};
use brk_types::{Cents, Day1, Dollars, SatsFract, Version};
use vecdb::{ColumnId, PcoVec, ReadOnlyColumnarVec, ReadableCloneableVec};

use crate::{DailyMappings, LazyColumnDailyMetric, LazyDailyMetric, Price};

pub type LazyColumnDailyPrice<C> = Price<
    LazyColumnDailyMetric<Cents, C>,
    LazyDailyMetric<Dollars, Cents>,
    LazyDailyMetric<SatsFract, Dollars>,
>;

impl<C> LazyColumnDailyPrice<C>
where
    C: ColumnId,
{
    pub fn new(
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Day1, Cents>, C>,
        column: C,
        mappings: &DailyMappings,
    ) -> Self {
        let cents =
            LazyColumnDailyMetric::new(&format!("{name}_cents"), version, source, column, mappings);
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
