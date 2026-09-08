mod common;

use bitview_collections::*;
use bitview_compute::*;
use bitview_transforms::{AvgCentsToUsd, AvgSatsToBtc, CentsUnsignedToDollars, SatsToBitcoin};
use bitview_vecs::*;
use brk_types::{
    Cents, CentsSigned, Height, PartsPerMillionSigned64, Sats, SatsSigned, Timestamp, Version,
};
use common::{indexes, stored};
use schemars::JsonSchema;
use vecdb::{AnyVec, CachedVec, Database, ReadableVec, UnaryTransform};

fn check_conversion<T, S, F>(
    view: &LazyPerBlock<T, S>,
    height: &impl ReadableVec<Height, S>,
    resolutions: &Resolutions<S>,
    name: &str,
    version: Version,
) where
    T: NumericValue + JsonSchema,
    S: NumericValue + JsonSchema,
    F: UnaryTransform<S, T>,
{
    assert_eq!(view.height.name(), name);
    assert_eq!(view.height.version(), version + height.version());
    assert_eq!(
        view.height.collect(),
        height
            .collect()
            .into_iter()
            .map(F::apply)
            .collect::<Vec<_>>()
    );
    macro_rules! period {
        ($($field:ident),+ $(,)?) => {$(
            assert_eq!(view.$field.name(), name);
            assert_eq!(view.$field.version(), version + resolutions.$field.version());
            assert_eq!(
                view.$field.collect(),
                resolutions.$field.collect().into_iter()
                    .map(|value| value.map(F::apply)).collect::<Vec<_>>()
            );
        )+};
    }
    period!(
        minute10, minute30, hour1, hour4, hour12, day1, day3, week1, month1, month3, month6, year1,
        year10
    );
    macro_rules! epoch {
        ($($field:ident),+ $(,)?) => {$(
            assert_eq!(view.$field.name(), name);
            assert_eq!(view.$field.version(), version + resolutions.$field.version());
            assert_eq!(
                view.$field.collect(),
                resolutions.$field.collect().into_iter().map(F::apply).collect::<Vec<_>>()
            );
        )+};
    }
    epoch!(halving, epoch);
}

