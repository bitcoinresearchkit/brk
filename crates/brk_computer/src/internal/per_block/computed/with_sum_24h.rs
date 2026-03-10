//! PerBlockWithSum24h - ComputedPerBlock + RollingWindow24hPerBlock rolling sum.
//!
//! Generic building block for metrics that store a per-block value
//! plus its 24h rolling sum. Used across activity and realized metrics.

use brk_traversable::Traversable;
use schemars::JsonSchema;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, ComputedVecValue, RollingWindow24hPerBlock};

#[derive(Traversable)]
pub struct PerBlockWithSum24h<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub raw: ComputedPerBlock<T, M>,
    pub sum: RollingWindow24hPerBlock<T, M>,
}
