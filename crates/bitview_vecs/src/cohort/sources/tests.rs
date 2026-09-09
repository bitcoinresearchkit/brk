use std::iter;

use bitview_cohort::{AgeRange, CohortId, OverAgeId, Term, UTXOCoreValues, UTXOValues, UnderAgeId};
use brk_types::{Cents, Height, OutputType, Sats, Version};
use tempfile::tempdir;
use vecdb::{CacheBudget, Database, ReadableVec};

use super::{CumulativeUTXOCoreValueSources, CumulativeUTXOValueSources, UTXOCoreSources};

#[test]
fn additive_and_cumulative_sources_share_exact_aggregate_selection() {
    static SOURCE_CACHE_BUDGET: CacheBudget = CacheBudget::new(64 * 1024 * 1024);
    let cache = &SOURCE_CACHE_BUDGET;

    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut additive =
        UTXOCoreSources::<Sats>::forced_import(cache, &db, "additive", Version::ONE).unwrap();
    let mut cumulative =
        CumulativeUTXOCoreValueSources::forced_import(cache, &db, "cumulative", Version::ONE)
            .unwrap();
    let mut full =
        CumulativeUTXOValueSources::forced_import(cache, &db, "full", Version::ONE).unwrap();
    let sats = UTXOValues {
        core: UTXOCoreValues {
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
    for cohort_id in iter::once(CohortId::All)
        .chain([CohortId::Term(Term::Sth), CohortId::Term(Term::Lth)])
        .chain(UnderAgeId::ALL.iter().copied().map(UnderAgeId::cohort))
        .chain(OverAgeId::ALL.iter().copied().map(OverAgeId::cohort))
    {
        let total: u64 = cohort_id
            .age_ranges()
            .unwrap()
            .map(|id| id.index() as u64 + 1)
            .sum();
        let additive = additive.get(cohort_id).unwrap();
        assert_eq!(additive.collect_range_at(0, 2), [Sats::from(total); 2]);
        for (sats, cents) in [
            cumulative.sources(cohort_id, "sum", Version::ONE).unwrap(),
            full.sources(cohort_id, "sum", Version::ONE).unwrap(),
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
    let unsupported = CohortId::Type(OutputType::OpReturn);
    assert!(additive.get(unsupported).is_none());
    assert!(
        cumulative
            .sources(unsupported, "unsupported", Version::ONE)
            .is_none()
    );
    assert!(
        full.sources(unsupported, "unsupported", Version::ONE)
            .is_none()
    );
}
