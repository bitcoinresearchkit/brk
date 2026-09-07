use bitview_cohort::*;

#[test]
fn profitability_series_preserves_names_ids_and_callback_order() {
    let mut visited = Vec::new();
    let row = ProfitabilityId::series(|id, name| {
        visited.push((id, name));
        id.index()
    });
    let names: Vec<_> = PROFITABILITY_RANGE_NAMES
        .iter()
        .chain(PROFIT_NAMES.iter())
        .chain(LOSS_NAMES.iter())
        .map(|name| name.id)
        .collect();
    assert_eq!(
        visited,
        ProfitabilityId::ALL
            .iter()
            .copied()
            .zip(names)
            .collect::<Vec<_>>()
    );
    for &id in ProfitabilityId::ALL {
        assert_eq!(*id.select(&row), id.index());
    }
}

#[test]
fn named_constructors_preserve_column_order_and_each_early_error() {
    macro_rules! check {
        ($row:ident, $id:ident, $names:ident) => {{
            let expected: Vec<_> = $id::ALL.iter().map(|id| id.select(&$names).id).collect();
            let direct = $row::new(|name| name.to_owned());
            let fallible = $row::try_new(|name| Ok::<_, ()>(name.to_owned())).unwrap();
            assert_eq!(
                direct.iter().map(String::as_str).collect::<Vec<_>>(),
                expected
            );
            assert_eq!(
                fallible.iter().map(String::as_str).collect::<Vec<_>>(),
                expected
            );

            for failure_at in 0..expected.len() {
                let mut visited = Vec::new();
                let result = $row::try_new(|name| {
                    visited.push(name);
                    if visited.len() == failure_at + 1 {
                        Err(name)
                    } else {
                        Ok(name.to_owned())
                    }
                });
                assert_eq!(result.err(), Some(expected[failure_at]));
                assert_eq!(visited, expected[..=failure_at]);
            }
        }};
    }
    check!(Profit, ProfitId, PROFIT_NAMES);
    check!(Loss, LossId, LOSS_NAMES);
    check!(
        ProfitabilityRange,
        ProfitabilityRangeId,
        PROFITABILITY_RANGE_NAMES
    );
}

#[test]
fn constructors_preserve_column_order_names_filters_and_early_errors() {
    macro_rules! check {
        ($row:ident, $id:ident, $filters:ident, $names:ident) => {{
            let expected: Vec<_> = $id::ALL
                .iter()
                .map(|id| (id.select(&$filters).clone(), id.select(&$names).id))
                .collect();
            let direct = $row::new(|filter, name| (filter, name));
            let fallible = $row::try_new(|filter, name| Ok::<_, ()>((filter, name))).unwrap();
            assert_eq!(direct.iter().cloned().collect::<Vec<_>>(), expected);
            assert_eq!(fallible.iter().cloned().collect::<Vec<_>>(), expected);

            let mut visited = Vec::new();
            let failure = $row::try_new(|filter, name| {
                visited.push((filter, name));
                if visited.len() == 2 {
                    Err(name)
                } else {
                    Ok(())
                }
            });
            assert_eq!(failure.err(), Some(expected[1].1));
            assert_eq!(visited, expected[..2]);
        }};
    }
    check!(
        AmountRange,
        AmountRangeId,
        AMOUNT_RANGE_FILTERS,
        AMOUNT_RANGE_NAMES
    );
    check!(OverAge, OverAgeId, OVER_AGE_FILTERS, OVER_AGE_NAMES);
    check!(UnderAge, UnderAgeId, UNDER_AGE_FILTERS, UNDER_AGE_NAMES);
    check!(
        OverAmount,
        OverAmountId,
        OVER_AMOUNT_FILTERS,
        OVER_AMOUNT_NAMES
    );
    check!(
        UnderAmount,
        UnderAmountId,
        UNDER_AMOUNT_FILTERS,
        UNDER_AMOUNT_NAMES
    );
}
