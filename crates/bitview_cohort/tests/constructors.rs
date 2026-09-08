use bitview_cohort::*;

#[test]
fn row_iteration_and_mutation_follow_column_order() {
    macro_rules! check {
        ($row:ident, $id:ident) => {{
            let mut row = $row::from_fn(|id| id.index());
            assert!(row.iter().copied().eq(0..$id::ALL.len()));
            for value in row.iter_mut() {
                *value += 1;
            }
            for &id in $id::ALL {
                assert_eq!(*id.select(&row), id.index() + 1);
            }
        }};
    }
    check!(AgeRange, AgeRangeId);
    check!(AmountRange, AmountRangeId);
    check!(ByEntry, EntryId);
    check!(ByEpoch, EpochId);
    check!(ByTerm, TermId);
    check!(Class, ClassId);
    check!(Profit, ProfitId);
    check!(Loss, LossId);
    check!(ProfitabilityRange, ProfitabilityRangeId);
    check!(UTXOAggregate, UTXOAggregateId);
    check!(UTXOAllAndSth, UTXOAllAndSthId);
    check!(UnderAge, UnderAgeId);
    check!(OverAge, OverAgeId);
    check!(UnderAmount, UnderAmountId);
    check!(OverAmount, OverAmountId);
}

#[test]
fn fallible_construction_stops_at_the_first_error() {
    let mut calls = 0;
    let result = AmountRange::try_new(|_, name| {
        calls += 1;
        if calls == 2 { Err(name) } else { Ok(()) }
    });
    assert_eq!(
        result.err(),
        Some(AMOUNT_RANGE_NAMES.iter().nth(1).unwrap().id)
    );
    assert_eq!(calls, 2);
}

#[test]
fn utxo_and_address_names_do_not_collide() {
    let groups = UTXOAndAddrGroups {
        utxo: UTXOGroups::new(|_, _| ()),
        addr_balance: Amount::new(|_, _| ()),
    };
    let mut names = std::collections::BTreeSet::new();
    groups.map_named(|context, filter, name, _| {
        assert!(names.insert(context.full_name(filter, name)));
    });
    assert_eq!(
        names.len(),
        groups.utxo.iter().count() + groups.addr_balance.iter().count()
    );
}
