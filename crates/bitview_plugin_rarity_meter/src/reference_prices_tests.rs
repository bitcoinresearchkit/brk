#[cfg(test)]
use crate::test_cache::init_cache;
use std::{array, ops::Range};

use bitview_cohort::{AgeRange, AgeRangeId, CohortId, Term, UTXOAggregate};
use bitview_transforms::price_ratio;
use bitview_traversable::TreeNode;
use bitview_vecs::UTXOAgeSources;
use brk_types::StoredF32;
use tempfile::tempdir;
use vecdb::{Budgeted, EagerVec, PcoVec};

use super::*;
use crate::test_common as common;

const SOURCE_COHORTS: [CohortId; 4] = [
    CohortId::Term(Term::Sth),
    CohortId::Term(Term::Lth),
    CohortId::Age(AgeRangeId::From4MTo5M),
    CohortId::Age(AgeRangeId::From5MTo6M),
];

fn compute(
    prices: &mut ReferencePrices,
    height: Height,
    caps: &UTXOAgeSources<CentsSats>,
    supplies: &[EagerVec<PcoVec<Height, Sats, Budgeted>>; 4],
    spot: &impl ReadableVec<Height, Cents>,
    exit: &Exit,
) -> Result<()> {
    prices.compute(
        height,
        [
            &caps.term.short,
            &caps.term.long,
            &caps.age._4m_to_5m,
            &caps.age._5m_to_6m,
        ],
        supplies.each_ref(),
        spot,
        exit,
    )
}

fn inputs(band: usize, row: usize) -> (CentsSats, Sats) {
    // Include entirely empty supply and lots sitting exactly in each boundary band.
    if row == 0
        || (row == 3 && band != AgeRangeId::From4MTo5M.index())
        || (row == 4 && band != AgeRangeId::From5MTo6M.index())
    {
        return (CentsSats::ZERO, Sats::ZERO);
    }
    let price = (u32::MAX as u128 + 101) * row as u128 + band as u128;
    let sats = band as u128 + 3;
    // Two acquisition prices, leaving a remainder after division by supply.
    (
        CentsSats::new(price * sats + (price + 7) * 2),
        Sats::new((sats + 2) as u64),
    )
}

fn append(
    caps: &mut UTXOAgeSources<CentsSats>,
    supplies: &mut [EagerVec<PcoVec<Height, Sats, Budgeted>>; 4],
    rows: Range<usize>,
) {
    for row in rows {
        caps.push_age(&AgeRange::from_fn(|id| inputs(id.index(), row).0));
        caps.push(&UTXOAggregate::from_fn(|id| {
            id.cohort()
                .age_ranges()
                .unwrap()
                .fold(CentsSats::ZERO, |sum, id| sum + inputs(id.index(), row).0)
        }));
        for (id, supply) in SOURCE_COHORTS.into_iter().zip(supplies.iter_mut()) {
            supply.push(
                id.age_ranges()
                    .unwrap()
                    .fold(Sats::ZERO, |sum, id| sum + inputs(id.index(), row).1),
            );
        }
    }
    for source in caps.collect_vecs_mut() {
        source.write().unwrap();
    }
    for source in supplies.iter_mut() {
        source.write().unwrap();
    }
}

fn check(prices: &ReferencePrices, spot: &[Cents], rows: &[usize]) {
    let split_4m = AgeRangeId::From4MTo5M.index();
    let split_6m = AgeRangeId::From6MTo9M.index();
    let ranges = [
        0..split_4m,
        0..split_6m,
        split_4m..AgeRangeId::ALL.len(),
        split_6m..AgeRangeId::ALL.len(),
    ];
    for (price, range) in [
        &prices.under_4m,
        &prices.under_6m,
        &prices.over_4m,
        &prices.over_6m,
    ]
    .into_iter()
    .zip(ranges)
    {
        assert_eq!(price.cents.height.len(), rows.len());
        assert_eq!(price.relative.ppm.height.len(), rows.len().min(spot.len()));
        assert_eq!(
            price.relative.ratio.height.len(),
            rows.len().min(spot.len())
        );
        for (height, &row) in rows.iter().enumerate() {
            let (cap, supply) = range.clone().fold((0u128, 0u128), |(cap, supply), band| {
                let (raw, sats) = inputs(band, row);
                (cap + raw.as_u128(), supply + sats.as_u128())
            });
            let expected = Cents::new(cap.checked_div(supply).unwrap_or(0) as u64);
            assert_eq!(price.cents.height.collect_one_at(height), Some(expected));
            if let Some(&spot) = spot.get(height) {
                let expected = price_ratio(spot, expected);
                assert_eq!(
                    price.relative.ppm.height.collect_one_at(height),
                    Some(expected)
                );
                let ratio = price.relative.ratio.height.collect_one_at(height).unwrap();
                if expected.is_nan() {
                    assert!(ratio.is_nan());
                } else {
                    assert_eq!(ratio, StoredF32::from(expected));
                }
            }
        }
    }
}

