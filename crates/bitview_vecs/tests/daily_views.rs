mod common;

use bitview_vecs::{DailyMappings, DailyViews};
use brk_types::{Date, Day1, StoredF64, Timestamp, Version};
use vecdb::{AnyVec, Database, ReadableCloneableVec, ReadableVec};

use common::{indexes, stored};

#[test]
fn daily_views_keep_each_resolution_mapping_and_strategy() {
    let directory = tempfile::tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut indexes = indexes(&db);
    let repeated_days = [0, 0, 1, 2, 4];
    let last_days = [0, 2, 2, 3, 7];
    let timestamp = |day| Timestamp::from(Date::from(Day1::from(day)));
    indexes.height_day1 =
        stored(&db, "daily_height", repeated_days.map(Day1::from)).read_only_boxed_clone();
    macro_rules! timestamps {
        ($days:expr; $($field:ident),+ $(,)?) => {$(
            indexes.timestamp.$field = stored(
                &db, concat!("daily_", stringify!($field)), $days.map(timestamp),
            ).read_only_boxed_clone();
        )+};
    }
    timestamps!(repeated_days; minute10, minute30, hour1, hour4, hour12);
    timestamps!(last_days; halving, epoch);
    macro_rules! dates {
        ($($field:ident),+ $(,)?) => {$(
            indexes.$field = stored(
                &db, concat!("daily_", stringify!($field)),
                last_days.map(|day| Date::from(Day1::from(day))),
            ).read_only_boxed_clone();
        )+};
    }
    dates!(
        day3_date,
        week1_date,
        month1_date,
        month3_date,
        month6_date,
        year1_date,
        year10_date
    );
    let mappings = DailyMappings::new(&indexes);
    let source = stored::<Day1, _>(
        &db,
        "daily_values",
        [10.0, 20.0, 30.0, 40.0].map(StoredF64::from),
    );
    let views = DailyViews::new(
        "daily",
        source.read_only_boxed_clone(),
        Version::new(7),
        &mappings,
    );

    macro_rules! check {
        ($expected:expr; $($field:ident),+ $(,)?) => {$(
            let expected = $expected.map(|value| value.map(StoredF64::from));
            assert_eq!(views.$field.name(), "daily");
            assert_eq!(views.$field.collect(), expected, stringify!($field));
            for (index, value) in expected.into_iter().enumerate() {
                assert_eq!(views.$field.collect_one_at(index), Some(value));
            }
        )+};
    }
    check!([Some(10.0), Some(10.0), Some(20.0), Some(30.0), None];
        height, minute10, minute30, hour1, hour4, hour12);
    check!([Some(20.0), None, Some(30.0), Some(40.0), None];
        day3, week1, month1, month3, month6, year1, year10, halving, epoch);
}
