use brk_error::{SeriesNotFound, truncate_series_name};

#[test]
fn truncation_respects_utf8_boundaries_and_the_byte_limit() {
    for prefix in ["", "a", "ab", "abc", "abcd"] {
        for unit in ["a", "é", "€", "🦀"] {
            for count in 0..110 {
                let original = format!("{prefix}{}", unit.repeat(count));
                let result = truncate_series_name(original.clone());
                if original.len() <= 100 {
                    assert_eq!(result, original);
                } else {
                    let shortened = result.strip_suffix("...").unwrap();
                    assert!(original.starts_with(shortened));
                    assert!(shortened.len() <= 100);
                    let next = original[shortened.len()..].chars().next().unwrap();
                    assert!(shortened.len() + next.len_utf8() > 100);
                }
            }
        }
    }
}

#[test]
fn suggestions_keep_the_existing_display_format() {
    for (suggestions, total, expected) in [
        (vec![], 0, "'x' not found"),
        (vec!["a"], 1, "'x' not found, did you mean 'a'?"),
        (vec!["a", "b"], 2, "'x' not found, did you mean 'a', 'b'?"),
        (
            vec!["é"],
            3,
            "'x' not found, did you mean 'é'? (2 more — /api/series/search?q=x for all)",
        ),
    ] {
        assert_eq!(
            SeriesNotFound::new("x".into(), suggestions, total).to_string(),
            expected
        );
    }
}
