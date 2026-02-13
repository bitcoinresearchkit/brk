use std::net::SocketAddr;

use axum::{
    Extension,
    body::{Body, Bytes},
    extract::{Query, State},
    http::{HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use brk_types::{Format, MetricSelection, OutputLegacy};

use crate::{
    Result,
    api::metrics::{CACHE_CONTROL, max_weight},
    extended::HeaderMapExtended,
};

use super::AppState;

pub async fn handler(
    uri: Uri,
    headers: HeaderMap,
    Extension(addr): Extension<SocketAddr>,
    Query(params): Query<MetricSelection>,
    State(state): State<AppState>,
) -> Result<Response> {
    // Phase 1: Search and resolve metadata (cheap)
    let resolved = state.run(move |q| q.resolve(params, max_weight(&addr))).await?;

    let format = resolved.format();
    let etag = resolved.etag();

    if headers.has_etag(etag.as_str()) {
        return Ok((StatusCode::NOT_MODIFIED, "").into_response());
    }

    // Phase 2: Format (expensive, server-side cached)
    let cache_key = format!("legacy-{}{}{}", uri.path(), uri.query().unwrap_or(""), etag);
    let query = &state;
    let bytes = state
        .get_or_insert(&cache_key, async move {
            let out = query.run(move |q| q.format_legacy(resolved)).await?;
            Ok(match out.output {
                OutputLegacy::CSV(s) => Bytes::from(s),
                OutputLegacy::Json(v) => Bytes::from(v.to_vec()),
            })
        })
        .await?;

    let mut response = Response::new(Body::from(bytes));
    let h = response.headers_mut();
    h.insert_etag(etag.as_str());
    h.insert_cache_control(CACHE_CONTROL);
    match format {
        Format::CSV => {
            h.insert_content_disposition_attachment();
            h.insert_content_type_text_csv()
        }
        Format::JSON => h.insert_content_type_application_json(),
    }

    Ok(response)
}
