use bitview_collections::ByPercentile;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_types::{Cents, Day1, PERCENTILES_LEN, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, CacheBudget, Database, Rw, StorageMode, WritableVec};

use crate::{DailyMappings, LazyDailyPrice, StoredSeries, import_stored};

#[derive(Deref, DerefMut, Traversable)]
pub struct DailyPercentilesVecs<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub prices: ByPercentile<LazyDailyPrice>,
    #[traversable(hidden)]
    pub stored: ByPercentile<StoredSeries<Day1, Cents, M>>,
}

impl DailyPercentilesVecs {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        mappings: &DailyMappings,
    ) -> Result<Self> {
        let version = version + Version::TWO;
        let stored = ByPercentile::try_from_fn(|id| {
            import_stored(
                cache,
                db,
                &format!("{name}_pct{:02}_cents", id.percentile()),
                version,
            )
        })?;
        let prices = ByPercentile::from_fn(|id| {
            LazyDailyPrice::from_day1_source(
                &format!("{name}_pct{:02}", id.percentile()),
                version,
                stored.select(id),
                mappings,
            )
        });
        Ok(Self { prices, stored })
    }

    pub fn push(&mut self, prices: &[Cents; PERCENTILES_LEN]) {
        for (target, &price) in self.stored.iter_mut().zip(prices) {
            target.push(price);
        }
    }

    pub fn min_len(&self) -> usize {
        self.stored.iter().map(AnyVec::len).min().unwrap_or(0)
    }
    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.stored
            .iter_mut()
            .map(|v| v as &mut dyn AnyStoredVec)
            .collect()
    }
}
