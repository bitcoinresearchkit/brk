use super::*;
#[test]
fn uses_catalog_and_prefers_static_paths() {
    let routes = catalog();
    assert_eq!(
        endpoint("/api/series/search?q=price", &routes),
        "/api/series/search"
    );
    assert_eq!(
        endpoint("/api/series/price_close/day1/data?start=1", &routes),
        "/api/series/{series}/{index}/data"
    );
    assert_eq!(
        endpoint("/api/tx/abc/status/", &routes),
        "/api/tx/{txid}/status"
    );
    assert_eq!(endpoint("/unknown?a=1", &routes), "/unknown");
}
