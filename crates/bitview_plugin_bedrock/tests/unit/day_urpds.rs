use bitview_cohort::{AgeRange, AgeRangeId, UTXO_ALL_NAME};
use bitview_plugin_distribution::{AgeRangeUrpds, UTXOStates};
use brk_types::{Cents, CentsCompact, Date, PercentileId, Sats, UrpdRaw};

use super::{DayUrpds, ModeWeights};

#[test]
fn weighted_sats_are_floored_after_summing() {
    assert_eq!(DayUrpds::floor_sats(0.6 + 0.6), Sats::from(1_u64));
    assert_eq!(DayUrpds::floor_sats(0.6), Sats::ZERO);
}

#[test]
fn current_entries_build_raw_and_weighted_urpds() {
    let weights = ModeWeights::from_fn(|_| Some(AgeRange::from_fn(|_| 0.5)));
    let price = CentsCompact::new(100);
    let urpds = DayUrpds::from_age_entries(
        [
            (AgeRangeId::Under1H, price, Sats::from(3_u64)),
            (AgeRangeId::From5MTo6M, price, Sats::from(5_u64)),
        ],
        &weights,
    );

    assert_eq!(urpds.raw.map[&price], Sats::from(8_u64));
    assert_eq!(urpds.all.cointime.map[&price], Sats::from(4_u64));
    assert_eq!(urpds.term.short.cointime.map[&price], Sats::from(1_u64));
    assert_eq!(urpds.term.long.cointime.map[&price], Sats::from(2_u64));
}

#[test]
fn names_cover_only_stored_aggregate_weights() {
    let names = DayUrpds::names();
    assert_eq!(names.all.cointime, "bedrock_cointime");
    assert_eq!(names.all.coinflow, "bedrock_coinflow");
    assert_eq!(names.sth.cointime, "bedrock_cointime_sth");
    assert_eq!(names.lth.coinflow, "bedrock_coinflow_lth");
}

#[test]
fn persisted_all_cost_basis_percentiles_match_in_memory_percentiles() {
    let root = tempfile::tempdir().unwrap();
    let date = Date::new(2026, 8, 28);
    let names = DayUrpds::names();
    let urpds = DayUrpds::repeated([(100, 5), (200, 5)]);
    assert!(
        DayUrpds::read_all_cost_basis_percentile_prices_if_exists(root.path(), &names, date)
            .unwrap()
            .is_none()
    );
    UrpdRaw::write(
        root.path(),
        &names.all.cointime,
        date,
        std::iter::once((CentsCompact::new(100), Sats::from(1_u64))),
    )
    .unwrap();
    assert!(
        DayUrpds::read_all_cost_basis_percentile_prices_if_exists(root.path(), &names, date)
            .is_err()
    );
    urpds.write(root.path(), &names, date).unwrap();

    let expected = urpds.all_cost_basis_percentile_prices();
    let actual =
        DayUrpds::read_all_cost_basis_percentile_prices_if_exists(root.path(), &names, date)
            .unwrap()
            .expect("persisted pair");

    assert_eq!(actual.cointime, expected.cointime);
    assert_eq!(actual.coinflow, expected.coinflow);
    assert_eq!(
        actual.cointime.per_coin[PercentileId::Pct60 as usize],
        Cents::new(200)
    );
    assert_eq!(
        actual.cointime.per_dollar[PercentileId::Pct50 as usize],
        Cents::new(200)
    );
}

#[test]
fn historical_read_uses_packed_source_without_legacy_all_file() {
    let root = tempfile::tempdir().unwrap();
    let date = Date::new(2026, 8, 26);
    let mut utxos = UTXOStates::new(root.path());
    utxos.reset().unwrap();
    utxos.write_urpds(date, root.path()).unwrap();

    assert!(AgeRangeUrpds::path(root.path(), date).exists());
    assert!(!UrpdRaw::path(root.path(), UTXO_ALL_NAME.id, date).exists());

    let weights = ModeWeights::from_fn(|_| None);
    let urpds = DayUrpds::read_if_exists(root.path(), date, &weights)
        .unwrap()
        .expect("packed source");
    assert!(urpds.raw.map.is_empty());
}
