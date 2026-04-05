use std::net::SocketAddr;

use axum::{
    Extension,
    body::{Body, Bytes},
    extract::{Query, State},
    http::{HeaderMap, Uri},
    response::Response,
};
use brk_types::{Format, OutputLegacy, SeriesSelection};

use crate::{
    Result,
    api::series::{CACHE_CONTROL, max_weight},
    extended::{ContentEncoding, HeaderMapExtended, ResponseExtended},
};

pub const SUNSET: &str = "2027-01-01T00:00:00Z";

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
        return Ok(Response::new_not_modified(&etag, CACHE_CONTROL));
    }

    // Phase 2: Format (expensive, server-side cached)
    let encoding = ContentEncoding::negotiate(&headers);
    let cache_key = format!(
        "legacy-{}{}{}-{}",
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
                    let out = q.format_legacy(resolved)?;
                    let raw = match out.output {
                        OutputLegacy::CSV(s) => Bytes::from(s),
                        OutputLegacy::Json(v) => Bytes::from(v.to_vec()),
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
    h.insert_deprecation(SUNSET);

    Ok(response)
}
