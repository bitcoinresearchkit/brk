use bitview_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::ReadableCloneableVec;

use crate::{CachedWindowStartVec, FiatType, LazyPerBlock, LazyRollingSumFromHeight, Windows};

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
        cumulative_cents: &(impl ReadableCloneableVec<Height, C> + 'static),
        cached_starts: &Windows<&CachedWindowStartVec>,
        indexes: &crate::IndexSources,
    ) -> Self {
        let cumulative_cents = cumulative_cents.read_only_boxed_clone();

        Self(cached_starts.map_with_suffix(|suffix, cached_start| {
            let name = format!("{name}_{suffix}");
            let cents = LazyRollingSumFromHeight::new(
                &format!("{name}_cents"),
                version,
                cumulative_cents.clone(),
                cached_start,
                indexes,
            );

            let usd = LazyPerBlock::from_resolutions::<C::ToDollars>(
                &name,
                version,
                cents.height.read_only_boxed_clone(),
                &cents.resolutions,
            );

            LazyRollingSumFiatFromHeight { usd, cents }
        }))
    }
}
