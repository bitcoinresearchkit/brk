use bitview_traversable::Traversable;
use brk_types::{Cents, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::ReadableCloneableVec;

use crate::{
    CachedWindowStartVec, CentsUnsignedToDollars, LazyPerBlock, LazyRollingSumAmountFromHeight,
    LazyRollingSumFromHeight, SatsToBitcoin, Windows,
};

/// Lazy rolling sums for all 4 windows, for Amount (sats + btc + cents + usd).
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyRollingSumsAmountFromHeight(
    /// Total of the per-block values over the trailing window ending at the
    /// represented block. At time-period indexes, the value is taken at the
    /// period's final block.
    pub Windows<LazyRollingSumAmountFromHeight>,
);

impl LazyRollingSumsAmountFromHeight {
    pub fn new(
        name: &str,
        version: Version,
        cumulative_sats: &(impl ReadableCloneableVec<Height, Sats> + 'static),
        cumulative_cents: &(impl ReadableCloneableVec<Height, Cents> + 'static),
        cached_starts: &Windows<&CachedWindowStartVec>,
        indexes: &crate::IndexSources,
    ) -> Self {
        let cum_sats = cumulative_sats.read_only_boxed_clone();
        let cum_cents = cumulative_cents.read_only_boxed_clone();

        let make_slot = |suffix: &str, cached_start: &&CachedWindowStartVec| {
            let full_name = format!("{name}_{suffix}");

            // Sats lazy rolling sum
            let sats = LazyRollingSumFromHeight::new(
                &format!("{full_name}_sats"),
                version,
                cum_sats.clone(),
                cached_start,
                indexes,
            );

            // Btc lazy from sats
            let btc = LazyPerBlock::from_resolutions::<SatsToBitcoin>(
                &full_name,
                version,
                sats.height.read_only_boxed_clone(),
                &sats.resolutions,
            );

            // Cents rolling sum
            let cents = LazyRollingSumFromHeight::new(
                &format!("{full_name}_cents"),
                version,
                cum_cents.clone(),
                cached_start,
                indexes,
            );

            // Usd lazy from cents
            let usd = LazyPerBlock::from_resolutions::<CentsUnsignedToDollars>(
                &format!("{full_name}_usd"),
                version,
                cents.height.read_only_boxed_clone(),
                &cents.resolutions,
            );

            LazyRollingSumAmountFromHeight {
                btc,
                sats,
                usd,
                cents,
            }
        };

        Self(cached_starts.map_with_suffix(make_slot))
    }
}
