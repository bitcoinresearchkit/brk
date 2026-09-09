use bitview_cohort::*;
use brk_types::OutputType;

#[test]
fn identity_roundtrips_through_composed_groups() {
    let cohorts = UTXOGroups::new(|id| id);
    assert_eq!(cohorts.iter().count(), 77);
    for &id in cohorts.iter() {
        assert_eq!(cohorts.get(id), Some(&id));
    }
    cohorts.map_with_id(|id, &value| assert_eq!(id, value));
    assert_eq!(cohorts.get(CohortId::Type(OutputType::OpReturn)), None);

    let core = UTXOGroupCore::new(|id| id);
    for &id in core.iter() {
        assert_eq!(core.get(id), Some(&id));
    }
    assert_eq!(core.get(CohortId::Term(Term::Sth)), None);
    assert_eq!(core.get(AmountRangeId::Zero.cohort()), None);
    assert_eq!(core.get(CohortId::Type(OutputType::P2PKH)), None);

    let ages = AgeRange::from_fn(|id| id);
    for &id in ages.iter() {
        assert_eq!(*id.select(&ages), id);
    }
    let amounts = AmountRange::from_fn(|id| id);
    for &id in amounts.iter() {
        assert_eq!(*id.select(&amounts), id);
    }
}

#[test]
fn holder_classification_matches_age_bounds_and_aggregate_selectors() {
    for &id in AgeRangeId::ALL {
        let bounds = id.bounds();
        match id.term() {
            Term::Sth => assert!(bounds.end <= Term::THRESHOLD_HOURS),
            Term::Lth => assert!(bounds.start >= Term::THRESHOLD_HOURS),
        }
    }
    for &id in UTXOAggregateId::ALL {
        assert_eq!(
            id.cohort().age_ranges().unwrap().collect::<Vec<_>>(),
            id.age_range_ids(),
        );
    }
}

#[test]
fn canonical_names_preserve_series_prefixes() {
    for (id, name) in [
        (CohortId::All, "supply"),
        (CohortId::Term(Term::Sth), "sth_supply"),
        (CohortId::Term(Term::Lth), "lth_supply"),
        (AgeRangeId::Under1H.cohort(), "utxos_under_1h_old_supply"),
        (AgeRangeId::Over15Y.cohort(), "utxos_over_15y_old_supply"),
        (AmountRangeId::Zero.cohort(), "utxos_0sats_supply"),
        (EpochId::_0.cohort(), "epoch_0_supply"),
        (ClassId::_2009.cohort(), "class_2009_supply"),
        (CohortId::Entry(EntryPrice::Discount), "veteran_supply"),
        (CohortId::Entry(EntryPrice::Premium), "rookie_supply"),
        (
            CohortId::Type(OutputType::Unknown),
            "unknown_outputs_supply",
        ),
        (CohortId::Type(OutputType::Empty), "empty_outputs_supply"),
        (CohortId::Type(OutputType::OpReturn), "op_return_supply"),
    ] {
        assert_eq!(CohortContext::Utxo.metric_name(id, "supply"), name);
    }
    assert_eq!(
        CohortContext::Addr.metric_name(AmountRangeId::Zero.cohort(), "supply"),
        "addrs_0sats_supply"
    );
}

#[test]
fn amount_identity_keeps_disjoint_bucket_names() {
    let values = AmountRange::from_fn(|id| id.index() as u64 + 1);
    for &id in AmountRangeId::ALL {
        assert_eq!(*id.select(&values), id.index() as u64 + 1);
        assert_eq!(id.name().id, id.cohort().name());
    }
}
