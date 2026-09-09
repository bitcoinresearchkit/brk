use bitview_cohort::{AgeRangeId, AmountRangeId};
use bitview_collections::Windows;
use bitview_plugin::ImportContext;
use bitview_plugin_indexer::Indexer;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_vecs::{CachedWindowStartVec, LazyWindowStartVec, import_stored};
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use brk_types::{Cents, CentsSats, CentsSquaredSats, Height, Sats, Version};
use tempfile::tempdir;
use vecdb::{CacheBudget, Database, ReadableVec, WritableVec};

use super::CohortMetrics;
use crate::state::{RealizedOps, UTXOStates};

static CACHE: CacheBudget = CacheBudget::new(64 * 1024 * 1024);

fn capitals(band: usize) -> (CentsSats, CentsSquaredSats) {
    let price = 100_001 + band as u128 * 7;
    (
        CentsSats::new(price * 3 + (price + 11) * 5),
        CentsSquaredSats::new(price.pow(2) * 3 + (price + 11).pow(2) * 5),
    )
}

#[test]
fn block_writes_keep_every_raw_cap_and_include_them_in_resume_checks() {
    let directory = tempdir().unwrap();
    let context = ImportContext::new(directory.path(), &CACHE);
    let client = Client::new("http://127.0.0.1:1", Auth::None).unwrap();
    let reader = Reader::new_without_rlimit(directory.path().join("blocks"), &client);
    let indexer = Indexer::import(context, &reader).unwrap();
    let mappings = MappingsVecs::import(context, &indexer).unwrap();
    let db = Database::open(&directory.path().join("cohorts")).unwrap();
    let spot = import_stored::<Height, Cents>(&CACHE, &db, "spot", Version::ONE).unwrap();
    let starts = CachedWindowStartVec::new(LazyWindowStartVec::days(
        "test_window",
        Version::ONE,
        1,
        mappings.timestamp.monotonic.read_only_cached_boxed_clone(),
    ));
    let windows = Windows {
        _24h: &starts,
        _1w: &starts,
        _1m: &starts,
        _1y: &starts,
    };
    let mut cohorts = CohortMetrics::forced_import(
        &CACHE,
        &db,
        Version::ONE,
        &mappings,
        &windows,
        &spot.read_only_cached_boxed_clone(),
    )
    .unwrap();
    let mut states = UTXOStates::new(&directory.path().join("states"));
    for &id in AgeRangeId::ALL {
        let state = id.select_mut(&mut states.age_range);
        let price = Cents::new(100_001 + id.index() as u64 * 7);
        state.realized.increment(price, Sats::new(3));
        state
            .realized
            .increment(Cents::new(price.inner() + 11), Sats::new(5));
        state.supply.value = Sats::new(8);
    }
    for &id in AmountRangeId::ALL {
        let state = id.select_mut(&mut states.amount_range);
        state.realized.set_cap_raw(capitals(id.index()).0);
        state.supply.value = Sats::new(8);
    }
    for _ in 0..2 {
        cohorts.push_realized(&states);
        cohorts.push_aggregate(&states, Cents::ZERO, &Default::default());
        cohorts.realized.push_addr_balance(
            Default::default(),
            &Default::default(),
            &Default::default(),
        );
    }
    for source in cohorts.realized.collect_vecs_mut() {
        source.write().unwrap();
    }
    let realized = &mut cohorts.realized;
    assert_eq!(realized.min_resume_len(), 2);
    for &id in AgeRangeId::ALL {
        let (rcap, ccap) = capitals(id.index());
        assert_eq!(
            id.select(&realized.cap_raw.age).collect_one_at(1),
            Some(rcap)
        );
        assert_eq!(
            id.select(&realized.capitalized_cap_raw.age)
                .collect_one_at(1),
            Some(ccap)
        );
    }
    for &id in AmountRangeId::ALL {
        assert_eq!(
            id.select(&realized.amount_cap_raw).collect_one_at(1),
            Some(capitals(id.index()).0)
        );
    }
    realized
        .cap_raw
        .age
        .under_1h
        .truncate_if_needed_at(1)
        .unwrap();
    assert_eq!(realized.min_resume_len(), 1);
    realized.cap_raw.age.under_1h.push(capitals(0).0);
    assert_eq!(realized.min_resume_len(), 2);
    realized
        .capitalized_cap_raw
        .age
        .under_1h
        .truncate_if_needed_at(1)
        .unwrap();
    assert_eq!(realized.min_resume_len(), 1);
    realized
        .capitalized_cap_raw
        .age
        .under_1h
        .push(capitals(0).1);
    assert_eq!(realized.min_resume_len(), 2);
    realized
        .amount_cap_raw
        ._0sats
        .truncate_if_needed_at(1)
        .unwrap();
    assert_eq!(realized.min_resume_len(), 1);
}
