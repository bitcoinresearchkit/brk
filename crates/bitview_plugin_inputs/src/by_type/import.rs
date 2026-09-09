use bitview_cohort::SpendableTypeId;
use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{
    CachedWindowStartVec, ColumnarPerBlock, ColumnarPerBlockCumulativeRolling, CountTotal,
};
use brk_error::Result;
use brk_types::{Height, StoredU16, StoredU64, Version};
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
        let columnar_version = version + Version::ONE;
        let all_input_count = mappings.input_count_source();
        let input_count = ColumnarPerBlock::<StoredU16, SpendableTypeId, _>::forced_import(
            cache,
            db,
            "prevout_count_by_type",
            columnar_version,
            |source| {
                WithInputTypes::from_columnar_count_source(
                    CountTotal::from_source(
                        "input_count_bis",
                        columnar_version,
                        &all_input_count,
                        mappings,
                        cached_starts,
                    ),
                    |t| format!("{t}_prevout_count"),
                    columnar_version,
                    source,
                    mappings,
                    cached_starts,
                )
            },
        )?;
        let input_share = input_count.lazy_shares(
            columnar_version,
            |name| format!("{name}_prevout_share"),
            cached_starts,
            mappings,
        );
        let transaction_count_source = mappings.transaction_count_source();
        let tx_count =
            ColumnarPerBlockCumulativeRolling::<StoredU64, SpendableTypeId, _>::forced_import(
                cache,
                db,
                "tx_count_with_prevout_by_type_cumulative",
                columnar_version,
                |source| {
                    WithInputTypes::from_columnar_source(
                        CountTotal::from_transformed_source(
                            "non_coinbase_tx_count",
                            columnar_version,
                            &transaction_count_source,
                            without_coinbase,
                            mappings,
                            cached_starts,
                        ),
                        |t| format!("tx_count_with_{t}_prevout"),
                        columnar_version,
                        source,
                        mappings,
                        cached_starts,
                    )
                },
            )?;
        let tx_share = tx_count.lazy_shares(
            columnar_version,
            |name| format!("tx_share_with_{name}_prevout"),
            cached_starts,
            mappings,
        );
        Ok(Self {
            input_count,
            input_share,
            tx_count,
            tx_share,
        })
    }
}
