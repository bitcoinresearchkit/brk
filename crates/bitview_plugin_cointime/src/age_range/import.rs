use brk_error::Result;

use bitview_cohort::{AgeRangeId, CohortContext};
use brk_types::{BoundedRatio, Cents, Height, Version};
use vecdb::{
    CachedBoxedVec, CachedColumnarVec, CachedReadableVec, Database, PcoVec, ReadOnlyColumnarVec,
    ReadableCloneableVec,
};

use super::{ActivitySeries, SupplyVecs, Vecs};
use bitview_compute::{
    BoundedOddsF64, BoundedToF64, CACHE_BUDGET, CachedWindowStartVec, ColumnarPerBlock,
    ColumnarPerBlockCumulativeRolling, LazyColumnPerBlockCumulativeRolling, LazyPerBlock, Windows,
    lazy_weighted_supply,
};

const VERSION: Version = Version::new(3);

pub fn forced_import(
    db: &Database,
    parent_version: Version,
    mappings: &bitview_plugin_mappings::Vecs,
    cached_starts: &Windows<&CachedWindowStartVec>,
    spot_price: &CachedBoxedVec<Height, Cents>,
    distribution: &bitview_plugin_distribution::Vecs,
) -> Result<Vecs> {
    let version = parent_version + VERSION;
    let coindays_consumed = ColumnarPerBlockCumulativeRolling::forced_import(
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
                .age_ranges
                .cached_column(column)
                .read_only_boxed_clone();
            let weight = activity.cached.cached_column(column).cached_boxed_clone();
            if complement {
                lazy_weighted_supply::<true>(&name, version, supply, weight, mappings, spot_price)
            } else {
                lazy_weighted_supply::<false>(&name, version, supply, weight, mappings, spot_price)
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
        mappings: &bitview_plugin_mappings::Vecs,
    ) -> Self {
        let cached =
            CachedColumnarVec::new(source.clone(), version, |column| CACHE_BUDGET.wrap(column));
        let wakefulness = AgeRangeId::series(CohortContext::Utxo, |column, name| {
            LazyPerBlock::from_height_source::<BoundedToF64>(
                &format!("{name}_wakefulness"),
                version,
                cached.cached_column(column).clone(),
                mappings,
            )
        });
        let dormancy = AgeRangeId::series(CohortContext::Utxo, |column, name| {
            LazyPerBlock::from_height_source::<BoundedToF64<true>>(
                &format!("{name}_dormancy"),
                version,
                cached.cached_column(column).clone(),
                mappings,
            )
        });
        let wakefulness_to_dormancy = AgeRangeId::series(CohortContext::Utxo, |column, name| {
            LazyPerBlock::from_height_source::<BoundedOddsF64>(
                &format!("{name}_wakefulness_to_dormancy"),
                version + Version::ONE,
                cached.cached_column(column).clone(),
                mappings,
            )
        });

        Self {
            cached,
            wakefulness,
            dormancy,
            wakefulness_to_dormancy,
        }
    }
}
