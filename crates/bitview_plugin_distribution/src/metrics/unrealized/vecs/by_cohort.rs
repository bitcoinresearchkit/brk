use std::ops::AddAssign;

use bitview_cohort::{CohortContext, UTXOGroupsWithoutAmount};
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::{FiatType, LazyFiatPerBlock};
use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, PcoVecValue, Rw, StorageMode};

use crate::metrics::UTXOTypedSources;

#[derive(Traversable)]
pub struct UnrealizedByCohort<C, M: StorageMode = Rw>
where
    C: FiatType + PcoVecValue,
{
    #[traversable(flatten)]
    pub cohorts: UTXOGroupsWithoutAmount<LazyFiatPerBlock<C>>,
    #[traversable(hidden)]
    pub stored: UTXOTypedSources<C, M>,
}

impl<C> UnrealizedByCohort<C>
where
    C: FiatType + PcoVecValue + AddAssign,
{
    pub fn forced_import(
        db: &Database,
        metric: &str,
        version: Version,
        mappings: &MappingsVecs,
    ) -> Result<Self> {
        let stored = UTXOTypedSources::forced_import(db, &format!("{metric}_cents"), version)?;
        let cohorts = UTXOGroupsWithoutAmount::new(|cohort_id| {
            let name = CohortContext::Utxo.metric_name(cohort_id, metric);
            let source = stored.get(cohort_id).expect("supported unrealized cohort");
            LazyFiatPerBlock::from_cents_source(&name, version, source, mappings)
        });
        Ok(Self { cohorts, stored })
    }
}
