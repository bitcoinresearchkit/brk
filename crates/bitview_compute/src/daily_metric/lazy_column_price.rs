use bitview_traversable::Traversable;
use brk_types::{Cents, Day1, Dollars, Height, PriceRatio, SatsFract, StoredF32, Version};
use vecdb::{ColumnId, LazyVec, PcoVec, ReadOnlyColumnarVec, ReadableCloneableVec};

use crate::{
    CentsUnsignedToDollars, DailyMappings, DollarsToSatsFract, IndexSources, LazyColumnDailyMetric,
    LazyDailyMetric, LazyPerBlock, LazyRatioPerBlock,
};

/// One daily price source. Finer price views repeat that day's value; they are
/// not intraday observations. Ratios compare spot with that daily reference.
#[derive(Clone, Traversable)]
pub struct LazyColumnDailyPriceWithRatio<C: ColumnId> {
    /// Daily price in USD per BTC.
    pub usd: LazyDailyMetric<Dollars, Cents>,
    /// Daily price in cents per BTC, read from the source column.
    pub cents: LazyColumnDailyMetric<Cents, C>,
    /// Satoshis per USD, the reciprocal of the USD price.
    pub sats: LazyDailyMetric<SatsFract, Dollars>,
    /// Spot/reference ratio in PPM. Finite overflow saturates at 4,294.967294;
    /// that ceiling means at least the maximum. Undefined values remain NaN.
    pub ppm: LazyPerBlock<PriceRatio>,
    /// Unitless spot/reference ratio derived from PriceRatio.
    pub ratio: LazyPerBlock<StoredF32, PriceRatio>,
}

impl<C: ColumnId> LazyColumnDailyPriceWithRatio<C> {
    pub fn new(
        name: &str,
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Day1, Cents>, C>,
        column: C,
        indexes: &IndexSources,
        mappings: &DailyMappings,
        spot: &impl ReadableCloneableVec<Height, Cents>,
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
        let reference = LazyVec::init(
            &format!("{name}_daily_reference"),
            version,
            cents.views.height.read_only_boxed_clone(),
            |_, value| value.unwrap_or(Cents::NAN),
        );
        let ratio = LazyRatioPerBlock::from_price_source(name, version, &reference, spot, indexes);
        Self {
            usd,
            cents,
            sats,
            ppm: ratio.ppm,
            ratio: ratio.ratio,
        }
    }
}
