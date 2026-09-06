use std::{hint::black_box, time::Instant};

use axum::{
    body::Bytes,
    http::{
        HeaderMap, HeaderValue, StatusCode,
        header::{ETAG, IF_NONE_MATCH},
    },
    response::Response,
};
use bitview_types::SeriesInfo;
use brk_types::Index;

use crate::{CacheParams, extended::ResponseExtended};

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
#[ignore = "list response assembly comparison; excludes query dispatch and body serialization"]
fn benchmark_series_validators() {
    for (resource, prepared) in [
        ("list", list_params(&["ab", "c"])),
        (
            "search",
            search_params(b"catalog descriptions", &["ab", "c"]),
        ),
    ] {
        let bytes = Bytes::from_static(b"{}");
        for (case, conditional) in [
            ("absent", false),
            ("weak", true),
            ("strong", true),
            ("list", true),
            ("quoted-comma", true),
            ("multiple-fields", true),
            ("wildcard", true),
            ("miss", false),
            ("malformed", false),
        ] {
            let mut validators = [HeaderMap::new(), HeaderMap::new()];
            for (headers, params) in validators
                .iter_mut()
                .zip([CacheParams::deploy(), prepared.clone()])
            {
                params.etag.insert(headers);
                let tag = headers.remove(ETAG).unwrap();
                let tag = tag.to_str().unwrap();
                let value = match case {
                    "absent" => continue,
                    "weak" => tag.to_owned(),
                    "strong" => tag.strip_prefix("W/").unwrap().to_owned(),
                    "list" => format!("W/\"other\", {tag}"),
                    "quoted-comma" => format!("\"other,tag\", {tag}"),
                    "multiple-fields" => {
                        headers.append(IF_NONE_MATCH, HeaderValue::from_static("\"other\""));
                        tag.to_owned()
                    }
                    "wildcard" => "*".to_owned(),
                    "miss" => "\"other\"".to_owned(),
                    "malformed" => params.etag.as_str().to_owned(),
                    _ => unreachable!(),
                };
                headers.append(IF_NONE_MATCH, HeaderValue::from_str(&value).unwrap());
            }
            let mut times = [Vec::new(), Vec::new()];
            for round in 0..24 {
                for index in [round % 2, 1 - round % 2] {
                    let headers = &validators[index];
                    let started = Instant::now();
                    for _ in 0..1000 {
                        let params = if index == 0 {
                            CacheParams::deploy()
                        } else {
                            black_box(&prepared).clone()
                        };
                        let response =
                            Response::json_bytes(black_box(headers), &params, || bytes.clone());
                        assert_eq!(response.status() == StatusCode::NOT_MODIFIED, conditional);
                        black_box(response);
                    }
                    if round >= 4 {
                        times[index].push(started.elapsed() / 1000);
                    }
                }
            }
            for samples in &mut times {
                samples.sort_unstable();
            }
            eprintln!(
                "resource={resource} case={case}: deploy {:?}, prepared {:?}",
                times[0][10], times[1][10]
            );
        }
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
