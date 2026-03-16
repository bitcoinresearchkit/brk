use brk_traversable::Traversable;
use brk_types::Cents;
use vecdb::{Rw, StorageMode};

use super::ByLookbackPeriod;
use crate::internal::{PerBlock, Price};
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub price_past: ByLookbackPeriod<Price<PerBlock<Cents, M>>>,
}
