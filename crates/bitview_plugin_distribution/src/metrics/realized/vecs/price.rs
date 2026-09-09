use bitview_cohort::{CohortContext, UTXOGroups};
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::LazyPriceWithRatioPerBlock;
use brk_error::Result;
use brk_types::{Cents, Height, Version};
use vecdb::{CacheBudget, CachedBoxedVec, Database, Rw, StorageMode};

use crate::metrics::ExactUTXOSources;

#[derive(Traversable)]
pub struct RealizedPriceByCohort<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub cohorts: UTXOGroups<LazyPriceWithRatioPerBlock>,
    /// Reported in cents per BTC.
    #[traversable(hidden)]
    pub stored: ExactUTXOSources<Cents, M>,
}

impl RealizedPriceByCohort {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        spot_price: &CachedBoxedVec<Height, Cents>,
    ) -> Result<Self> {
        let version = version + Version::ONE;
        let stored = ExactUTXOSources::forced_import(cache, db, "realized_price_cents", version)?;
        let cohorts = UTXOGroups::new(|filter, cohort_name| {
            let name = CohortContext::Utxo.metric_name(&filter, cohort_name, "realized_price");
            LazyPriceWithRatioPerBlock::from_height_source(
                &name,
                version,
                stored.get(&filter).expect("realized-price cohort source"),
                mappings,
                spot_price,
            )
        });
        Ok(Self { cohorts, stored })
    }
}
