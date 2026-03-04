use brk_traversable::Traversable;
use brk_types::Cents;
use vecdb::{Rw, StorageMode};

use super::ByLookbackPeriod;
use crate::internal::{ComputedFromHeight, Price};
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub price_lookback: ByLookbackPeriod<Price<ComputedFromHeight<Cents, M>>>,
}
