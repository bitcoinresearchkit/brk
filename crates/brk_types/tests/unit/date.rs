use super::*;

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
        assert!(crate::Day3::try_from(date).is_err());
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
        assert!(crate::Day3::try_from(date).is_ok());
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
        assert_eq!(serde_json::from_value::<Date>(text.into()).unwrap(), date);
        assert_eq!(serde_json::to_value(date).unwrap(), text);
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
        assert!(
            serde_json::from_value::<Date>(text.into()).is_err(),
            "{text}"
        );
    }
}

#[test]
fn test_date_from_day1_zero() {
    // Day1 0 is Jan 1, 2009
    let date = Date::from(Day1::from(0_usize));
    assert_eq!(date, Date::INDEX_ZERO);
    assert_eq!(date.year(), 2009);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_day1_two() {
    // Day1 2 is Jan 3, 2009 (genesis)
    let date = Date::from(Day1::from(2_usize));
    assert_eq!(date.year(), 2009);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 3);
}

#[test]
fn test_date_from_day1_eight() {
    // Day1 8 is Jan 9, 2009
    let date = Date::from(Day1::from(8_usize));
    assert_eq!(date.year(), 2009);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 9);
}

#[test]
fn test_date_from_week1_zero() {
    // Week1 0 starts at Jan 1, 2009
    let date = Date::from(Week1::from(0_usize));
    assert_eq!(date.year(), 2009);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_week1_one() {
    // Week1 1 is Jan 8, 2009 (one week after epoch)
    let date = Date::from(Week1::from(1_usize));
    assert_eq!(date.year(), 2009);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 8);
}

#[test]
fn test_date_from_month1_zero() {
    // Month1 0 is Jan 1, 2009
    let date = Date::from(Month1::from(0_usize));
    assert_eq!(date.year(), 2009);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_month1_one() {
    // Month1 1 is Feb 1, 2009
    let date = Date::from(Month1::from(1_usize));
    assert_eq!(date.year(), 2009);
    assert_eq!(date.month(), 2);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_month1_twelve() {
    // Month1 12 is Jan 1, 2010
    let date = Date::from(Month1::from(12_usize));
    assert_eq!(date.year(), 2010);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_year1_zero() {
    // Year1 0 is Jan 1, 2009
    let date = Date::from(Year1::from(0_usize));
    assert_eq!(date.year(), 2009);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_year1_one() {
    // Year1 1 is Jan 1, 2010
    let date = Date::from(Year1::from(1_usize));
    assert_eq!(date.year(), 2010);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_month3_zero() {
    // Month3 0 is Q1 2009: Jan 1, 2009
    let date = Date::from(Month3::from(0_usize));
    assert_eq!(date.year(), 2009);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_month3_one() {
    // Month3 1 is Q2 2009: Apr 1, 2009
    let date = Date::from(Month3::from(1_usize));
    assert_eq!(date.year(), 2009);
    assert_eq!(date.month(), 4);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_month3_four() {
    // Month3 4 is Q1 2010: Jan 1, 2010
    let date = Date::from(Month3::from(4_usize));
    assert_eq!(date.year(), 2010);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_month6_zero() {
    // Month6 0 is H1 2009: Jan 1, 2009
    let date = Date::from(Month6::from(0_usize));
    assert_eq!(date.year(), 2009);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_month6_one() {
    // Month6 1 is H2 2009: Jul 1, 2009
    let date = Date::from(Month6::from(1_usize));
    assert_eq!(date.year(), 2009);
    assert_eq!(date.month(), 7);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_month6_two() {
    // Month6 2 is H1 2010: Jan 1, 2010
    let date = Date::from(Month6::from(2_usize));
    assert_eq!(date.year(), 2010);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_year10_zero() {
    // Year10 0 is 2009: Jan 1, 2009
    let date = Date::from(Year10::from(0_usize));
    assert_eq!(date.year(), 2009);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn test_date_from_year10_one() {
    // Year10 1 is 2019: Jan 1, 2019
    let date = Date::from(Year10::from(1_usize));
    assert_eq!(date.year(), 2019);
    assert_eq!(date.month(), 1);
    assert_eq!(date.day(), 1);
}

#[test]
fn schema_matches_date_string_serialization() {
    let date = Date::new(2024, 4, 20);
    assert_eq!(serde_json::to_value(date).unwrap(), "2024-04-20");

    let schema = serde_json::to_value(schemars::schema_for!(Date)).unwrap();
    assert_eq!(schema["type"], "string");
    assert_eq!(schema["format"], "date");
    assert_eq!(schema["pattern"], r"^\d{4}-\d{2}-\d{2}$");
}
