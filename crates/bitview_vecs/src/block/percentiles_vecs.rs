use bitview_collections::ByPercentile;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Cents, PERCENTILES_LEN, PercentileId, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use crate::{ColumnarPerBlock, IndexSources, LazyColumnPerBlock, Price};

#[derive(Deref, DerefMut, Traversable)]
pub struct PercentilesVecs<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub prices: ColumnarPerBlock<
        Cents,
        PercentileId,
        ByPercentile<Price<LazyColumnPerBlock<Cents, PercentileId>>>,
        M,
    >,
}

const VERSION: Version = Version::ONE;

impl PercentilesVecs {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        prefix: &str,
        version: Version,
        indexes: &IndexSources,
    ) -> Result<Self> {
        let version = version + VERSION;
        let prices = ColumnarPerBlock::<Cents, PercentileId, _>::forced_import(
            db,
            &format!("{prefix}_cents"),
            version,
            |source| {
                ByPercentile::from_fn(|id| {
                    Price::from_columnar_source(
                        cache,
                        &format!("{prefix}_pct{:02}", id.percentile()),
                        version,
                        source,
                        id,
                        indexes,
                    )
                })
            },
        )?;

        Ok(Self { prices })
    }

    /// Push percentile prices (in cents).
    #[inline(always)]
    pub fn push(&mut self, percentile_prices: &[Cents; PERCENTILES_LEN]) {
        self.prices.push(*percentile_prices);
    }

    /// Validate computed versions or reset if mismatched.
    pub fn validate_computed_version_or_reset(&mut self, version: Version) -> Result<()> {
        self.prices.validate_computed_version_or_reset(version)?;
        Ok(())
    }
}
