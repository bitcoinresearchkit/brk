use super::*;

#[test]
#[should_panic(expected = "invalid ETag token")]
fn construction_rejects_embedded_quotes() {
    let _ = Etag::from("invalid\"tag".to_owned());
}

fn headers(values: &[&'static str]) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for value in values {
        headers.append(IF_NONE_MATCH, HeaderValue::from_static(value));
    }
    headers
}

#[test]
fn matches_weak_strong_wildcard_and_list() {
    let etag = Etag::from("s1-abc".to_string());
    assert!(etag.matches(&headers(&["W/\"s1-abc\""])));
    assert!(etag.matches(&headers(&["\"s1-abc\""])));
    assert!(etag.matches(&headers(&["*"])));
    assert!(etag.matches(&headers(&["W/\"a\", W/\"s1-abc\""])));
    assert!(etag.matches(&headers(&["  W/\"s1-abc\"  "])));
}

#[test]
fn checks_every_if_none_match_field() {
    let etag = Etag::from("s1-abc".to_string());
    assert!(etag.matches(&headers(&["W/\"other\"", "W/\"s1-abc\""])));
}

#[test]
fn rejects_mismatch_and_missing() {
    let etag = Etag::from("s1-abc".to_string());
    assert!(!etag.matches(&headers(&["W/\"other\""])));
    assert!(!etag.matches(&HeaderMap::new()));
}

#[test]
fn commas_inside_tags_are_not_list_separators() {
    let etag = Etag::from("s1-abc".to_string());
    assert!(!etag.matches(&headers(&["\"other,s1-abc,other\""])));
    assert!(!etag.matches(&headers(&["s1-abc"])));
    assert!(!etag.matches(&headers(&["W/s1-abc"])));
    assert!(etag.matches(&headers(&["\"other,tag\", W/\"s1-abc\""])));
    let comma_tag = Etag::from("a,b".to_string());
    assert!(comma_tag.matches(&headers(&["W/\"a,b\""])));
}

#[test]
fn inserts_exact_weak_header() {
    let etag = Etag::from("s1-abc".to_string());
    let mut headers = HeaderMap::new();
    etag.insert(&mut headers);
    assert_eq!(
        headers.get(ETAG),
        Some(&HeaderValue::from_static("W/\"s1-abc\""))
    );
}
