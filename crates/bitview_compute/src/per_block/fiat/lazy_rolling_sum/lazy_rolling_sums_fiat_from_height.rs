use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::ReadableCloneableVec;

use crate::{FiatType, IndexSources, LazyPerBlock, LazyRollingSumFromHeight, Windows};

use super::LazyRollingSumFiatFromHeight;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyRollingSumsFiatFromHeight<C: FiatType>(
    /// Total of the per-block values over the trailing window ending at the
    /// represented block. At time-period indexes, the value is taken at the
    /// period's final block.
    pub Windows<LazyRollingSumFiatFromHeight<C>>,
);

impl<C: FiatType> LazyRollingSumsFiatFromHeight<C> {
    pub fn new(
        name: &str,
        version: Version,
        cumulative_cents: &impl ReadableCloneableVec<Height, C>,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
        indexes: &IndexSources,
    ) -> Self {
        let cumulative_cents = cumulative_cents.read_only_boxed_clone();

        Self(window_starts.map_with_suffix(|suffix, window_start| {
            let name = format!("{name}_{suffix}");
            let cents = LazyRollingSumFromHeight::new(
                &format!("{name}_cents"),
                version,
                cumulative_cents.clone(),
                *window_start,
                indexes,
            );

            let usd =
                LazyPerBlock::from_resolutions::<C::ToDollars>(&name, version, &cents.resolutions);

            LazyRollingSumFiatFromHeight { usd, cents }
        }))
    }
}
