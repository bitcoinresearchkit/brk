use schemars::schema_for;
use serde_json::{from_value, to_value};

use super::*;
use crate::Day3;

#[test]
fn fallible_indexes_reject_unchecked_invalid_calendar_values() {
    for date in [
        Date::new(2026, 2, 29),
        Date::new(2026, 2, 30),
        Date::new(2026, 2, 31),
        Date::new(2009, 0, 1),
        Date::new(2009, 13, 1),
        Date::new(2009, 1, 0),
        Date::new(10_000, 1, 1),
        Date::new(u16::MAX, 1, 1),
        Date(u32::MAX),
    ] {
        assert!(date.try_into_jiff().is_err(), "{date}");
        assert!(Day1::try_from(date).is_err());
        assert!(Day3::try_from(date).is_err());
        assert!(Week1::try_from(date).is_err());
        assert!(Month1::try_from(date).is_err());
        assert!(Month3::try_from(date).is_err());
        assert!(Month6::try_from(date).is_err());
        assert!(Year1::try_from(date).is_err());
        assert!(Year10::try_from(date).is_err());
    }
    for date in [Date::new(2024, 2, 29), Date::INDEX_ZERO] {
        assert_eq!(date.try_into_jiff().unwrap(), date.into_jiff());
        assert!(Day1::try_from(date).is_ok());
        assert!(Day3::try_from(date).is_ok());
        assert!(Week1::try_from(date).is_ok());
        assert!(Month1::try_from(date).is_ok());
        assert!(Month3::try_from(date).is_ok());
        assert!(Month6::try_from(date).is_ok());
        assert!(Year1::try_from(date).is_ok());
        assert!(Year10::try_from(date).is_ok());
    }
}

#[test]
fn parsing_and_serde_share_strict_calendar_validation() {
    for text in [
        "0000-01-01",
        "0001-01-01",
        "2000-02-29",
        "2024-02-29",
        "9999-12-31",
    ] {
        let date: Date = text.parse().unwrap();
        assert_eq!(date.to_string(), text);
        assert_eq!(from_value::<Date>(text.into()).unwrap(), date);
        assert_eq!(to_value(date).unwrap(), text);
    }
    for text in [
        "2026-02-29",
        "2026-02-30",
        "2026-02-31",
        "2023-02-29",
        "1900-02-29",
        "2026-04-31",
        "2026-00-01",
        "2026-13-01",
        "2026-01-00",
        "2026-01-32",
        "2026_01_01",
        "2026-+1-01",
        "+026-01-01",
        "123é01-01",
        "2026-1-01",
        "",
        "2026-01-01extra",
    ] {
        assert!(text.parse::<Date>().is_err(), "{text}");
        assert!(from_value::<Date>(text.into()).is_err(), "{text}");
    }
}

#[test]
fn calendar_conversions_match_epoch_and_period_boundaries() {
    for (actual, expected) in [
        (Date::from(Day1::from(0_usize)), Date::new(2009, 1, 1)),
        (Date::from(Day1::from(2_usize)), Date::new(2009, 1, 3)),
        (Date::from(Week1::from(1_usize)), Date::new(2009, 1, 8)),
        (Date::from(Month1::from(1_usize)), Date::new(2009, 2, 1)),
        (Date::from(Month1::from(12_usize)), Date::new(2010, 1, 1)),
        (Date::from(Month3::from(1_usize)), Date::new(2009, 4, 1)),
        (Date::from(Month6::from(1_usize)), Date::new(2009, 7, 1)),
        (Date::from(Year1::from(1_usize)), Date::new(2010, 1, 1)),
        (Date::from(Year10::from(1_usize)), Date::new(2019, 1, 1)),
    ] {
        assert_eq!(actual, expected);
    }
}

#[test]
fn schema_matches_date_string_serialization() {
    let date = Date::new(2024, 4, 20);
    assert_eq!(to_value(date).unwrap(), "2024-04-20");

    let schema = to_value(schema_for!(Date)).unwrap();
    assert_eq!(schema["type"], "string");
    assert_eq!(schema["format"], "date");
    assert_eq!(schema["pattern"], r"^\d{4}-\d{2}-\d{2}$");
}
