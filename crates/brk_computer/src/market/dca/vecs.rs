use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Cents, Height, Sats};
use vecdb::{EagerVec, PcoVec, Rw, StorageMode};

use super::{ByDcaCagr, ByDcaClass, ByDcaPeriod};
use crate::internal::{AmountPerBlock, ComputedPerBlock, PercentPerBlock, Price};

#[derive(Traversable)]
pub struct PeriodVecs<M: StorageMode = Rw> {
    pub stack: ByDcaPeriod<AmountPerBlock<M>>,
    pub cost_basis: ByDcaPeriod<Price<ComputedPerBlock<Cents, M>>>,
    pub r#return: ByDcaPeriod<PercentPerBlock<BasisPointsSigned32, M>>,
    pub cagr: ByDcaCagr<PercentPerBlock<BasisPointsSigned32, M>>,
    pub lump_sum_stack: ByDcaPeriod<AmountPerBlock<M>>,
    pub lump_sum_return: ByDcaPeriod<PercentPerBlock<BasisPointsSigned32, M>>,
}

#[derive(Traversable)]
pub struct ClassVecs<M: StorageMode = Rw> {
    pub stack: ByDcaClass<AmountPerBlock<M>>,
    pub cost_basis: ByDcaClass<Price<ComputedPerBlock<Cents, M>>>,
    pub r#return: ByDcaClass<PercentPerBlock<BasisPointsSigned32, M>>,
}

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub sats_per_day: M::Stored<EagerVec<PcoVec<Height, Sats>>>,
    pub period: PeriodVecs<M>,
    pub class: ClassVecs<M>,
}
