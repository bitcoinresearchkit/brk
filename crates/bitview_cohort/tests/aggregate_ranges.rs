use bitview_cohort::*;

#[test]
fn aggregate_ranges_match_canonical_cohorts() {
    for (cohort, expected) in [
        (CohortId::All, AgeRangeId::ALL),
        (CohortId::Term(Term::Sth), STH_AGE_RANGE_IDS),
        (CohortId::Term(Term::Lth), LTH_AGE_RANGE_IDS),
    ] {
        assert_eq!(cohort.age_ranges().unwrap().collect::<Vec<_>>(), expected);
    }
    for &cohort in UnderAgeId::ALL {
        let hours = *cohort.select(&UNDER_AGE_HOURS);
        let expected: Vec<_> = AgeRangeId::ALL
            .iter()
            .copied()
            .filter(|id| id.bounds().end <= hours)
            .collect();
        assert_eq!(
            cohort.cohort().age_ranges().unwrap().collect::<Vec<_>>(),
            expected
        );
    }
    for &cohort in OverAgeId::ALL {
        let hours = *cohort.select(&OVER_AGE_HOURS);
        let expected: Vec<_> = AgeRangeId::ALL
            .iter()
            .copied()
            .filter(|id| id.bounds().start >= hours)
            .collect();
        assert_eq!(
            cohort.cohort().age_ranges().unwrap().collect::<Vec<_>>(),
            expected
        );
    }
    for &range in AgeRangeId::ALL {
        assert_eq!(
            range.cohort().age_ranges().unwrap().collect::<Vec<_>>(),
            [range]
        );
    }
}

#[test]
fn non_age_cohorts_do_not_resolve_to_age_ranges() {
    for cohort in AmountRangeId::ALL
        .iter()
        .copied()
        .map(AmountRangeId::cohort)
        .chain(EpochId::ALL.iter().copied().map(EpochId::cohort))
        .chain(ClassId::ALL.iter().copied().map(ClassId::cohort))
        .chain(EntryPrice::ALL.iter().copied().map(CohortId::Entry))
    {
        assert!(cohort.age_ranges().is_none(), "{cohort:?}");
    }
}
