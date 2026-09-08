use bitview_traversable::Traversable;
use brk_types::{Cents, Dollars, Sats};
use vecdb::{Pinned, Rw, StorageMode};

use bitview_compute::{LazyPerBlock, PerBlock};

#[derive(Traversable)]
pub struct PriceByUnit<M: StorageMode = Rw> {
    /// Reported in USD per BTC.
    pub usd: LazyPerBlock<Dollars, Cents>,
    /// Reported in cents per BTC.
    pub cents: PerBlock<Cents, M, Pinned>,
    /// Reported in sats per USD: 100,000,000 divided by the price in USD per BTC.
    pub sats: LazyPerBlock<Sats, Cents>,
}
