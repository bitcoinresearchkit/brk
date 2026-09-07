mod common;

use bitview_compute::{
    CachedWindowStartVec, LazyRollingAvgFiatFromHeight, LazyRollingAvgsAmountFromHeight,
    LazyRollingAvgsFromHeight, LazyRollingSumsAmountFromHeight, LazyRollingSumsFiatFromHeight,
    LazyRollingSumsFromHeight, LazyWindowStartVec, Windows,
};
use brk_types::{Cents, Height, Sats, StoredF32, Timestamp, Version};
use vecdb::{
    AnyVec, CachedVec, DeltaAvg, DeltaSub, LazyDeltaVec, ReadableCloneableVec, ReadableVec,
};

#[test]
fn rolling_average_families_preserve_values_names_and_versions() {
    let directory = tempfile::tempdir().unwrap();
    let db = vecdb::Database::open(directory.path()).unwrap();
    let mut indexes = common::indexes(&db);
    indexes.cached_first_height.day1 = CachedVec::wrap(common::stored(
        &db,
        "average_days",
        [0usize, 2, 3].map(Height::from),
    ))
    .read_only_cached_boxed_clone();
    let timestamps = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "average_timestamps",
        [0u32, 43_200, 86_400, 129_600].map(Timestamp::from),
    ))
    .read_only_cached_boxed_clone();
    let start = CachedWindowStartVec::new(LazyWindowStartVec::days(
        "average_start",
        Version::ONE,
        1,
        timestamps,
    ));
    let starts = Windows {
        _24h: &start,
        _1w: &start,
        _1m: &start,
        _1y: &start,
    };
    let sats =
        common::stored::<Height, _>(&db, "average_sats", [10u64, 30, 60, 100].map(Sats::from));
    let cents = common::stored::<Height, _>(
        &db,
        "average_cents",
        [100u64, 300, 600, 1000].map(Cents::from),
    );
    let version = Version::new(5);
    let plain = LazyRollingAvgsFromHeight::new("plain", version, &sats, &starts, &indexes);
    let amount =
        LazyRollingAvgsAmountFromHeight::new("amount", version, &sats, &cents, &starts, &indexes);
    let fiat = LazyRollingAvgFiatFromHeight::new("fiat", version, &cents, &start, &indexes);

    macro_rules! check {
        ($view:expr, $name:expr, $source:expr, $values:expr) => {{
            let view = $view;
            let cached = start.clone();
            let original: LazyDeltaVec<Height, _, StoredF32, DeltaAvg> = LazyDeltaVec::new(
                &$name,
                version,
                $source.read_only_boxed_clone(),
                cached.version(),
                move || cached.snapshot(),
            );
            assert_eq!(view.height.name(), original.name());
            assert_eq!(view.height.version(), original.version());
            assert_eq!(view.height.collect(), original.collect());
            assert_eq!(view.height.collect(), $values.map(StoredF32::from));
            assert_eq!(
                view.resolutions.day1.collect(),
                [$values[1], $values[2], $values[3]].map(|value| Some(StoredF32::from(value)))
            );
        }};
    }
    for ((suffix, plain), amount) in Windows::<()>::SUFFIXES
        .into_iter()
        .zip(plain.as_array())
        .zip(amount.as_array())
    {
        check!(
            plain,
            format!("plain_{suffix}"),
            sats,
            [10.0, 15.0, 25.0, 35.0]
        );
        check!(
            &amount.sats,
            format!("amount_{suffix}_sats"),
            sats,
            [10.0, 15.0, 25.0, 35.0]
        );
        check!(
            &amount.cents,
            format!("amount_{suffix}_cents"),
            cents,
            [100.0, 150.0, 250.0, 350.0]
        );
        assert_eq!(amount.btc.height.name(), format!("amount_{suffix}"));
        assert_eq!(amount.usd.height.name(), format!("amount_{suffix}_usd"));
    }
    check!(
        &fiat.cents,
        "fiat_cents",
        cents,
        [100.0, 150.0, 250.0, 350.0]
    );
    assert_eq!(fiat.usd.height.name(), "fiat");

    let plain = LazyRollingSumsFromHeight::new("plain_sum", version, &sats, &starts, &indexes);
    let amount = LazyRollingSumsAmountFromHeight::new(
        "amount_sum",
        version,
        &sats,
        &cents,
        &starts,
        &indexes,
    );
    let fiat = LazyRollingSumsFiatFromHeight::new("fiat_sum", version, &cents, &starts, &indexes);
    macro_rules! check_sum {
        ($view:expr, $name:expr, $source:expr, $values:expr) => {{
            let view = $view;
            let cached = start.clone();
            let original: LazyDeltaVec<Height, _, _, DeltaSub> = LazyDeltaVec::new(
                &$name,
                version,
                $source.read_only_boxed_clone(),
                cached.version(),
                move || cached.snapshot(),
            );
            assert_eq!(view.height.name(), original.name());
            assert_eq!(view.height.version(), original.version());
            assert_eq!(view.height.collect(), original.collect());
            assert_eq!(view.height.collect(), $values);
            assert_eq!(
                view.resolutions.day1.collect(),
                [$values[1], $values[2], $values[3]].map(Some)
            );
        }};
    }
    for (((suffix, plain), amount), fiat) in Windows::<()>::SUFFIXES
        .into_iter()
        .zip(plain.as_array())
        .zip(amount.as_array())
        .zip(fiat.as_array())
    {
        check_sum!(
            plain,
            format!("plain_sum_{suffix}"),
            sats,
            [10u64, 30, 50, 70].map(Sats::from)
        );
        check_sum!(
            &amount.sats,
            format!("amount_sum_{suffix}_sats"),
            sats,
            [10u64, 30, 50, 70].map(Sats::from)
        );
        check_sum!(
            &amount.cents,
            format!("amount_sum_{suffix}_cents"),
            cents,
            [100u64, 300, 500, 700].map(Cents::from)
        );
        check_sum!(
            &fiat.cents,
            format!("fiat_sum_{suffix}_cents"),
            cents,
            [100u64, 300, 500, 700].map(Cents::from)
        );
        assert_eq!(amount.btc.height.name(), format!("amount_sum_{suffix}"));
        assert_eq!(amount.usd.height.name(), format!("amount_sum_{suffix}_usd"));
        assert_eq!(fiat.usd.height.name(), format!("fiat_sum_{suffix}"));
    }
}
