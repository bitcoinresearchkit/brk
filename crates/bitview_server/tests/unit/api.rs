use serde_json::to_value;

use super::*;
use crate::finish_openapi;

#[test]
fn schema_routes_match_enabled_features() {
    let (_, spec) = finish_openapi(ApiRouter::new().add_api_routes());
    let spec = to_value(spec).unwrap();
    let paths = spec["paths"].as_object().unwrap();
    for (path, enabled) in [
        ("/health", true),
        ("/api/blocks", cfg!(feature = "chain")),
        ("/api/v1/historical-price", cfg!(feature = "chain")),
        ("/api/series", cfg!(feature = "series")),
        ("/api/urpd/{cohort}", cfg!(feature = "urpd")),
        ("/api/oracle/price", cfg!(feature = "price")),
    ] {
        assert_eq!(paths.contains_key(path), enabled, "{path}");
    }
}
