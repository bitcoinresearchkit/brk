use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{DeltaSub, LazyDeltaVec, LazyVecFrom1, ReadableCloneableVec};

use crate::{
    indexes,
    internal::{
        CachedWindowStarts, CentsUnsignedToDollars, DerivedResolutions, LazyPerBlock,
        LazyRollingSumFromHeight, Resolutions, SatsToBitcoin, Windows,
    },
};

/// Single window slot: lazy rolling sum for Amount (sats + btc + cents + usd).
#[derive(Clone, Traversable)]
pub struct LazyRollingSumAmountFromHeight {
    pub sats: LazyRollingSumFromHeight<Sats>,
    pub btc: LazyPerBlock<Bitcoin, Sats>,
    pub cents: LazyRollingSumFromHeight<Cents>,
    pub usd: LazyPerBlock<Dollars, Cents>,
}

/// Lazy rolling sums for all 4 windows, for Amount (sats + btc + cents + usd).
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyRollingSumsAmountFromHeight(pub Windows<LazyRollingSumAmountFromHeight>);

impl LazyRollingSumsAmountFromHeight {
    pub fn new(
        name: &str,
        version: Version,
        cumulative_sats: &(impl ReadableCloneableVec<Height, Sats> + 'static),
        cumulative_cents: &(impl ReadableCloneableVec<Height, Cents> + 'static),
        cached_starts: &CachedWindowStarts,
        indexes: &indexes::Vecs,
    ) -> Self {
        let cum_sats = cumulative_sats.read_only_boxed_clone();
        let cum_cents = cumulative_cents.read_only_boxed_clone();

        let make_slot = |suffix: &str, cached_start: &vecdb::CachedVec<Height, Height>| {
            let full_name = format!("{name}_{suffix}");
            let cached = cached_start.clone();
            let starts_version = cached.version();

            // Sats lazy rolling sum
            let sats_sum = LazyDeltaVec::<Height, Sats, Sats, DeltaSub>::new(
                &format!("{full_name}_sats"),
                version,
                cum_sats.clone(),
                starts_version,
                {
                    let cached = cached.clone();
                    move || cached.get()
                },
            );
            let sats_resolutions = Resolutions::forced_import(
                &format!("{full_name}_sats"),
                sats_sum.read_only_boxed_clone(),
                version,
                indexes,
            );
            let sats = LazyRollingSumFromHeight {
                height: sats_sum,
                resolutions: Box::new(sats_resolutions),
            };

            // Btc lazy from sats
            let btc = LazyPerBlock {
                height: LazyVecFrom1::transformed::<SatsToBitcoin>(
                    &full_name,
                    version,
                    sats.height.read_only_boxed_clone(),
                ),
                resolutions: Box::new(DerivedResolutions::from_derived_computed::<SatsToBitcoin>(
                    &full_name,
                    version,
                    &sats.resolutions,
                )),
            };

            // Cents rolling sum
            let cents_sum = LazyDeltaVec::<Height, Cents, Cents, DeltaSub>::new(
                &format!("{full_name}_cents"),
                version,
                cum_cents.clone(),
                starts_version,
                move || cached.get(),
            );
            let cents_resolutions = Resolutions::forced_import(
                &format!("{full_name}_cents"),
                cents_sum.read_only_boxed_clone(),
                version,
                indexes,
            );
            let cents = LazyRollingSumFromHeight {
                height: cents_sum,
                resolutions: Box::new(cents_resolutions),
            };

            // Usd lazy from cents
            let usd = LazyPerBlock {
                height: LazyVecFrom1::transformed::<CentsUnsignedToDollars>(
                    &format!("{full_name}_usd"),
                    version,
                    cents.height.read_only_boxed_clone(),
                ),
                resolutions: Box::new(DerivedResolutions::from_derived_computed::<
                    CentsUnsignedToDollars,
                >(
                    &format!("{full_name}_usd"), version, &cents.resolutions
                )),
            };

            LazyRollingSumAmountFromHeight {
                sats,
                btc,
                cents,
                usd,
            }
        };

        Self(cached_starts.0.map_with_suffix(make_slot))
    }
}
