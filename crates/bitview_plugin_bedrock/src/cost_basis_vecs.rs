use bitview_traversable::Traversable;
use bitview_vecs::DailyMappings;
use brk_error::Result;
use brk_types::{CostBasisByPercentile, Version};
use vecdb::{AnyStoredVec, CacheBudget, Database, Rw, StorageMode};

use crate::{DailyPercentilesVecs, WeightedPair};

#[derive(Traversable)]
pub struct CostBasisVecs<M: StorageMode = Rw> {
    pub per_coin: WeightedPair<DailyPercentilesVecs<M>>,
    pub per_dollar: WeightedPair<DailyPercentilesVecs<M>>,
}

impl CostBasisVecs {
    fn import_weighting(
        cache: &'static CacheBudget,
        db: &Database,
        weighting: &str,
        version: Version,
        mappings: &DailyMappings,
    ) -> Result<WeightedPair<DailyPercentilesVecs>> {
        WeightedPair::try_from_fn(|weight| {
            DailyPercentilesVecs::forced_import(
                cache,
                db,
                &format!("bedrock_{}_cost_basis_{weighting}", weight.as_str()),
                version,
                mappings,
            )
        })
    }

    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &DailyMappings,
    ) -> Result<Self> {
        Ok(Self {
            per_coin: Self::import_weighting(cache, db, "per_coin", version, mappings)?,
            per_dollar: Self::import_weighting(cache, db, "per_dollar", version, mappings)?,
        })
    }

    pub fn push(&mut self, prices: &WeightedPair<CostBasisByPercentile>) {
        self.per_coin.cointime.push(&prices.cointime.per_coin);
        self.per_coin.coinflow.push(&prices.coinflow.per_coin);
        self.per_dollar.cointime.push(&prices.cointime.per_dollar);
        self.per_dollar.coinflow.push(&prices.coinflow.per_dollar);
    }

    pub fn stored_vecs_mut(&mut self) -> impl Iterator<Item = &mut dyn AnyStoredVec> {
        self.per_coin
            .iter_mut()
            .chain(self.per_dollar.iter_mut())
            .flat_map(DailyPercentilesVecs::collect_vecs_mut)
    }

    pub fn minimum_len(&mut self) -> usize {
        self.stored_vecs_mut()
            .map(|vec| vec.len())
            .min()
            .unwrap_or_default()
    }
}
