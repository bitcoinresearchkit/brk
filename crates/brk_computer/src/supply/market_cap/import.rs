use brk_types::Version;
use vecdb::{IterableCloneableVec, LazyVecFrom1};

use super::Vecs;
use crate::{
    distribution,
    internal::{DollarsIdentity, LazyVecsFromDateIndex},
};

impl Vecs {
    pub fn import(version: Version, distribution: &distribution::Vecs) -> Self {
        let v0 = Version::ZERO;
        let supply_metrics = &distribution.utxo_cohorts.all.metrics.supply;

        // Market cap by height (lazy from distribution's supply in USD)
        let height = supply_metrics
            .height_to_supply_value
            .dollars
            .as_ref()
            .map(|d| {
                LazyVecFrom1::init(
                    "market_cap",
                    version + v0,
                    d.boxed_clone(),
                    |height, iter| iter.get(height),
                )
            });

        // Market cap by indexes (lazy from distribution's supply in USD)
        let indexes = supply_metrics.indexes_to_supply.dollars.as_ref().map(|d| {
            LazyVecsFromDateIndex::from_computed::<DollarsIdentity>(
                "market_cap",
                version + v0,
                d.dateindex.as_ref().map(|v| v.boxed_clone()),
                d,
            )
        });

        Self { height, indexes }
    }
}
