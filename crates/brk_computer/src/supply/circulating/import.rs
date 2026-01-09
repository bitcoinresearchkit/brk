use brk_types::Version;

use super::Vecs;
use crate::{
    distribution,
    internal::{DollarsIdentity, LazyLastBlockValue, SatsIdentity},
};

impl Vecs {
    pub fn import(version: Version, distribution: &distribution::Vecs) -> Self {
        let supply_metrics = &distribution.utxo_cohorts.all.metrics.supply;

        Self(LazyLastBlockValue::from_block_source::<
            SatsIdentity,
            DollarsIdentity,
        >("circulating_supply", &supply_metrics.supply, version))
    }
}
