use super::*;

#[test]
fn fee_floor_rejects_invalid_values_before_integer_conversion() {
    for value in [
        -1.0,
        -f64::MIN_POSITIVE,
        f64::NAN,
        f64::INFINITY,
        f64::MAX,
        1e12,
    ] {
        assert!(matches!(Client::build_min_fee(value), Err(Error::Parse(_))));
    }
    for (value, milli) in [
        (0.0, 0),
        (0.00001, 1_000),
        (0.00002, 2_000),
        (0.0001014, 10_140),
    ] {
        assert_eq!(
            Client::build_min_fee(value).unwrap(),
            FeeRate::from_milli(milli)
        );
    }
}

#[test]
fn template_height_rejects_underflow_negative_and_truncating_values() {
    for height in [i64::MIN, -1, 0, i64::from(u32::MAX) + 2, i64::MAX] {
        assert!(matches!(
            Client::template_tip_height(height),
            Err(Error::Parse(_))
        ));
    }
    for (next, tip) in [
        (1, 0),
        (900_001, 900_000),
        (i64::from(u32::MAX) + 1, u32::MAX),
    ] {
        assert_eq!(
            Client::template_tip_height(next).unwrap(),
            Height::from(tip)
        );
    }
}
