use std::collections::BTreeMap;

use rustc_hash::FxHashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Bitcoin, Cents, Cohort, Date, Dollars, Sats, UrpdAggregation, UrpdBucket, UrpdRaw};

/// UTXO Realized Price Distribution for a cohort on a specific date.
///
/// Supply is grouped by the close price at which each UTXO was last moved.
/// Each bucket exposes three values derived from the same `(price_floor, supply)`
/// pairs: supply in BTC, realized cap contribution in USD (`price_floor * supply`),
/// and unrealized P&L against that date's close in USD
/// (`(close - price_floor) * supply`, can be negative).
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Urpd {
    pub cohort: Cohort,
    pub date: Date,
    /// Aggregation strategy applied to the buckets.
    pub aggregation: UrpdAggregation,
    /// Close price on `date`, in USD. Anchor for `unrealized_pnl`.
    pub close: Dollars,
    /// Sum of `supply` across all buckets, in BTC.
    pub total_supply: Bitcoin,
    pub buckets: Vec<UrpdBucket>,
}

impl Urpd {
    /// Build from the raw on-disk distribution plus context.
    pub fn build(
        cohort: Cohort,
        date: Date,
        close_cents: Cents,
        raw: &UrpdRaw,
        aggregation: UrpdAggregation,
    ) -> Self {
        let mut agg: FxHashMap<Cents, Sats> =
            FxHashMap::with_capacity_and_hasher(raw.map.len(), Default::default());
        for (&price_cents, &sats) in &raw.map {
            let price = Cents::from(price_cents);
            let key = match aggregation {
                UrpdAggregation::Raw => price,
                _ => aggregation.bucket_floor(price).unwrap_or(price),
            };
            *agg.entry(key).or_insert(Sats::ZERO) += sats;
        }

        let sorted: BTreeMap<Cents, Sats> = agg.into_iter().collect();
        let close = Dollars::from(close_cents);

        let mut total_sats = Sats::ZERO;
        let mut buckets = Vec::with_capacity(sorted.len());
        for (price_floor_cents, supply) in sorted {
            total_sats += supply;
            let price_floor = Dollars::from(price_floor_cents);
            buckets.push(UrpdBucket {
                price_floor,
                supply: Bitcoin::from(supply),
                realized_cap: price_floor * supply,
                unrealized_pnl: (close - price_floor) * supply,
            });
        }

        Self {
            cohort,
            date,
            aggregation,
            close,
            total_supply: Bitcoin::from(total_sats),
            buckets,
        }
    }
}
