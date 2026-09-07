use bitview_cohort::*;
use rayon::iter::ParallelIterator;

#[test]
fn named_age_and_amount_mapping_matches_constructor_order() {
    macro_rules! check {
        ($group:ident) => {{
            let mut expected = Vec::new();
            let original = $group::new(|filter, name| {
                expected.push((filter, name));
                expected.len() - 1
            });
            let mut seen = Vec::new();
            let mapped = original.map_named(|filter, name, value| {
                assert_eq!(&(filter.clone(), name), &expected[*value]);
                seen.push(*value);
                value + 10
            });
            assert_eq!(seen, (0..expected.len()).collect::<Vec<_>>());
            assert!(mapped.iter().copied().eq(10..10 + expected.len()));
        }};
    }
    check!(ByAge);
    check!(Amount);
}

#[test]
fn address_type_views_share_order_names_and_mutable_fields() {
    let mut row = ByAddrType::from_fn(|id| id.index());
    let expected: Vec<_> = ADDR_TYPE_IDS
        .into_iter()
        .map(|id| (id.output_type(), id.index()))
        .collect();
    assert_eq!(
        row.iter()
            .map(|(kind, &value)| (kind, value))
            .collect::<Vec<_>>(),
        expected
    );
    assert_eq!(
        row.values().copied().collect::<Vec<_>>(),
        (0..ADDR_TYPE_COUNT).collect::<Vec<_>>()
    );
    assert_eq!(
        row.par_values().copied().sum::<usize>(),
        (0..ADDR_TYPE_COUNT).sum::<usize>()
    );
    for ((kind, value), &(expected_kind, index)) in row.iter_mut().zip(&expected) {
        assert_eq!(kind, expected_kind);
        assert_eq!(*value, index);
        *value += 10;
    }
    row.par_values_mut().for_each(|value| *value += 20);
    let mapped = row.map_with_name(|name, value| (name, *value));
    for id in ADDR_TYPE_IDS {
        assert_eq!(*id.select(&mapped), (id.name(), id.index() + 30));
    }
    assert_eq!(
        row.into_iter().collect::<Vec<_>>(),
        expected
            .into_iter()
            .map(|(kind, value)| (kind, value + 30))
            .collect::<Vec<_>>()
    );
}

#[test]
fn row_views_preserve_column_order_and_visit_each_field_once() {
    macro_rules! check {
        ($row:ident, $id:ident) => {{
            let mut row = $row::from_fn(|id| id.index());
            let expected: Vec<_> = (0..$id::ALL.len()).collect();
            assert_eq!(row.iter().copied().collect::<Vec<_>>(), expected);
            assert_eq!(
                row.as_array().into_iter().copied().collect::<Vec<_>>(),
                expected
            );
            assert_eq!(
                row.iter().rev().copied().collect::<Vec<_>>(),
                expected.iter().rev().copied().collect::<Vec<_>>()
            );

            for (index, value) in row.iter_mut().enumerate() {
                assert_eq!(*value, index);
                *value += 10;
            }
            for (index, value) in row.as_array_mut().into_iter().enumerate() {
                assert_eq!(*value, index + 10);
                *value += 20;
            }
            row.par_iter_mut().for_each(|value| *value += 30);
            for &id in $id::ALL {
                assert_eq!(*id.select(&row), id.index() + 60);
            }
        }};
    }
    check!(AmountRange, AmountRangeId);
    check!(ByEntry, EntryId);
    check!(Loss, LossId);
    check!(Class, ClassId);
    check!(UTXOAllAndSth, UTXOAllAndSthId);
    check!(UTXOAggregate, UTXOAggregateId);
    check!(UnderAmount, UnderAmountId);
    check!(OverAge, OverAgeId);
    check!(OverAmount, OverAmountId);
    check!(ProfitabilityRange, ProfitabilityRangeId);
    check!(ByEpoch, EpochId);
    check!(Profit, ProfitId);
    check!(ByTerm, TermId);
    check!(UnderAge, UnderAgeId);
}

#[test]
fn typed_views_match_authoritative_bounds_and_profit_flags() {
    let amounts = AmountRange::from_fn(|id| id.index());
    for ((lower, &index), &id) in amounts.iter_typed().zip(AmountRangeId::ALL) {
        assert_eq!(lower, id.select(&AMOUNT_RANGE_BOUNDS).start);
        assert_eq!(index, id.index());
        assert_eq!(*amounts.get(lower), index);
    }
    let mut ranges = ProfitabilityRange::from_fn(|id| id.index());
    for ((is_profit, value), &id) in ranges
        .iter_mut_with_is_profit()
        .zip(ProfitabilityRangeId::ALL)
    {
        assert_eq!(is_profit, id.is_profit());
        assert_eq!(*value, id.index());
        *value += 1;
    }
    assert!(
        ranges
            .iter()
            .copied()
            .eq(1..=ProfitabilityRangeId::ALL.len())
    );
}
