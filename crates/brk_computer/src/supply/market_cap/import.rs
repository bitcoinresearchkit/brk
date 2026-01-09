use brk_types::Version;
use vecdb::IterableCloneableVec;

use super::Vecs;
use crate::{
    distribution,
    internal::{DollarsIdentity, LazyBlockLast},
};

impl Vecs {
    pub fn import(version: Version, distribution: &distribution::Vecs) -> Option<Self> {
        let supply_metrics = &distribution.utxo_cohorts.all.metrics.supply;

        supply_metrics.total.dollars.as_ref().map(|d| {
            Self(LazyBlockLast::from_computed::<DollarsIdentity>(
                "market_cap",
                version,
                d.height.boxed_clone(),
                d,
            ))
        })
    }
}
