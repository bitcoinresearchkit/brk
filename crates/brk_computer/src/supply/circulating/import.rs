use brk_types::Version;
use vecdb::{IterableCloneableVec, LazyVecFrom1};

use super::Vecs;
use crate::{
    distribution,
    internal::{DollarsIdentity, LazyValueVecsFromDateIndex, SatsIdentity, SatsToBitcoin},
};

impl Vecs {
    pub fn import(version: Version, distribution: &distribution::Vecs) -> Self {
        let v0 = Version::ZERO;

        // Reference distribution's actual circulating supply lazily
        let supply_metrics = &distribution.utxo_cohorts.all.metrics.supply;

        let height_to_sats = LazyVecFrom1::init(
            "circulating_sats",
            version + v0,
            supply_metrics.height_to_supply.boxed_clone(),
            |height, iter| iter.get(height),
        );

        let height_to_btc = LazyVecFrom1::transformed::<SatsToBitcoin>(
            "circulating_btc",
            version + v0,
            supply_metrics.height_to_supply.boxed_clone(),
        );

        let height_to_usd = supply_metrics
            .height_to_supply_value
            .dollars
            .as_ref()
            .map(|d| {
                LazyVecFrom1::init(
                    "circulating_usd",
                    version + v0,
                    d.boxed_clone(),
                    |height, iter| iter.get(height),
                )
            });

        // Create lazy identity wrapper around the FULL supply (not half!)
        let indexes = LazyValueVecsFromDateIndex::from_source::<
            SatsIdentity,
            SatsToBitcoin,
            DollarsIdentity,
        >("circulating", &supply_metrics.indexes_to_supply, version + v0);

        Self {
            height_to_sats,
            height_to_btc,
            height_to_usd,
            indexes,
        }
    }
}
