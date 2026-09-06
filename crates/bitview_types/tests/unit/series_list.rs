use super::*;

#[test]
fn normalized_count_is_bounded_for_strings_and_arrays() {
    for separator in [",", " ", "+"] {
        for count in [MAX_VECS, MAX_VECS + 1] {
            let names = vec!["price_close"; count].join(separator);
            for value in [Value::String(names.clone()), serde_json::json!([names])] {
                let parsed = serde_json::from_value::<SeriesList>(value);
                assert_eq!(
                    parsed.is_ok(),
                    count == MAX_VECS,
                    "count={count} separator={separator}"
                );
                if let Ok(parsed) = parsed {
                    assert_eq!(parsed.len(), count);
                }
            }
        }
    }
}

#[test]
fn normalization_preserves_order_duplicates_and_empty_filtering() {
    for value in [
        serde_json::json!(" ,PRICE-CLOSE++price_close,ΟΣ ΟΣ,İ---É,!!!"),
        serde_json::json!([
            " ,PRICE-CLOSE++price_close",
            "ΟΣ ΟΣ",
            "İ---É",
            "!!!",
            null,
            1
        ]),
    ] {
        let parsed: SeriesList = serde_json::from_value(value).unwrap();
        assert_eq!(parsed.to_string(), "price_close,price_close,ος,ος,i___é");
    }
}

#[test]
fn decoded_string_bytes_are_bounded_before_normalization() {
    for prefix in [
        "a".repeat(MAX_STRING_SIZE),
        "é".repeat(MAX_STRING_SIZE / 2),
        "!".repeat(MAX_STRING_SIZE),
    ] {
        for extra in ["", "a"] {
            let text = format!("{prefix}{extra}");
            for value in [
                Value::String(text.clone()),
                serde_json::json!([text]),
                serde_json::json!([prefix, extra]),
            ] {
                assert_eq!(
                    serde_json::from_value::<SeriesList>(value).is_ok(),
                    extra.is_empty()
                );
            }
        }
    }
}