#[test]
fn rolling_units_preserve_height_and_all_resolution_views() {
    let directory = tempfile::tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut indexes = indexes(&db);
    macro_rules! mappings {
        ($($field:ident),+ $(,)?) => {$(
            indexes.first_height.$field = CachedVec::wrap(stored(
                &db, concat!("populated_", stringify!($field)),
                [0usize, 2, 2, 7, 200].map(Height::from),
            )).read_only_boxed_clone();
        )+};
    }
    mappings!(
        minute10, minute30, hour1, hour4, hour12, day1, day3, week1, month1, month3, month6, year1,
        year10
    );
    indexes.first_height.halving =
        CachedVec::wrap(stored(&db, "populated_halving", [Height::ZERO])).read_only_boxed_clone();
    indexes.first_height.epoch =
        CachedVec::wrap(stored(&db, "populated_epoch", [Height::ZERO])).read_only_boxed_clone();

    let timestamps = CachedVec::wrap(stored::<Height, _>(
        &db,
        "timestamps",
        (0..32u32).map(|i| Timestamp::from(i * i * 43_200)),
    ));
    let starts = WindowId::series(|id| {
        CachedWindowStartVec::new(LazyWindowStartVec::days(
            id.suffix(),
            Version::new(3),
            Windows::<()>::DAYS[vecdb::ColumnId::index(id)] as u64,
            timestamps.read_only_cached_boxed_clone(),
        ))
    });
    let starts_ref = WindowId::series(|id| id.select(&starts));
    let sats_values: Vec<_> = (0..32u64)
        .map(|i| Sats::from(i * (i + 1) * 50_000_003))
        .collect();
    let cents_values: Vec<_> = (0..32u64)
        .map(|i| Cents::from(i * (i + 1) * 12_347))
        .collect();
    let sats = stored::<Height, _>(&db, "sats", sats_values.iter().copied());
    let cents = stored::<Height, _>(&db, "cents", cents_values.iter().copied());
    let spot_sats = stored::<Height, _>(
        &db,
        "spot_sats",
        (0..32u64).map(|i| Sats::from((i % 7) * 100_000_003)),
    );
    let spot_cents = stored::<Height, _>(
        &db,
        "spot_cents",
        (0..32u64).map(|i| Cents::from((i % 5) * 10_003)),
    );
    let version = Version::new(7);
    let sums =
        LazyRollingSumsAmountFromHeight::new("sum", version, &sats, &cents, &starts_ref, &indexes);
    let averages =
        LazyRollingAvgsAmountFromHeight::new("avg", version, &sats, &cents, &starts_ref, &indexes);
    let fiat = LazyRollingSumsFiatFromHeight::new("fiat", version, &cents, &starts_ref, &indexes);
    let amount_delta =
        LazyRollingDeltasAmountFromHeight::<Sats, SatsSigned, PartsPerMillionSigned64>::new(
            "amount_delta",
            version,
            &spot_sats,
            &starts_ref,
            &indexes,
        );
    let fiat_delta =
        LazyRollingDeltasFiatFromHeight::<Cents, CentsSigned, PartsPerMillionSigned64>::new(
            "fiat_delta",
            version,
            &spot_cents,
            &starts_ref,
            &indexes,
        );

    for id in [
        WindowId::Day1,
        WindowId::Week1,
        WindowId::Month1,
        WindowId::Year1,
    ] {
        let suffix = id.suffix();
        let sum = id.select(&sums);
        let avg = id.select(&averages);
        let fiat = id.select(&fiat);
        let start = id.select(&starts);
        let fiat_avg =
            LazyRollingAvgFiatFromHeight::new("fiat_avg", version, &cents, start, &indexes);
        let expected_sats: Vec<_> = start
            .collect()
            .into_iter()
            .enumerate()
            .map(|(i, start)| {
                sats_values[i]
                    - usize::from(start)
                        .checked_sub(1)
                        .map(|j| sats_values[j])
                        .unwrap_or_default()
            })
            .collect();
        assert_eq!(sum.sats.height.collect(), expected_sats);
        assert_eq!(
            sum.sats.resolutions.day1.collect(),
            vec![
                Some(expected_sats[1]),
                None,
                Some(expected_sats[6]),
                Some(expected_sats[31]),
                None,
            ]
        );
        check_conversion::<_, _, SatsToBitcoin>(
            &sum.btc,
            &sum.sats.height,
            &sum.sats.resolutions,
            &format!("sum_{suffix}"),
            version,
        );
        check_conversion::<_, _, CentsUnsignedToDollars>(
            &sum.usd,
            &sum.cents.height,
            &sum.cents.resolutions,
            &format!("sum_{suffix}_usd"),
            version,
        );
        check_conversion::<_, _, AvgSatsToBtc>(
            &avg.btc,
            &avg.sats.height,
            &avg.sats.resolutions,
            &format!("avg_{suffix}"),
            version,
        );
        check_conversion::<_, _, AvgCentsToUsd>(
            &avg.usd,
            &avg.cents.height,
            &avg.cents.resolutions,
            &format!("avg_{suffix}_usd"),
            version,
        );
        check_conversion::<_, _, CentsUnsignedToDollars>(
            &fiat.usd,
            &fiat.cents.height,
            &fiat.cents.resolutions,
            &format!("fiat_{suffix}"),
            version,
        );
        check_conversion::<_, _, AvgCentsToUsd>(
            &fiat_avg.usd,
            &fiat_avg.cents.height,
            &fiat_avg.cents.resolutions,
            "fiat_avg",
            version,
        );
        let amount = id.select(&amount_delta.absolute);
        let money = id.select(&fiat_delta.absolute);
        check_conversion::<_, _, <SatsSigned as AmountType>::ToBitcoin>(
            &amount.btc,
            &amount.sats.height,
            &amount.sats.resolutions,
            &format!("amount_delta_{suffix}"),
            version,
        );
        check_conversion::<_, _, <CentsSigned as FiatType>::ToDollars>(
            &money.usd,
            &money.cents.height,
            &money.cents.resolutions,
            &format!("fiat_delta_{suffix}"),
            version,
        );
        macro_rules! check_rate {
            ($name:literal, $rate:expr) => {{
                let name = $name;
                let rate = $rate;
                check_conversion::<_, _, <PartsPerMillionSigned64 as FixedRatio>::ToRatio>(
                    &rate.ratio,
                    &rate.ppm.height,
                    &rate.ppm.resolutions,
                    &format!("{name}_{suffix}_rate_ratio"),
                    version,
                );
                check_conversion::<_, _, <PartsPerMillionSigned64 as FixedRatio>::ToPercent>(
                    &rate.percent,
                    &rate.ppm.height,
                    &rate.ppm.resolutions,
                    &format!("{name}_{suffix}_rate"),
                    version,
                );
            }};
        }
        check_rate!("amount_delta", id.select(&amount_delta.rate));
        check_rate!("fiat_delta", id.select(&fiat_delta.rate));
    }
}

#[test]
fn compact_and_general_sum_constructors_match() {
    let directory = tempfile::tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let indexes = indexes(&db);
    let timestamps = CachedVec::wrap(stored::<Height, _>(
        &db,
        "timestamps",
        (0..16u32).map(|i| Timestamp::from(i * 43_200)),
    ));
    let starts = WindowId::series(|id| {
        CachedWindowStartVec::new(LazyWindowStartVec::days(
            id.suffix(),
            Version::ONE,
            Windows::<()>::DAYS[vecdb::ColumnId::index(id)] as u64,
            timestamps.read_only_cached_boxed_clone(),
        ))
    });
    let starts = WindowId::series(|id| id.select(&starts));
    let cumulative = stored::<Height, _>(&db, "counts", (0..16u64).map(brk_types::StoredU64::from));
    let general =
        LazyRollingSumsFromHeight::new("counts_sum", Version::ONE, &cumulative, &starts, &indexes);
    let compact =
        LazyRollingSumsFromHeight::new("counts_sum", Version::ONE, &cumulative, &starts, &indexes);
    for (a, b) in general.as_array().into_iter().zip(compact.as_array()) {
        assert_eq!(a.height.name(), b.height.name());
        assert_eq!(a.height.version(), b.height.version());
        assert_eq!(a.height.collect(), b.height.collect());
    }
}
