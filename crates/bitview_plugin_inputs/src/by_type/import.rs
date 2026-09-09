use bitview_cohort::SpendableType;
use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{CachedWindowStartVec, CountTotal, import_stored};
use brk_error::Result;
use brk_types::{Height, StoredU64, Version};
use vecdb::{CacheBudget, Database};

use super::{Vecs, WithInputTypes};

fn without_coinbase(height: Height, total: StoredU64) -> StoredU64 {
    total - StoredU64::from(height.incremented())
}

impl Vecs {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
    ) -> Result<Self> {
        let version = version + Version::TWO;
        let input_count_stored = SpendableType::try_new(|_, name| {
            import_stored(cache, db, &format!("{name}_prevout_count"), version)
        })?;
        let input_count = WithInputTypes::from_count_sources(
            CountTotal::from_source(
                "input_count_bis",
                version,
                &mappings.input_count_source(),
                mappings,
                cached_starts,
            ),
            |name| format!("{name}_prevout_count"),
            version,
            &input_count_stored,
            mappings,
            cached_starts,
        );
        let input_share = input_count.lazy_shares(
            version,
            |name| format!("{name}_prevout_share"),
            cached_starts,
            mappings,
        );
        let tx_count_stored = SpendableType::try_new(|_, name| {
            import_stored(
                cache,
                db,
                &format!("tx_count_with_{name}_prevout_cumulative"),
                version,
            )
        })?;
        let tx_count = WithInputTypes::from_cumulative_sources(
            CountTotal::from_transformed_source(
                "non_coinbase_tx_count",
                version,
                &mappings.transaction_count_source(),
                without_coinbase,
                mappings,
                cached_starts,
            ),
            |name| format!("tx_count_with_{name}_prevout"),
            version,
            &tx_count_stored,
            mappings,
            cached_starts,
        );
        let tx_share = tx_count.lazy_shares(
            version,
            |name| format!("tx_share_with_{name}_prevout"),
            cached_starts,
            mappings,
        );

        Ok(Self {
            input_count,
            input_share,
            tx_count,
            tx_share,
            input_count_stored,
            tx_count_stored,
        })
    }
}
