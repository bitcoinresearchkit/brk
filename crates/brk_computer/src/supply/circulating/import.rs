use brk_types::Version;

use super::Vecs;
use crate::{
    distribution,
    internal::{DollarsIdentity, LazyValueBlockLast, SatsIdentity},
};

impl Vecs {
    pub fn import(version: Version, distribution: &distribution::Vecs) -> Self {
        let supply_metrics = &distribution.utxo_cohorts.all.metrics.supply;

        Self(LazyValueBlockLast::from_block_source::<
            SatsIdentity,
            DollarsIdentity,
        >(
            "circulating_supply", &supply_metrics.total, version)
        )
    }
}
