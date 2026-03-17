use brk_traversable::Traversable;
use brk_types::BasisPointsSigned32;
use vecdb::{Rw, StorageMode};

use crate::{
    internal::{PercentPerBlock, StdDevPerBlock, Windows},
    market::{dca::ByDcaCagr, lookback::ByLookbackPeriod},
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub periods: ByLookbackPeriod<PercentPerBlock<BasisPointsSigned32, M>>,
    pub cagr: ByDcaCagr<PercentPerBlock<BasisPointsSigned32, M>>,
    pub sd_24h: Windows<StdDevPerBlock<M>>,
}
