use brk_types::Version;

use super::Vecs;
use crate::{
    distribution,
    internal::{DollarsIdentity, LazyValueFromHeightLast, SatsIdentity},
};

impl Vecs {
    pub fn import(version: Version, distribution: &distribution::Vecs) -> Self {
        let supply_metrics = &distribution.utxo_cohorts.all.metrics.supply;

        Self(LazyValueFromHeightLast::from_block_source::<
            SatsIdentity,
            DollarsIdentity,
        >(
            "circulating_supply", &supply_metrics.total, version)
        )
    }
}
