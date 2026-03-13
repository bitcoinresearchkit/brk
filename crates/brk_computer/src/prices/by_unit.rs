use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, OHLCCents, OHLCDollars, OHLCSats, Sats};
use vecdb::{Rw, StorageMode};

use crate::internal::{
    PerBlock, Resolutions, EagerIndexes, LazyEagerIndexes, LazyPerBlock,
};

use super::ohlcs::{LazyOhlcVecs, OhlcVecs};

#[derive(Traversable)]
pub struct SplitByUnit<M: StorageMode = Rw> {
    pub open: SplitIndexesByUnit<M>,
    pub high: SplitIndexesByUnit<M>,
    pub low: SplitIndexesByUnit<M>,
    pub close: SplitCloseByUnit,
}

#[derive(Traversable)]
pub struct SplitIndexesByUnit<M: StorageMode = Rw> {
    pub cents: EagerIndexes<Cents, M>,
    pub usd: LazyEagerIndexes<Dollars, Cents>,
    pub sats: LazyEagerIndexes<Sats, Cents>,
}

#[derive(Clone, Traversable)]
pub struct SplitCloseByUnit {
    pub cents: Resolutions<Cents>,
    pub usd: Resolutions<Dollars>,
    pub sats: Resolutions<Sats>,
}

#[derive(Traversable)]
pub struct OhlcByUnit<M: StorageMode = Rw> {
    pub cents: OhlcVecs<OHLCCents, M>,
    pub usd: LazyOhlcVecs<OHLCDollars, OHLCCents>,
    pub sats: LazyOhlcVecs<OHLCSats, OHLCCents>,
}

#[derive(Traversable)]
pub struct PriceByUnit<M: StorageMode = Rw> {
    pub cents: PerBlock<Cents, M>,
    pub usd: LazyPerBlock<Dollars, Cents>,
    pub sats: LazyPerBlock<Sats, Cents>,
}
