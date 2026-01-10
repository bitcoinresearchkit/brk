use brk_types::Version;
use vecdb::IterableCloneableVec;

use super::Vecs;
use crate::{
    distribution,
    internal::{DollarsIdentity, LazyFromHeightLast},
};

impl Vecs {
    pub fn import(version: Version, distribution: &distribution::Vecs) -> Option<Self> {
        let supply_metrics = &distribution.utxo_cohorts.all.metrics.supply;

        supply_metrics.total.dollars.as_ref().map(|d| {
            Self(LazyFromHeightLast::from_lazy_binary_computed::<DollarsIdentity, _, _>(
                "market_cap",
                version,
                d.height.boxed_clone(),
                d,
            ))
        })
    }
}