#[test]
fn reference_prices_keep_the_standard_price_and_ratio_schema() {
    init_cache();
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let indexes = common::indexes(&db);
    let prices = ReferencePrices::forced_import(&db, Version::ONE, &indexes).unwrap();
    for (name, price) in [
        ("under_4m", &prices.under_4m),
        ("under_6m", &prices.under_6m),
        ("over_4m", &prices.over_4m),
        ("over_6m", &prices.over_6m),
    ] {
        let TreeNode::Branch(branch) = price.to_tree_node() else {
            panic!("expected price branch");
        };
        for (name, kind) in [
            ("cents", "Cents"),
            ("ppm", "PriceRatio"),
            ("ratio", "StoredF32"),
        ] {
            let Some(TreeNode::Leaf(leaf)) = branch.get(name) else {
                panic!("expected {name} leaf");
            };
            assert_eq!(leaf.kind(), kind);
        }
        assert!(!branch.contains_key("relative"));
        assert_eq!(
            price.relative.ppm.height.name(),
            format!("rarity_meter_{name}_realized_price_ratio_ppm")
        );
    }
}

#[test]
fn exact_reference_prices_survive_append_reorg_shortening_and_reopen() {
    init_cache();
    let directory = tempdir().unwrap();
    let spot_values = [Cents::new(18_000_000_000); 5];
    {
        let db = Database::open(directory.path()).unwrap();
        let indexes = common::indexes(&db);
        let spot = common::stored(&db, "spot", spot_values);
        let mut caps = UTXOAgeSources::forced_import(&db, "cap_raw", Version::ONE).unwrap();
        let mut supplies = array::from_fn(|i| common::stored(&db, &format!("supply_{i}"), []));
        let mut prices = ReferencePrices::forced_import(&db, Version::ONE, &indexes).unwrap();
        let exit = Exit::new();
        append(&mut caps, &mut supplies, 0..3);
        compute(&mut prices, Height::ZERO, &caps, &supplies, &spot, &exit).unwrap();
        check(&prices, &spot_values, &[0, 1, 2]);
        append(&mut caps, &mut supplies, 3..5);
        compute(
            &mut prices,
            Height::from(3usize),
            &caps,
            &supplies,
            &spot,
            &exit,
        )
        .unwrap();
        check(&prices, &spot_values, &[0, 1, 2, 3, 4]);

        for source in caps.collect_vecs_mut() {
            source.any_truncate_if_needed_at(2).unwrap();
        }
        for source in supplies.iter_mut() {
            source.truncate_if_needed_at(2).unwrap();
        }
        append(&mut caps, &mut supplies, 7..10);
        compute(
            &mut prices,
            Height::from(2usize),
            &caps,
            &supplies,
            &spot,
            &exit,
        )
        .unwrap();
        check(&prices, &spot_values, &[0, 1, 7, 8, 9]);

        // A short input must shorten all outputs even when safe height is later.
        supplies[0].truncate_if_needed_at(3).unwrap();
        supplies[0].write().unwrap();
        compute(
            &mut prices,
            Height::from(99usize),
            &caps,
            &supplies,
            &spot,
            &exit,
        )
        .unwrap();
        check(&prices, &spot_values, &[0, 1, 7]);
        db.flush().unwrap();
    }
    let db = Database::open(directory.path()).unwrap();
    let indexes = common::indexes(&db);
    let prices = ReferencePrices::forced_import(&db, Version::ONE, &indexes).unwrap();
    check(&prices, &spot_values, &[0, 1, 7]);
}

#[test]
fn changed_raw_input_versions_rebuild_prices_and_ratios_from_zero() {
    init_cache();
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let indexes = common::indexes(&db);
    let spot_values = [Cents::new(23_000_000_000); 2];
    let mut spot = common::stored(&db, "spot", spot_values);
    let mut caps = UTXOAgeSources::forced_import(&db, "cap_raw", Version::ONE).unwrap();
    let mut supplies = array::from_fn(|i| common::stored(&db, &format!("supply_{i}"), []));
    let mut prices = ReferencePrices::forced_import(&db, Version::ONE, &indexes).unwrap();
    let exit = Exit::new();
    append(&mut caps, &mut supplies, 1..3);
    compute(&mut prices, Height::ZERO, &caps, &supplies, &spot, &exit).unwrap();
    check(&prices, &spot_values, &[1, 2]);
    drop(caps);
    let mut caps = UTXOAgeSources::forced_import(&db, "cap_raw", Version::new(2)).unwrap();
    for supply in supplies.iter_mut() {
        supply.truncate_if_needed_at(0).unwrap();
    }
    append(&mut caps, &mut supplies, 8..10);
    compute(
        &mut prices,
        Height::from(2usize),
        &caps,
        &supplies,
        &spot,
        &exit,
    )
    .unwrap();
    check(&prices, &spot_values, &[8, 9]);
    spot.truncate_if_needed_at(1).unwrap();
    spot.write().unwrap();
    compute(
        &mut prices,
        Height::from(2usize),
        &caps,
        &supplies,
        &spot,
        &exit,
    )
    .unwrap();
    check(&prices, &spot_values[..1], &[8, 9]);

    caps.age.under_1h.truncate_if_needed_at(0).unwrap();
    caps.age.under_1h.write().unwrap();
    compute(
        &mut prices,
        Height::from(2usize),
        &caps,
        &supplies,
        &spot,
        &exit,
    )
    .unwrap();
    check(&prices, &spot_values[..1], &[8, 9]);

    caps.age._5m_to_6m.truncate_if_needed_at(0).unwrap();
    caps.age._5m_to_6m.write().unwrap();
    compute(
        &mut prices,
        Height::from(2usize),
        &caps,
        &supplies,
        &spot,
        &exit,
    )
    .unwrap();
    check(&prices, &spot_values[..1], &[]);
}
