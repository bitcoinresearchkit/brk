use bitview_cohort::{
    AgeRange, AgeRangeId, Filter, OVER_AGE_FILTERS, TERM_FILTERS, TimeFilter, UNDER_AGE_FILTERS,
    UTXORows,
};
use brk_types::{Cents, Height, Sats, Version};
use vecdb::{Database, ReadableVec};

use super::{CumulativeUTXOCoreValueColumns, CumulativeUTXOValueColumns, UTXOCoreColumns};

#[test]
fn additive_and_cumulative_sources_share_exact_aggregate_selection() {
    static SOURCE_CACHE_BUDGET: vecdb::CacheBudget = vecdb::CacheBudget::new(64 * 1024 * 1024);

    let directory = tempfile::tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut additive =
        UTXOCoreColumns::<Sats>::forced_import(&db, "additive", Version::ONE).unwrap();
    let mut cumulative =
        CumulativeUTXOCoreValueColumns::forced_import(&db, "cumulative", Version::ONE).unwrap();
    let mut full = CumulativeUTXOValueColumns::forced_import(&db, "full", Version::ONE).unwrap();
    let sats = UTXORows {
        core: bitview_cohort::UTXOCoreRows {
            age_range: AgeRange::from_fn(|id| Sats::from(id.index() as u64 + 1)),
            ..Default::default()
        },
        ..Default::default()
    };
    let cents = sats.map(|value| Cents::from(u64::from(*value) * 10));
    for _ in 0..2 {
        additive.push(sats.clone());
        cumulative.push_block(sats.clone(), cents.clone());
        full.push_block(sats.clone(), cents.clone());
    }
    for vector in additive
        .collect_vecs_mut()
        .into_iter()
        .chain(cumulative.collect_vecs_mut())
        .chain(full.collect_vecs_mut())
    {
        vector.write().unwrap();
    }
    for filter in std::iter::once(&Filter::All)
        .chain(TERM_FILTERS.iter())
        .chain(UNDER_AGE_FILTERS.iter())
        .chain(OVER_AGE_FILTERS.iter())
    {
        let total: u64 = AgeRangeId::ALL
            .iter()
            .filter(|id| filter.includes(id.filter()))
            .map(|id| id.index() as u64 + 1)
            .sum();
        let additive = additive
            .aggregate_source(&SOURCE_CACHE_BUDGET, filter, "sum", Version::ONE)
            .unwrap();
        assert_eq!(additive.collect_range_at(0, 2), [Sats::from(total); 2]);
        for (sats, cents) in [
            cumulative
                .aggregate_sources(&SOURCE_CACHE_BUDGET, filter, "sum", Version::ONE)
                .unwrap(),
            full.sources(&SOURCE_CACHE_BUDGET, filter, "sum", Version::ONE)
                .unwrap(),
        ] {
            assert_eq!(
                sats.collect_one(Height::from(1_usize)),
                Some(Sats::from(total * 2))
            );
            assert_eq!(
                cents.collect_one(Height::from(1_usize)),
                Some(Cents::from(total * 20))
            );
        }
    }
    let unsupported = Filter::Time(TimeFilter::LowerThan(17));
    assert!(
        additive
            .aggregate_source(
                &SOURCE_CACHE_BUDGET,
                &unsupported,
                "unsupported",
                Version::ONE
            )
            .is_none()
    );
    assert!(
        cumulative
            .aggregate_sources(
                &SOURCE_CACHE_BUDGET,
                &unsupported,
                "unsupported",
                Version::ONE
            )
            .is_none()
    );
    assert!(
        full.sources(
            &SOURCE_CACHE_BUDGET,
            &unsupported,
            "unsupported",
            Version::ONE
        )
        .is_none()
    );
}
