use super::*;

#[test]
fn date_conversion_rejects_indices_outside_u16() {
    for days in [-1_i64, 0, 2, 65_535, 65_536, 65_538] {
        let date = Date::from(
            Date::INDEX_ZERO_
                .checked_add(Span::new().days(days))
                .unwrap(),
        );
        match Day1::try_from(date) {
            Ok(index) => assert_eq!(i64::from(index), days),
            Err(Error::UnindexableDate) => assert!(!(0..=65_535).contains(&days)),
            result => panic!("unexpected conversion: {result:?}"),
        }
    }
}
