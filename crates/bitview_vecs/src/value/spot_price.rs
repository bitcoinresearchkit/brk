use bitview_transforms::{CentsUnsignedToDollars, CentsUnsignedToSats};
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Cents, Dollars, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{CacheBudget, Database, Pinned, Rw, StorageMode};

use crate::{IndexSources, LazyPerBlock, PerBlock, Price};

/// Pinned cents with integer sats per USD, preserving the spot-price conversion.
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct SpotPrice<M: StorageMode = Rw>(
    pub Price<PerBlock<Cents, M, Pinned>, LazyPerBlock<Dollars, Cents>, LazyPerBlock<Sats, Cents>>,
);

impl SpotPrice {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
    ) -> Result<Self> {
        let cents = PerBlock::forced_import(cache, db, &format!("{name}_cents"), version, indexes)?;
        let usd = LazyPerBlock::from_resolutions::<CentsUnsignedToDollars>(name, version, &cents);
        let sats = LazyPerBlock::from_resolutions::<CentsUnsignedToSats>(
            &format!("{name}_sats"),
            version,
            &cents,
        );
        Ok(Self(Price { usd, cents, sats }))
    }
}
