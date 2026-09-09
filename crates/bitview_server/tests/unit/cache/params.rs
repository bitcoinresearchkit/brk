use super::*;

use axum::http::{HeaderValue, header::ETAG};

fn tag(params: &CacheParams) -> HeaderValue {
    let mut headers = HeaderMap::new();
    params.apply_to(&mut headers);
    headers.remove(ETAG).unwrap()
}

#[test]
fn deploy_revalidates_without_stale_reuse() {
    let params = CacheParams::deploy();
    assert_eq!(tag(&params), format!("W/\"d{VERSION}\""));
    assert_eq!(params.cache_control, "public, max-age=1, must-revalidate");
    assert_eq!(params.cdn_cache_control, params.cache_control);
}

#[test]
fn exact_live_tag_preserves_identity_without_stale_reuse() {
    let params = CacheParams::resolve(
        &CacheStrategy::Live("sync1-1-2-3".to_owned().into()),
        CdnCacheMode::Live,
    );
    assert_eq!(tag(&params), "W/\"sync1-1-2-3\"");
    assert_eq!(params.cache_control, "public, no-cache, must-revalidate");
    assert_eq!(
        params.cdn_cache_control,
        "public, max-age=1, must-revalidate"
    );
}

fn v(n: u32) -> Version {
    Version::new(n)
}

fn h(n: u64) -> BlockHashPrefix {
    BlockHashPrefix::from(n)
}

#[test]
fn activity_bound_has_a_distinct_etag_and_live_cdn_policy() {
    let p = CacheParams::activity_bound(v(1), h(0xabcd));
    assert_eq!(tag(&p), "W/\"a1-abcd\"");
    assert_eq!(p.cdn_cache_control, CDN_LIVE);
}

#[test]
fn live_hash_uses_hash_and_live_cdn_policy() {
    let p = CacheParams::resolve(&CacheStrategy::LiveHash(0xabcd), CdnCacheMode::Live);
    assert_eq!(tag(&p), "W/\"labcd\"");
    assert_eq!(p.cdn_cache_control, CDN_LIVE);
}

#[test]
fn series_tail_when_end_exceeds_stable_count() {
    let p = CacheParams::series(v(3), 0, 60, Some(50), h(0xabcd), CdnCacheMode::Live);
    assert_eq!(tag(&p), "W/\"s3-tabcd\"");
}

#[test]
fn series_historical_when_end_at_or_below_stable_count() {
    let p = CacheParams::series(v(3), 10, 50, Some(50), h(0xabcd), CdnCacheMode::Live);
    assert_eq!(tag(&p), "W/\"s3-h10-50\"");
}

#[test]
fn series_historical_ignores_tip_hash() {
    let a = CacheParams::series(v(3), 0, 50, Some(100), h(0xabcd), CdnCacheMode::Live);
    let b = CacheParams::series(v(3), 0, 50, Some(100), h(0xdead), CdnCacheMode::Live);
    assert_eq!(tag(&a), tag(&b));
}

#[test]
fn series_tail_changes_with_tip_hash() {
    let a = CacheParams::series(v(3), 0, 100, Some(50), h(0xabcd), CdnCacheMode::Live);
    let b = CacheParams::series(v(3), 0, 100, Some(50), h(0xdead), CdnCacheMode::Live);
    assert_ne!(tag(&a), tag(&b));
}

#[test]
fn series_mutable_class_always_tail() {
    let small = CacheParams::series(v(3), 0, 5, None, h(0xabcd), CdnCacheMode::Live);
    let large = CacheParams::series(v(3), 0, 1_000_000, None, h(0xabcd), CdnCacheMode::Live);
    assert_eq!(tag(&small), "W/\"s3-tabcd\"");
    assert_eq!(tag(&large), "W/\"s3-tabcd\"");
}

#[test]
fn series_at_stable_boundary_is_historical() {
    let p = CacheParams::series(v(3), 0, 50, Some(50), h(0xabcd), CdnCacheMode::Live);
    assert_eq!(tag(&p), "W/\"s3-h0-50\"");
}

#[test]
fn series_just_past_stable_boundary_is_tail() {
    let p = CacheParams::series(v(3), 0, 51, Some(50), h(0xabcd), CdnCacheMode::Live);
    assert_eq!(tag(&p), "W/\"s3-tabcd\"");
}

#[test]
fn series_different_ranges_get_different_etags() {
    let a = CacheParams::series(v(3), 0, 50, Some(100), h(0xabcd), CdnCacheMode::Live);
    let b = CacheParams::series(v(3), 10, 50, Some(100), h(0xabcd), CdnCacheMode::Live);
    assert_ne!(tag(&a), tag(&b));
}
