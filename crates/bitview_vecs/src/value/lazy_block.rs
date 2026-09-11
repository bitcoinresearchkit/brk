use bitview_transforms::{CentsUnsignedToDollars, SatsToBitcoin};
use brk_types::{Bitcoin, Cents, Dollars, Height, Sats, Version};
use vecdb::{CachePolicy, LazyVec, ReadableCloneableVec, Rw};

use crate::{LazyPreviousDeltaVec, Value, ValuePerBlock};

/// Per-block amount data derived from stored cumulative sats and cents.
pub type LazyValueBlock = Value<
    LazyPreviousDeltaVec<Height, Sats>,
    LazyPreviousDeltaVec<Height, Cents>,
    LazyVec<Height, Bitcoin, Height, Sats>,
    LazyVec<Height, Dollars, Height, Cents>,
>;

impl LazyValueBlock {
    pub fn from_cumulative<P: CachePolicy>(
        name: &str,
        version: Version,
        cumulative: &ValuePerBlock<Rw, P>,
    ) -> Self {
        Self::from_cumulative_sources(
            name,
            version,
            cumulative.sats.resolutions.height_source(),
            cumulative.cents.resolutions.height_source(),
        )
    }

    pub fn from_cumulative_sources(
        name: &str,
        version: Version,
        cumulative_sats: &impl ReadableCloneableVec<Height, Sats>,
        cumulative_cents: &impl ReadableCloneableVec<Height, Cents>,
    ) -> Self {
        let sats = LazyPreviousDeltaVec::new(&format!("{name}_sats"), version, cumulative_sats);
        let btc =
            LazyVec::transformed::<SatsToBitcoin>(name, version, sats.read_only_boxed_clone());
        let cents = LazyPreviousDeltaVec::new(&format!("{name}_cents"), version, cumulative_cents);
        let usd = LazyVec::transformed::<CentsUnsignedToDollars>(
            &format!("{name}_usd"),
            version,
            cents.read_only_boxed_clone(),
        );

        Self {
            btc,
            sats,
            usd,
            cents,
        }
    }
}
