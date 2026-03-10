use brk_traversable::Traversable;
use brk_types::BasisPointsSigned32;
use vecdb::{Rw, StorageMode};

use crate::{
    internal::{PercentPerBlock, StdDevPerBlock},
    market::{dca::ByDcaCagr, lookback::ByLookbackPeriod},
};

#[derive(Traversable)]
pub struct PriceReturn24hSdVecs<M: StorageMode = Rw> {
    pub _1w: StdDevPerBlock<M>,
    pub _1m: StdDevPerBlock<M>,
    pub _1y: StdDevPerBlock<M>,
}

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_return: ByLookbackPeriod<PercentPerBlock<BasisPointsSigned32, M>>,
    pub price_cagr: ByDcaCagr<PercentPerBlock<BasisPointsSigned32, M>>,
    pub price_return_24h_sd: PriceReturn24hSdVecs<M>,
}
