use super::*;

#[test]
fn test_is_date_based_day1() {
    assert!(Index::Day1.is_date_based());
}

#[test]
fn test_is_date_based_week1() {
    assert!(Index::Week1.is_date_based());
}

#[test]
fn test_is_date_based_month1() {
    assert!(Index::Month1.is_date_based());
}

#[test]
fn test_is_date_based_year1() {
    assert!(Index::Year1.is_date_based());
}

#[test]
fn test_is_date_based_month3() {
    assert!(Index::Month3.is_date_based());
}

#[test]
fn test_is_date_based_month6() {
    assert!(Index::Month6.is_date_based());
}

#[test]
fn test_is_date_based_year10() {
    assert!(Index::Year10.is_date_based());
}

#[test]
fn test_is_not_date_based_height() {
    assert!(!Index::Height.is_date_based());
}

#[test]
fn test_is_not_date_based_tx_index() {
    assert!(!Index::TxIndex.is_date_based());
}

#[test]
fn test_index_to_date_day1_zero() {
    let date = Index::Day1.index_to_date(0).unwrap();
    assert_eq!(date, Date::from(Day1::from(0_usize)));
}

#[test]
fn test_index_to_date_day1_one() {
    let date = Index::Day1.index_to_date(1).unwrap();
    assert_eq!(date, Date::from(Day1::from(1_usize)));
}

#[test]
fn test_index_to_date_week1() {
    let date = Index::Week1.index_to_date(1).unwrap();
    assert_eq!(date, Date::from(Week1::from(1_usize)));
}

#[test]
fn test_index_to_date_month1() {
    let date = Index::Month1.index_to_date(12).unwrap();
    assert_eq!(date, Date::from(Month1::from(12_usize)));
}

#[test]
fn test_index_to_date_year1() {
    let date = Index::Year1.index_to_date(5).unwrap();
    assert_eq!(date, Date::from(Year1::from(5_usize)));
}

#[test]
fn test_index_to_date_month3() {
    let date = Index::Month3.index_to_date(4).unwrap();
    assert_eq!(date, Date::from(Month3::from(4_usize)));
}

#[test]
fn test_index_to_date_month6() {
    let date = Index::Month6.index_to_date(2).unwrap();
    assert_eq!(date, Date::from(Month6::from(2_usize)));
}

#[test]
fn test_index_to_date_year10() {
    let date = Index::Year10.index_to_date(1).unwrap();
    assert_eq!(date, Date::from(Year10::from(1_usize)));
}

#[test]
fn test_index_to_date_height_returns_none() {
    assert!(Index::Height.index_to_date(100).is_none());
}

#[test]
fn test_index_to_date_tx_index_returns_none() {
    assert!(Index::TxIndex.index_to_date(100).is_none());
}

#[test]
fn test_date_to_index_day1_zero() {
    assert_eq!(Index::Day1.date_to_index(Date::INDEX_ZERO), Some(0));
}

#[test]
fn test_date_to_index_day1_genesis() {
    assert_eq!(Index::Day1.date_to_index(Date::new(2009, 1, 3)), Some(2));
}

#[test]
fn test_date_to_index_roundtrip_day1() {
    let date = Index::Day1.index_to_date(100).unwrap();
    assert_eq!(Index::Day1.date_to_index(date), Some(100));
}

#[test]
fn test_date_to_index_roundtrip_week1() {
    let date = Index::Week1.index_to_date(50).unwrap();
    assert_eq!(Index::Week1.date_to_index(date), Some(50));
}

#[test]
fn test_date_to_index_roundtrip_month1() {
    let date = Index::Month1.index_to_date(24).unwrap();
    assert_eq!(Index::Month1.date_to_index(date), Some(24));
}

#[test]
fn test_date_to_index_roundtrip_year1() {
    let date = Index::Year1.index_to_date(5).unwrap();
    assert_eq!(Index::Year1.date_to_index(date), Some(5));
}

#[test]
fn test_date_to_index_roundtrip_month3() {
    let date = Index::Month3.index_to_date(4).unwrap();
    assert_eq!(Index::Month3.date_to_index(date), Some(4));
}

#[test]
fn test_date_to_index_roundtrip_month6() {
    let date = Index::Month6.index_to_date(2).unwrap();
    assert_eq!(Index::Month6.date_to_index(date), Some(2));
}

#[test]
fn test_date_to_index_roundtrip_year10() {
    let date = Index::Year10.index_to_date(1).unwrap();
    assert_eq!(Index::Year10.date_to_index(date), Some(1));
}

#[test]
fn test_date_to_index_pre_epoch_returns_none() {
    let pre_epoch = Date::new(2008, 12, 31);
    assert!(Index::Day1.date_to_index(pre_epoch).is_none());
    assert!(Index::Week1.date_to_index(pre_epoch).is_none());
    assert!(Index::Month1.date_to_index(pre_epoch).is_none());
}

#[test]
fn test_calendar_width_boundaries() {
    for (index, last, next, maximum) in [
        (
            Index::Month1,
            Date::new(7470, 4, 30),
            Date::new(7470, 5, 1),
            65_535,
        ),
        (
            Index::Month3,
            Date::new(2072, 12, 31),
            Date::new(2073, 1, 1),
            255,
        ),
        (
            Index::Month6,
            Date::new(2136, 12, 31),
            Date::new(2137, 1, 1),
            255,
        ),
        (
            Index::Year1,
            Date::new(2264, 12, 31),
            Date::new(2265, 1, 1),
            255,
        ),
        (
            Index::Year10,
            Date::new(4559, 12, 31),
            Date::new(4560, 1, 1),
            255,
        ),
    ] {
        assert_eq!(index.date_to_index(Date::INDEX_ZERO), Some(0));
        assert_eq!(index.date_to_index(last), Some(maximum), "{index:?}");
        assert_eq!(index.date_to_index(next), None, "{index:?}");
        assert_eq!(index.date_to_index(Date::new(2008, 12, 31)), None);
        assert_eq!(index.date_to_index(Date::new(9999, 12, 31)), None);
    }
    // Month1 must not narrow through Year1's smaller storage width.
    assert_eq!(
        Index::Month1.date_to_index(Date::new(2265, 1, 1)),
        Some(3072)
    );
    for day in 0..=65_535 {
        let day = Day1::from(day);
        let date = Date::from(day);
        assert_eq!(Month1::from(day), Month1::try_from(date).unwrap());
        assert_eq!(Year1::from(day), Year1::try_from(date).unwrap());
        assert_eq!(Year10::from(day), Year10::try_from(date).unwrap());
    }
}

#[test]
fn test_date_to_index_height_returns_none() {
    assert!(Index::Height.date_to_index(Date::INDEX_ZERO).is_none());
}
