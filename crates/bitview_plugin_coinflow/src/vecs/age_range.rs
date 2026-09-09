use bitview_cohort::AgeRange;
use bitview_traversable::Traversable;
use brk_types::{BoundedRatio, Height, StoredF64};
use vecdb::{Rw, StorageMode};

use bitview_vecs::{LazySpotValuePerBlock, PerBlock, StoredSeries};

use super::{Mobility, SpendingExposureSeries};

#[derive(Traversable)]
pub struct AgeRangeVecs<M: StorageMode = Rw> {
    /// Empirical daily spending hazard for each UTXO age range: cumulative
    /// transfer volume in BTC divided by cumulative coin days created in that
    /// range. It estimates the fraction of the range's supply spent per day;
    /// higher values indicate faster turnover. Returns zero when cumulative
    /// coin days created is zero.
    pub spending_rate: AgeRange<PerBlock<StoredF64, M>>,
    /// Estimated remaining-lifetime spending exposure for each UTXO age range.
    /// It integrates observed positive spending hazards from the range midpoint
    /// through subsequent complete ranges, then integrates an exponential tail
    /// fitted by duration-weighted regression of log hazard on age. Returns
    /// zero when a decreasing finite tail cannot be fitted. Larger exposure
    /// implies a greater eventual probability of spending.
    pub spending_exposure: SpendingExposureSeries<M>,
    /// Canonical bounded spending probability, batched by age range.
    #[traversable(hidden)]
    pub mobility_source: AgeRange<StoredSeries<Height, BoundedRatio, M>>,
    pub supply: Mobility<AgeRange<LazySpotValuePerBlock>>,
}
