use bitview_cohort::{AgeRangeId, CohortContext};
use bitview_collections::Windows;
use bitview_plugin_distribution::Vecs as DistributionVecs;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_transforms::{BoundedOddsF64, BoundedToF64};
use bitview_vecs::{
    CachedWindowStartVec, ColumnarPerBlock, ColumnarPerBlockCumulativeRolling,
    LazyColumnPerBlockCumulativeRolling, LazyPerBlock, LazySpotValuePerBlock,
};
use brk_error::Result;
use brk_types::{BoundedRatio, Cents, Height, Version};
use vecdb::{
    CacheBudget, CachedBoxedVec, Database, PcoVec, ReadOnlyClone, ReadOnlyColumnarVec,
    ReadableColumnarVec,
};

use super::{ActivitySeries, SupplyVecs, Vecs};

const VERSION: Version = Version::new(3);

pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    parent_version: Version,
    mappings: &MappingsVecs,
    cached_starts: &Windows<&CachedWindowStartVec>,
    spot_price: &CachedBoxedVec<Height, Cents>,
    distribution: &DistributionVecs,
) -> Result<Vecs> {
    let version = parent_version + VERSION;
    let coindays_consumed = ColumnarPerBlockCumulativeRolling::forced_import(
        cache,
        db,
        &CohortContext::Utxo.prefixed("age_range_coindays_consumed_cumulative"),
        version,
        |source| {
            AgeRangeId::series(CohortContext::Utxo, |column, name| {
                LazyColumnPerBlockCumulativeRolling::new(
                    &format!("{name}_coindays_consumed"),
                    version,
                    source,
                    column,
                    mappings,
                    cached_starts,
                )
            })
        },
    )?;
    let coindays_stored = ColumnarPerBlockCumulativeRolling::forced_import(
        cache,
        db,
        &CohortContext::Utxo.prefixed("age_range_coindays_stored_cumulative"),
        version,
        |source| {
            AgeRangeId::series(CohortContext::Utxo, |column, name| {
                LazyColumnPerBlockCumulativeRolling::new(
                    &format!("{name}_coindays_stored"),
                    version,
                    source,
                    column,
                    mappings,
                    cached_starts,
                )
            })
        },
    )?;
    let activity = ColumnarPerBlock::forced_import(
        cache,
        db,
        &CohortContext::Utxo.prefixed("age_range_wakefulness_bounded_source"),
        version + Version::ONE,
        |source| ActivitySeries::new(version + Version::ONE, source, mappings),
    )?;
    let import_supply = |side: &str, complement: bool| {
        AgeRangeId::series(CohortContext::Utxo, |column, name| {
            let name = format!("{name}_{side}_supply");
            let supply = distribution
                .cohorts
                .supply
                .total
                .stored
                .age_range
                .height
                .read_only_clone()
                .column(&name, version, column);
            let weight = activity
                .height
                .read_only_clone()
                .column(&name, version, column);
            if complement {
                LazySpotValuePerBlock::from_weighted_supply::<true>(
                    &name, version, &supply, &weight, mappings, spot_price,
                )
            } else {
                LazySpotValuePerBlock::from_weighted_supply::<false>(
                    &name, version, &supply, &weight, mappings, spot_price,
                )
            }
        })
    };
    let supply = SupplyVecs {
        awake: import_supply("awake", false),
        dormant: import_supply("dormant", true),
    };

    Ok(Vecs {
        coindays_consumed,
        coindays_stored,
        activity,
        supply,
    })
}

impl ActivitySeries {
    fn new(
        version: Version,
        source: &ReadOnlyColumnarVec<PcoVec<Height, BoundedRatio>, AgeRangeId>,
        mappings: &MappingsVecs,
    ) -> Self {
        let wakefulness = AgeRangeId::series(CohortContext::Utxo, |column, name| {
            LazyPerBlock::from_height_source::<BoundedToF64>(
                &format!("{name}_wakefulness"),
                version,
                &source.column(&format!("{name}_wakefulness_source"), version, column),
                mappings,
            )
        });
        let dormancy = AgeRangeId::series(CohortContext::Utxo, |column, name| {
            LazyPerBlock::from_height_source::<BoundedToF64<true>>(
                &format!("{name}_dormancy"),
                version,
                &source.column(&format!("{name}_dormancy_source"), version, column),
                mappings,
            )
        });
        let wakefulness_to_dormancy = AgeRangeId::series(CohortContext::Utxo, |column, name| {
            LazyPerBlock::from_height_source::<BoundedOddsF64>(
                &format!("{name}_wakefulness_to_dormancy"),
                version + Version::ONE,
                &source.column(
                    &format!("{name}_wakefulness_to_dormancy_source"),
                    version,
                    column,
                ),
                mappings,
            )
        });

        Self {
            wakefulness,
            dormancy,
            wakefulness_to_dormancy,
        }
    }
}
