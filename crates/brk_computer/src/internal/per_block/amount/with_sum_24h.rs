//! AmountPerBlockWithSum24h - AmountPerBlock raw + RollingWindow24hAmountPerBlock sum.

use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::{AmountPerBlock, RollingWindow24hAmountPerBlock};

/// Amount per-block value (sats + cents) with 24h rolling sum (also amount).
#[derive(Traversable)]
pub struct AmountPerBlockWithSum24h<M: StorageMode = Rw> {
    pub raw: AmountPerBlock<M>,
    pub sum: RollingWindow24hAmountPerBlock<M>,
}
