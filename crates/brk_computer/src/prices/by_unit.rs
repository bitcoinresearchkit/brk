use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, OHLCCents, OHLCDollars, OHLCSats, Sats};
use vecdb::{LazyVecFrom1, PcoVec, Rw, StorageMode};

use crate::internal::{ComputedHeightDerivedLast, EagerIndexes, LazyEagerIndexes};

use super::ohlcs::{LazyOhlcVecs, OhlcVecs};

// ── SplitByUnit ─────────────────────────────────────────────────────

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
    pub cents: ComputedHeightDerivedLast<Cents>,
    pub usd: ComputedHeightDerivedLast<Dollars>,
    pub sats: ComputedHeightDerivedLast<Sats>,
}

// ── OhlcByUnit ──────────────────────────────────────────────────────

#[derive(Traversable)]
pub struct OhlcByUnit<M: StorageMode = Rw> {
    pub cents: OhlcVecs<OHLCCents, M>,
    pub usd: LazyOhlcVecs<OHLCDollars, OHLCCents>,
    pub sats: LazyOhlcVecs<OHLCSats, OHLCCents>,
}

// ── PriceByUnit ─────────────────────────────────────────────────────

#[derive(Traversable)]
pub struct PriceByUnit<M: StorageMode = Rw> {
    pub cents: M::Stored<PcoVec<Height, Cents>>,
    pub usd: LazyVecFrom1<Height, Dollars, Height, Cents>,
    pub sats: LazyVecFrom1<Height, Sats, Height, Cents>,
}
