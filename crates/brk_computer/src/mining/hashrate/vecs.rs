use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned16, BasisPointsSigned32, StoredF32, StoredF64};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, PercentPerBlock};

#[derive(Traversable)]
pub struct HashRateSmaVecs<M: StorageMode = Rw> {
    pub _1w: ComputedPerBlock<StoredF64, M>,
    pub _1m: ComputedPerBlock<StoredF64, M>,
    pub _2m: ComputedPerBlock<StoredF64, M>,
    pub _1y: ComputedPerBlock<StoredF64, M>,
}

#[derive(Traversable)]
pub struct HashPriceValueVecs<M: StorageMode = Rw> {
    pub ths: ComputedPerBlock<StoredF32, M>,
    pub ths_min: ComputedPerBlock<StoredF32, M>,
    pub phs: ComputedPerBlock<StoredF32, M>,
    pub phs_min: ComputedPerBlock<StoredF32, M>,
    pub rebound: PercentPerBlock<BasisPointsSigned32, M>,
}

#[derive(Traversable)]
pub struct RateVecs<M: StorageMode = Rw> {
    pub raw: ComputedPerBlock<StoredF64, M>,
    pub sma: HashRateSmaVecs<M>,
    pub ath: ComputedPerBlock<StoredF64, M>,
    pub drawdown: PercentPerBlock<BasisPointsSigned16, M>,
}

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub rate: RateVecs<M>,
    pub price: HashPriceValueVecs<M>,
    pub value: HashPriceValueVecs<M>,
}
