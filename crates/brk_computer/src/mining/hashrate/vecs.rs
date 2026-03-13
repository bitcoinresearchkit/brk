use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned16, BasisPointsSigned32, StoredF32, StoredF64};
use vecdb::{Rw, StorageMode};

use crate::internal::{LazyPerBlock, PerBlock, PercentPerBlock};

#[derive(Traversable)]
pub struct HashRateSmaVecs<M: StorageMode = Rw> {
    pub _1w: PerBlock<StoredF64, M>,
    pub _1m: PerBlock<StoredF64, M>,
    pub _2m: PerBlock<StoredF64, M>,
    pub _1y: PerBlock<StoredF64, M>,
}

#[derive(Traversable)]
pub struct HashPriceValueVecs<M: StorageMode = Rw> {
    pub ths: PerBlock<StoredF32, M>,
    pub ths_min: PerBlock<StoredF32, M>,
    pub phs: LazyPerBlock<StoredF32>,
    pub phs_min: LazyPerBlock<StoredF32>,
    pub rebound: PercentPerBlock<BasisPointsSigned32, M>,
}

#[derive(Traversable)]
pub struct RateVecs<M: StorageMode = Rw> {
    pub base: PerBlock<StoredF64, M>,
    pub sma: HashRateSmaVecs<M>,
    pub ath: PerBlock<StoredF64, M>,
    pub drawdown: PercentPerBlock<BasisPointsSigned16, M>,
}

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub rate: RateVecs<M>,
    pub price: HashPriceValueVecs<M>,
    pub value: HashPriceValueVecs<M>,
}
