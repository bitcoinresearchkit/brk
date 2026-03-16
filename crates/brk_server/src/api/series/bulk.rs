use std::net::SocketAddr;

use axum::{
    Extension,
    body::{Body, Bytes},
    extract::{Query, State},
    http::{HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use brk_types::{Format, Output, SeriesSelection};

use crate::{
    Result,
    api::series::{CACHE_CONTROL, max_weight},
    extended::{ContentEncoding, HeaderMapExtended},
};

use super::AppState;

pub async fn handler(
    uri: Uri,
    headers: HeaderMap,
    Extension(addr): Extension<SocketAddr>,
    Query(params): Query<SeriesSelection>,
    State(state): State<AppState>,
) -> Result<Response> {
    // Phase 1: Search and resolve metadata (cheap)
    let resolved = state
        .run(move |q| q.resolve(params, max_weight(&addr)))
        .await?;

    let format = resolved.format();
    let etag = resolved.etag();
    let csv_filename = resolved.csv_filename();

    if headers.has_etag(etag.as_str()) {
        return Ok((StatusCode::NOT_MODIFIED, "").into_response());
    }

    // Phase 2: Format (expensive, server-side cached)
    let encoding = ContentEncoding::negotiate(&headers);
    let cache_key = format!(
        "bulk-{}{}{}-{}",
        uri.path(),
        uri.query().unwrap_or(""),
        etag,
        encoding.as_str()
    );
    let query = &state;
    let bytes = state
        .get_or_insert(&cache_key, async move {
            query
                .run(move |q| {
                    let out = q.format(resolved)?;
                    let raw = match out.output {
                        Output::CSV(s) => Bytes::from(s),
                        Output::Json(v) => Bytes::from(v),
                    };
                    Ok(encoding.compress(raw))
                })
                .await
        })
        .await?;

    let mut response = Response::new(Body::from(bytes));
    let h = response.headers_mut();
    h.insert_etag(etag.as_str());
    h.insert_cache_control(CACHE_CONTROL);
    h.insert_content_encoding(encoding);
    match format {
        Format::CSV => {
            h.insert_content_disposition_attachment(&csv_filename);
            h.insert_content_type_text_csv();
        }
        Format::JSON => h.insert_content_type_application_json(),
    }

    Ok(response)
}
