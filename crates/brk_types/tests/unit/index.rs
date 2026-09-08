use super::*;

#[test]
fn calendar_dispatch_matches_known_dates() {
    for (index, value, date) in [
        (Index::Day1, 0, Date::new(2009, 1, 1)),
        (Index::Day1, 2, Date::new(2009, 1, 3)),
        (Index::Week1, 1, Date::new(2009, 1, 8)),
        (Index::Month1, 12, Date::new(2010, 1, 1)),
        (Index::Month3, 4, Date::new(2010, 1, 1)),
        (Index::Month6, 2, Date::new(2010, 1, 1)),
        (Index::Year1, 5, Date::new(2014, 1, 1)),
        (Index::Year10, 1, Date::new(2019, 1, 1)),
    ] {
        assert!(index.is_date_based());
        assert_eq!(index.index_to_date(value), Some(date), "{index:?}");
        assert_eq!(index.date_to_index(date), Some(value), "{index:?}");
        assert_eq!(index.date_to_index(Date::new(2008, 12, 31)), None);
    }
    for index in [Index::Height, Index::TxIndex] {
        assert!(!index.is_date_based());
        assert_eq!(index.index_to_date(100), None);
        assert_eq!(index.date_to_index(Date::INDEX_ZERO), None);
    }
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
