use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{DeltaAvg, LazyDeltaVec, LazyVecFrom1, ReadableCloneableVec};

use crate::{
    indexes,
    internal::{
        CachedWindowStarts, CentsUnsignedToDollars, DerivedResolutions, LazyPerBlock,
        LazyRollingAvgFromHeight, Resolutions, SatsToBitcoin, Windows,
    },
};

/// Single window slot: lazy rolling average for Amount (sats + btc + cents + usd).
#[derive(Clone, Traversable)]
pub struct LazyRollingAvgAmountFromHeight {
    pub btc: LazyPerBlock<Bitcoin, Sats>,
    pub sats: LazyRollingAvgFromHeight<Sats>,
    pub usd: LazyPerBlock<Dollars, Cents>,
    pub cents: LazyRollingAvgFromHeight<Cents>,
}

/// Lazy rolling averages for all 4 windows, for Amount (sats + btc + cents + usd).
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyRollingAvgsAmountFromHeight(pub Windows<LazyRollingAvgAmountFromHeight>);

impl LazyRollingAvgsAmountFromHeight {
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

            // Sats lazy rolling avg
            let sats_avg = LazyDeltaVec::<Height, Sats, Sats, DeltaAvg>::new(
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
                sats_avg.read_only_boxed_clone(),
                version,
                indexes,
            );
            let sats = LazyRollingAvgFromHeight {
                height: sats_avg,
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

            // Cents rolling avg
            let cents_avg = LazyDeltaVec::<Height, Cents, Cents, DeltaAvg>::new(
                &format!("{full_name}_cents"),
                version,
                cum_cents.clone(),
                starts_version,
                move || cached.get(),
            );
            let cents_resolutions = Resolutions::forced_import(
                &format!("{full_name}_cents"),
                cents_avg.read_only_boxed_clone(),
                version,
                indexes,
            );
            let cents = LazyRollingAvgFromHeight {
                height: cents_avg,
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

            LazyRollingAvgAmountFromHeight {
                btc,
                sats,
                usd,
                cents,
            }
        };

        Self(cached_starts.0.map_with_suffix(make_slot))
    }
}
