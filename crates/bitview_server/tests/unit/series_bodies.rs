use axum::http::{
    HeaderMap,
    header::{ETAG, IF_NONE_MATCH},
};
use bitview_types::SeriesInfo;
use brk_types::Index;

use super::{info_params, list_params, search_params};

#[test]
fn info_revision_binds_names_and_serialized_metadata() {
    let make = || SeriesInfo {
        description: Some("description".into()),
        indexes: vec![Index::Height],
        value_type: "u64".into(),
    };
    let first = info_params([("name", make())]);
    let mut headers = HeaderMap::new();
    first.etag.insert(&mut headers);
    let tag = headers.remove(ETAG).unwrap();
    headers.insert(IF_NONE_MATCH, tag);
    assert!(info_params([("name", make())]).matches_etag(&headers));
    assert!(!info_params([("renamed", make())]).matches_etag(&headers));
    assert!(!info_params([]).matches_etag(&headers));
    for field in 0..3 {
        let mut info = make();
        match field {
            0 => info.description = None,
            1 => info.indexes.push(Index::Day1),
            _ => info.value_type = "f64".into(),
        }
        assert!(!info_params([("name", info)]).matches_etag(&headers));
    }
}

#[test]
fn search_revision_binds_descriptions_and_names() {
    let first = search_params(b"description", &["ab", "c"]);
    let mut headers = HeaderMap::new();
    first.etag.insert(&mut headers);
    let tag = headers.remove(ETAG).unwrap();
    headers.insert(IF_NONE_MATCH, tag);
    assert!(search_params(b"description", &["ab", "c"]).matches_etag(&headers));
    for changed in [
        search_params(b"changed", &["ab", "c"]),
        search_params(b"description", &["c", "ab"]),
        search_params(b"description", &["ab"]),
        search_params(b"description", &["a", "bc"]),
    ] {
        assert!(!changed.matches_etag(&headers));
    }
}

#[test]
fn list_revision_binds_order_and_unambiguous_names() {
    let first = list_params(&["ab", "c"]);
    let mut headers = HeaderMap::new();
    first.etag.insert(&mut headers);
    let tag = headers.remove(ETAG).unwrap();
    headers.insert(IF_NONE_MATCH, tag);
    assert!(list_params(&["ab", "c"]).matches_etag(&headers));
    for names in [&["a", "bc"][..], &["c", "ab"], &["ab"], &[]] {
        assert!(!list_params(names).matches_etag(&headers));
    }
}
