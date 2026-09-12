use bitview_cohort::UTXOAggregate;
use bitview_traversable::Traversable;
use bitview_vecs::{
    CachedSeries, DailyMappings, IndexSources, LazyDailyPriceWithRatio, import_cached,
};
use brk_error::Result;
use brk_types::{Cents, Day1, Height, Version};
use vecdb::{AnyStoredVec, Database, ReadableBoxedVec, Rw, StorageMode, WritableVec};

use crate::WeightedPair;

#[derive(Traversable)]
pub struct CapitalizedPriceVecs<M: StorageMode = Rw> {
    /// Approximate capitalized prices from daily wakefulness-weighted URPDs:
    /// sum(bucket price squared * weighted sats) / sum(bucket price * weighted sats).
    /// Uses rounded URPD buckets, not exact per-output second moments. Empty or
    /// zero-capitalization distributions are undefined. Final cents are floored.
    pub awake: UTXOAggregate<LazyDailyPriceWithRatio>,
    /// The same capital-weighted mean using daily mobility-weighted URPDs.
    pub coinflow: UTXOAggregate<LazyDailyPriceWithRatio>,
    #[traversable(hidden)]
    pub stored: WeightedPair<UTXOAggregate<CachedSeries<Day1, Cents, M>>>,
}

impl CapitalizedPriceVecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &IndexSources,
        mappings: &DailyMappings,
        spot: &ReadableBoxedVec<Height, Cents>,
    ) -> Result<Self> {
        let version = version + Version::TWO;
        let import = |weight: &str| {
            UTXOAggregate::try_from_fn(|id| {
                import_cached(
                    db,
                    &id.metric_name(&format!("{weight}_capitalized_price_cents")),
                    version,
                )
            })
        };
        let stored = WeightedPair {
            cointime: import("awake")?,
            coinflow: import("coinflow")?,
        };
        let build = |weight: &str, sources: &UTXOAggregate<CachedSeries<Day1, Cents>>| {
            UTXOAggregate::from_fn(|id| {
                LazyDailyPriceWithRatio::from_day1_source(
                    &id.metric_name(&format!("{weight}_capitalized_price")),
                    version,
                    id.select(sources),
                    indexes,
                    mappings,
                    spot,
                )
            })
        };
        let awake = build("awake", &stored.cointime);
        let coinflow = build("coinflow", &stored.coinflow);
        Ok(Self {
            awake,
            coinflow,
            stored,
        })
    }

    pub fn push(&mut self, prices: &UTXOAggregate<WeightedPair<Cents>>) {
        for (target, price) in self.stored.cointime.iter_mut().zip(prices.iter()) {
            target.push(price.cointime);
        }
        for (target, price) in self.stored.coinflow.iter_mut().zip(prices.iter()) {
            target.push(price.coinflow);
        }
    }

    pub fn stored_vecs_mut(&mut self) -> impl Iterator<Item = &mut dyn AnyStoredVec> {
        self.stored
            .iter_mut()
            .flat_map(|sources| sources.iter_mut())
            .map(|v| v as &mut dyn AnyStoredVec)
    }

    pub fn minimum_len(&mut self) -> usize {
        self.stored_vecs_mut()
            .map(|vec| vec.len())
            .min()
            .unwrap_or_default()
    }
}
