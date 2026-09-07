use serde_json::{Value, json};

use super::{de_unquote_i64, de_unquote_usize};

#[test]
fn unquoted_numbers_keep_null_and_single_unquote_semantics() {
    for value in [
        Value::Null,
        json!(""),
        json!("null"),
        json!("\"\""),
        json!("\"null\""),
    ] {
        assert_eq!(de_unquote_i64(value.clone()).unwrap(), None);
        assert_eq!(de_unquote_usize(value).unwrap(), None);
    }
    for value in [json!(42), json!("42"), json!("\"42\""), json!("+42")] {
        assert_eq!(de_unquote_i64(value.clone()).unwrap(), Some(42));
        assert_eq!(de_unquote_usize(value).unwrap(), Some(42));
    }
    for value in [json!(-42), json!("-42"), json!("\"-42\"")] {
        assert_eq!(de_unquote_i64(value.clone()).unwrap(), Some(-42));
        assert!(de_unquote_usize(value).is_err());
    }
    for value in [
        json!(true),
        json!([]),
        json!({}),
        json!(1.5),
        json!(1.0),
        json!(" "),
        json!(" 42"),
        json!("42 "),
        json!("é"),
        json!("\""),
        json!("\"\"42\"\""),
        json!("18446744073709551616"),
    ] {
        assert!(de_unquote_i64(value.clone()).is_err(), "{value}");
        assert!(de_unquote_usize(value.clone()).is_err(), "{value}");
    }
}

#[test]
fn unquoted_numbers_keep_integer_boundaries_and_errors() {
    assert_eq!(de_unquote_i64(json!(i64::MIN)).unwrap(), Some(i64::MIN));
    assert_eq!(
        de_unquote_i64(json!(i64::MAX.to_string())).unwrap(),
        Some(i64::MAX)
    );
    assert_eq!(
        de_unquote_usize(json!(usize::MAX)).unwrap(),
        Some(usize::MAX)
    );
    assert_eq!(
        de_unquote_usize(json!(usize::MAX.to_string())).unwrap(),
        Some(usize::MAX)
    );
    assert_eq!(
        de_unquote_i64(json!(false)).unwrap_err().to_string(),
        "expected a string or number"
    );
    assert_eq!(
        de_unquote_usize(json!(-1)).unwrap_err().to_string(),
        "expected a string or number"
    );
}
