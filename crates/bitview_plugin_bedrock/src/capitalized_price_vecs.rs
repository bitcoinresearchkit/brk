use bitview_cohort::{UTXOAggregate, UTXOAggregateId};
use bitview_traversable::Traversable;
use bitview_vecs::{
    ColumnarDailyMetric, DailyMappings, IndexSources, LazyColumnDailyPriceWithRatio,
};
use brk_error::Result;
use brk_types::{Cents, Height, Version};
use vecdb::{AnyStoredVec, CacheBudget, CachedBoxedVec, Database, Rw, StorageMode};

use crate::WeightedPair;

#[derive(Traversable)]
pub struct CapitalizedPriceVecs<M: StorageMode = Rw> {
    /// Approximate capitalized prices from daily wakefulness-weighted URPDs:
    /// sum(bucket price squared * weighted sats) / sum(bucket price * weighted sats).
    /// Uses rounded URPD buckets, not exact per-output second moments. Empty or
    /// zero-capitalization distributions are undefined. Final cents are floored.
    pub awake: ColumnarDailyMetric<
        Cents,
        UTXOAggregateId,
        UTXOAggregate<LazyColumnDailyPriceWithRatio<UTXOAggregateId>>,
        M,
    >,
    /// The same capital-weighted mean using daily mobility-weighted URPDs.
    pub coinflow: ColumnarDailyMetric<
        Cents,
        UTXOAggregateId,
        UTXOAggregate<LazyColumnDailyPriceWithRatio<UTXOAggregateId>>,
        M,
    >,
}

impl CapitalizedPriceVecs {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        indexes: &IndexSources,
        mappings: &DailyMappings,
        spot: &CachedBoxedVec<Height, Cents>,
    ) -> Result<Self> {
        let import = |weight| {
            ColumnarDailyMetric::forced_import(
                db,
                &format!("{weight}_capitalized_price_cents_by_aggregate"),
                version + Version::ONE,
                |source| {
                    let cohort = |cohort: UTXOAggregateId| {
                        LazyColumnDailyPriceWithRatio::new(
                            cache,
                            &cohort.metric_name(&format!("{weight}_capitalized_price")),
                            version + Version::ONE,
                            source,
                            cohort,
                            indexes,
                            mappings,
                            spot,
                        )
                    };
                    UTXOAggregate {
                        all: cohort(UTXOAggregateId::All),
                        sth: cohort(UTXOAggregateId::Sth),
                        lth: cohort(UTXOAggregateId::Lth),
                    }
                },
            )
        };
        Ok(Self {
            awake: import("awake")?,
            coinflow: import("coinflow")?,
        })
    }

    pub fn push(&mut self, prices: &UTXOAggregate<WeightedPair<Cents>>) {
        self.awake.push(prices.map(|price| price.cointime));
        self.coinflow.push(prices.map(|price| price.coinflow));
    }

    pub fn stored_vecs_mut(&mut self) -> impl Iterator<Item = &mut dyn AnyStoredVec> {
        [self.awake.stored_mut(), self.coinflow.stored_mut()].into_iter()
    }

    pub fn minimum_len(&mut self) -> usize {
        self.stored_vecs_mut()
            .map(|vec| vec.len())
            .min()
            .unwrap_or_default()
    }
}
