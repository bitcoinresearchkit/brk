use brk_types::{Date, Day1, Height};
use serde_json::{json, to_value};

use super::BRK;

#[test]
fn cached_height_and_date_keep_chunk_offsets() {
    let mut source = BRK::new();
    let price = BRK::value_to_height_ohlc(&json!(12.34)).unwrap();
    source
        .height_to_ohlc
        .insert(Height::new(10_000), vec![price.clone(), price.clone()]);
    assert_eq!(
        to_value(source.get_from_height(Height::new(10_001)).unwrap()).unwrap(),
        json!([1234, 1234, 1234, 1234])
    );
    let date = Date::new(2024, 1, 1);
    let (key, offset) = BRK::day_chunk(Day1::try_from(date).unwrap());
    source.day1_to_ohlc.insert(key, vec![price; offset + 1]);
    assert_eq!(
        to_value(source.get_from_date(date).unwrap()).unwrap(),
        json!([1234, 1234, 1234, 1234])
    );
}

#[test]
fn current_price_shapes_preserve_units_and_candle_values() {
    let point = BRK::value_to_height_ohlc(&json!(12.34)).unwrap();
    assert_eq!(to_value(point).unwrap(), json!([1234, 1234, 1234, 1234]));
    let candle = BRK::value_to_ohlc(&json!([12.34, 15.0, 10.0, 13.0])).unwrap();
    assert_eq!(to_value(candle).unwrap(), json!([1234, 1500, 1000, 1300]));
    assert!(BRK::value_to_height_ohlc(&json!([12.34])).is_err());
    assert!(BRK::value_to_ohlc(&json!(12.34)).is_err());
}

#[test]
fn range_ends_do_not_wrap_at_native_index_limits() {
    let (height, _) = BRK::height_chunk(Height::from(u32::MAX));
    assert!(
        BRK::chunk_url("price", "height", u64::from(height))
            .ends_with("/price/height/data?start=4294960000&end=4294970000")
    );
    let (day, _) = BRK::day_chunk(Day1::from(u16::MAX as usize));
    assert!(
        BRK::chunk_url("price_ohlc", "day1", u64::from(day))
            .ends_with("/price_ohlc/day1/data?start=60000&end=70000")
    );
}

#[test]
fn malformed_prices_are_not_silently_clamped() {
    for value in [json!(-1.0), json!(1e300), json!(null), json!("12.34")] {
        assert!(BRK::value_to_height_ohlc(&value).is_err());
        assert!(BRK::value_to_ohlc(&json!([12.0, 13.0, 11.0, value])).is_err());
    }
    for value in [json!([]), json!([1, 2, 3]), json!([1, 2, 3, 4, 5])] {
        assert!(BRK::value_to_ohlc(&value).is_err());
    }
    assert!(BRK::value_to_height_ohlc(&json!(0)).is_ok());
}
