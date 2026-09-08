use bitview_traversable::Traversable;
use brk_types::{Cents, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::ReadableCloneableVec;

use crate::{
    AvgCentsToUsd, AvgSatsToBtc, IndexSources, LazyPerBlock, LazyRollingAvgAmountFromHeight,
    LazyRollingAvgFromHeight, Windows,
};

/// Lazy rolling averages for all 4 windows, with StoredF32 sats/cents and BTC/USD views.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyRollingAvgsAmountFromHeight(
    /// Arithmetic mean of the per-block values over the trailing window ending
    /// at the represented block; each block has equal weight. At time-period
    /// indexes, the value is taken at the period's final block.
    pub Windows<LazyRollingAvgAmountFromHeight>,
);

impl LazyRollingAvgsAmountFromHeight {
    pub fn new(
        name: &str,
        version: Version,
        cumulative_sats: &impl ReadableCloneableVec<Height, Sats>,
        cumulative_cents: &impl ReadableCloneableVec<Height, Cents>,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
        indexes: &IndexSources,
    ) -> Self {
        let cum_sats = cumulative_sats.read_only_boxed_clone();
        let cum_cents = cumulative_cents.read_only_boxed_clone();

        Self(window_starts.map_with_suffix(|suffix, window_start| {
            let full_name = format!("{name}_{suffix}");

            // Sats rolling average, stored as a float.
            let sats = LazyRollingAvgFromHeight::new(
                &format!("{full_name}_sats"),
                version,
                cum_sats.clone(),
                *window_start,
                indexes,
            );

            // BTC from the sats average.
            let btc = LazyPerBlock::from_resolutions::<AvgSatsToBtc>(
                &full_name,
                version,
                &sats.resolutions,
            );

            // Cents rolling average, stored as a float.
            let cents = LazyRollingAvgFromHeight::new(
                &format!("{full_name}_cents"),
                version,
                cum_cents.clone(),
                *window_start,
                indexes,
            );

            // USD from the cents average.
            let usd = LazyPerBlock::from_resolutions::<AvgCentsToUsd>(
                &format!("{full_name}_usd"),
                version,
                &cents.resolutions,
            );

            LazyRollingAvgAmountFromHeight {
                btc,
                sats,
                usd,
                cents,
            }
        }))
    }
}
