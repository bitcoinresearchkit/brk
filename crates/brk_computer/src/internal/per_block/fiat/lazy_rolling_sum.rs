use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{DeltaSub, LazyDeltaVec, LazyVecFrom1, ReadOnlyClone, ReadableCloneableVec};

use crate::{
    indexes,
    internal::{
        CentsType, DerivedResolutions, LazyPerBlock, LazyRollingSumFromHeight, Resolutions,
        WindowStartVec, Windows,
    },
};

#[derive(Clone, Traversable)]
pub struct LazyRollingSumFiatFromHeight<C: CentsType> {
    pub usd: LazyPerBlock<Dollars, C>,
    pub cents: LazyRollingSumFromHeight<C>,
}

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct LazyRollingSumsFiatFromHeight<C: CentsType>(
    pub Windows<LazyRollingSumFiatFromHeight<C>>,
);

impl<C: CentsType> LazyRollingSumsFiatFromHeight<C> {
    pub fn new(
        name: &str,
        version: Version,
        cumulative_cents: &(impl ReadableCloneableVec<Height, C> + 'static),
        cached_starts: &Windows<&WindowStartVec>,
        indexes: &indexes::Vecs,
    ) -> Self {
        let cum_cents = cumulative_cents.read_only_boxed_clone();

        let make_slot = |suffix: &str, cached_start: &&WindowStartVec| {
            let full_name = format!("{name}_{suffix}");
            let cached = cached_start.read_only_clone();
            let starts_version = cached.version();

            let cents_sum = LazyDeltaVec::<Height, C, C, DeltaSub>::new(
                &format!("{full_name}_cents"),
                version,
                cum_cents.clone(),
                starts_version,
                move || cached.cached(),
            );
            let cents_resolutions = Resolutions::forced_import(
                &format!("{full_name}_cents"),
                cents_sum.clone(),
                version,
                indexes,
            );
            let cents = LazyRollingSumFromHeight {
                height: cents_sum,
                resolutions: Box::new(cents_resolutions),
            };

            let usd = LazyPerBlock {
                height: LazyVecFrom1::transformed::<C::ToDollars>(
                    &full_name,
                    version,
                    cents.height.read_only_boxed_clone(),
                ),
                resolutions: Box::new(DerivedResolutions::from_derived_computed::<C::ToDollars>(
                    &full_name,
                    version,
                    &cents.resolutions,
                )),
            };

            LazyRollingSumFiatFromHeight { usd, cents }
        };

        Self(cached_starts.map_with_suffix(make_slot))
    }
}
