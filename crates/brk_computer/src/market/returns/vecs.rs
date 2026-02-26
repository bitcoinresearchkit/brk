use brk_traversable::Traversable;
use brk_types::{Height, StoredF32};
use vecdb::{EagerVec, PcoVec, Rw, StorageMode};

use crate::{
    internal::{ComputedFromHeightLast, ComputedFromHeightStdDev},
    market::{dca::ByDcaCagr, lookback::ByLookbackPeriod},
};

/// Price returns, CAGR, and returns standard deviation metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_returns: ByLookbackPeriod<ComputedFromHeightLast<StoredF32, M>>,

    // CAGR (computed from returns, 2y+ only)
    pub cagr: ByDcaCagr<ComputedFromHeightLast<StoredF32, M>>,

    // Returns standard deviation (computed from 1d returns)
    pub _1d_returns_1w_sd: ComputedFromHeightStdDev<M>,
    pub _1d_returns_1m_sd: ComputedFromHeightStdDev<M>,
    pub _1d_returns_1y_sd: ComputedFromHeightStdDev<M>,

    // Downside returns and deviation (for Sortino ratio)
    pub downside_returns: M::Stored<EagerVec<PcoVec<Height, StoredF32>>>,
    pub downside_1w_sd: ComputedFromHeightStdDev<M>,
    pub downside_1m_sd: ComputedFromHeightStdDev<M>,
    pub downside_1y_sd: ComputedFromHeightStdDev<M>,
}
