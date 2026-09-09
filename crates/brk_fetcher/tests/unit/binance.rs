use serde_json::{json, to_value};

use super::*;

#[test]
fn cached_dates_and_missing_dates_do_not_fetch() {
    let mut source = Binance::new(None);
    let date = Date::new(2024, 1, 1);
    let later = Date::new(2024, 1, 3);
    source._1d = Some(BTreeMap::from([
        (date, OHLCCents::default()),
        (later, OHLCCents::default()),
    ]));
    assert!(source.get_from_1d(&date).is_ok());
    assert!(matches!(source.get_from_1d(&Date::new(2024, 1, 2)),
        Err(Error::NotFound(message)) if message == "Couldn't find date"));
}

#[test]
fn parsing_preserves_order_duplicates_and_errors() {
    let rows = json!([
        [2000, "2", "3", "1", "2"],
        null,
        [1000, "1", "2", "0.5", "1"],
        [2000, "4", "5", "3", "4"]
    ]);
    let map = Binance::parse_ohlc_array(&rows).unwrap();
    assert_eq!(
        map.keys().copied().collect::<Vec<_>>(),
        [Timestamp::new(1), Timestamp::new(2)]
    );
    assert_eq!(
        to_value(&map[&Timestamp::new(2)]).unwrap(),
        json!([400, 500, 300, 400])
    );
    let daily = Binance::parse_date_ohlc_array(&rows).unwrap();
    assert_eq!(daily.len(), 1);
    assert_eq!(
        to_value(daily.values().next().unwrap()).unwrap(),
        json!([400, 500, 300, 400])
    );
    assert!(Binance::parse_ohlc_array(&json!([])).unwrap().is_empty());
    for bad in [json!(null), json!({}), json!(1)] {
        assert!(
            matches!(Binance::parse_ohlc_array(&bad), Err(Error::Parse(message)) if message == "Expected JSON array")
        );
    }
}
