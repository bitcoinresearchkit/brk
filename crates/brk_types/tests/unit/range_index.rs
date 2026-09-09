use schemars::schema_for;
use serde_json::{from_str, to_string, to_value};

use super::RangeIndex;

#[test]
fn dates_require_calendar_validation() {
    for date in ["2009-01-03", "2024-02-29", "9999-12-31"] {
        let value = to_string(date).unwrap();
        assert!(matches!(
            from_str::<RangeIndex>(&value),
            Ok(RangeIndex::Date(_))
        ));
    }
    for date in [
        "2026-02-29",
        "2026-02-30",
        "2026-02-31",
        "2023-02-29",
        "2009-02-30",
        "2009-00-01",
        "2009-01-00",
        "2009-04-31",
        "123é01-01",
    ] {
        let value = to_string(date).unwrap();
        assert!(from_str::<RangeIndex>(&value).is_err(), "{date}");
    }
}

#[test]
fn schema_matches_accepted_wire_forms() {
    let schema = to_value(schema_for!(RangeIndex)).expect("RangeIndex schema should serialize");
    let variants = schema["anyOf"]
        .as_array()
        .expect("RangeIndex schema should contain variants");

    assert_eq!(variants[0]["type"], "integer");
    assert_eq!(variants[1]["type"], "string");
    assert_eq!(variants[1]["format"], "date");
    assert_eq!(variants[2]["type"], "string");
    assert_eq!(variants[2]["format"], "date-time");
}
