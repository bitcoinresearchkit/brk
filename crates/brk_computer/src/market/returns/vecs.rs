use brk_traversable::Traversable;
use brk_types::BasisPointsSigned32;
use vecdb::{Rw, StorageMode};

use crate::{
    internal::{ComputedFromHeightStdDev, PercentFromHeight},
    market::{dca::ByDcaCagr, lookback::ByLookbackPeriod},
};
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_return: ByLookbackPeriod<PercentFromHeight<BasisPointsSigned32, M>>,

    // CAGR (computed from returns, 2y+ only)
    pub price_cagr: ByDcaCagr<PercentFromHeight<BasisPointsSigned32, M>>,

    // Returns standard deviation (computed from 24h returns)
    pub price_return_24h_sd_1w: ComputedFromHeightStdDev<M>,
    pub price_return_24h_sd_1m: ComputedFromHeightStdDev<M>,
    pub price_return_24h_sd_1y: ComputedFromHeightStdDev<M>,

}
