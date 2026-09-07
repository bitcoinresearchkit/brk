use bitview_cohort::*;

#[test]
fn aggregate_columns_match_named_filters_and_reject_unsupported_selections() {
    for (filter, expected) in [
        (&Filter::All, AgeRangeId::ALL),
        (&TERM_FILTERS.short, STH_AGE_RANGE_IDS.as_slice()),
        (&TERM_FILTERS.long, LTH_AGE_RANGE_IDS.as_slice()),
    ] {
        assert_eq!(
            AgeRangeId::aggregate_columns(filter)
                .unwrap()
                .collect::<Vec<_>>(),
            expected
        );
    }
    for filter in UNDER_AGE_FILTERS.iter().chain(OVER_AGE_FILTERS.iter()) {
        let expected: Vec<_> = AgeRangeId::ALL
            .iter()
            .copied()
            .filter(|id| filter.includes(id.filter()))
            .collect();
        assert_eq!(
            AgeRangeId::aggregate_columns(filter)
                .unwrap()
                .collect::<Vec<_>>(),
            expected
        );
    }
    for filter in AGE_RANGE_FILTERS
        .iter()
        .chain(AMOUNT_RANGE_FILTERS.iter())
        .chain(EPOCH_FILTERS.iter())
        .chain(CLASS_FILTERS.iter())
        .chain(ENTRY_FILTERS.iter())
    {
        assert!(
            AgeRangeId::aggregate_columns(filter).is_none(),
            "{filter:?}"
        );
    }
    for filter in [
        Filter::Time(TimeFilter::LowerThan(17)),
        Filter::Time(TimeFilter::GreaterOrEqual(17)),
        Filter::Time(TimeFilter::Range(0..usize::MAX)),
    ] {
        assert!(AgeRangeId::aggregate_columns(&filter).is_none());
    }
}
