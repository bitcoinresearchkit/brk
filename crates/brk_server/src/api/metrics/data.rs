use std::time::Duration;

use axum::{
    body::Body,
    extract::{Query, State},
    http::{HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use brk_types::{Format, MetricSelection, Output};
use quick_cache::sync::GuardResult;

use crate::{
    Result,
    api::metrics::{CACHE_CONTROL, MAX_WEIGHT},
    extended::HeaderMapExtended,
};

use super::AppState;

pub async fn handler(
    uri: Uri,
    headers: HeaderMap,
    Query(params): Query<MetricSelection>,
    State(AppState { query, cache, .. }): State<AppState>,
) -> Result<Response> {
    // Phase 1: Search and resolve metadata (cheap)
    let resolved = query.run(move |q| q.resolve(params, MAX_WEIGHT)).await?;

    let format = resolved.format();
    let etag = resolved.etag();

    // Check if client has fresh cache
    if headers.has_etag(etag.as_str()) {
        let response = (StatusCode::NOT_MODIFIED, "").into_response();
        return Ok(response);
    }

    // Check server-side cache
    let cache_key = format!("single-{}{}{}", uri.path(), uri.query().unwrap_or(""), etag);
    let guard_res = cache.get_value_or_guard(&cache_key, Some(Duration::from_millis(50)));

    let mut response = if let GuardResult::Value(v) = guard_res {
        Response::new(Body::from(v))
    } else {
        // Phase 2: Format (expensive, only on cache miss)
        let metric_output = query.run(move |q| q.format(resolved)).await?;

        match metric_output.output {
            Output::CSV(s) => {
                if let GuardResult::Guard(g) = guard_res {
                    let _ = g.insert(s.clone().into());
                }
                s.into_response()
            }
            Output::Json(v) => {
                if let GuardResult::Guard(g) = guard_res {
                    let _ = g.insert(v.clone().into());
                }
                Response::new(Body::from(v))
            }
        }
    };

    let headers = response.headers_mut();
    headers.insert_etag(etag.as_str());
    headers.insert_cache_control(CACHE_CONTROL);

    match format {
        Format::CSV => {
            headers.insert_content_disposition_attachment();
            headers.insert_content_type_text_csv()
        }
        Format::JSON => headers.insert_content_type_application_json(),
    }

    Ok(response)
}
