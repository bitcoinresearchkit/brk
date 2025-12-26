use std::time::Duration;

use axum::{
    body::Body,
    extract::{Query, State},
    http::{HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use brk_query::{MetricSelection, Output};
use brk_types::Format;
use quick_cache::sync::GuardResult;

use crate::{
    CacheStrategy, api::metrics::MAX_WEIGHT, cache::CacheParams, extended::HeaderMapExtended,
};

use super::AppState;

pub async fn handler(
    uri: Uri,
    headers: HeaderMap,
    query: Query<MetricSelection>,
    State(state): State<AppState>,
) -> Response {
    match req_to_response_res(uri, headers, query, state).await {
        Ok(response) => response,
        Err(error) => {
            let mut response =
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response();
            response.headers_mut().insert_cors();
            response
        }
    }
}

async fn req_to_response_res(
    uri: Uri,
    headers: HeaderMap,
    Query(params): Query<MetricSelection>,
    AppState { query, cache, .. }: AppState,
) -> brk_error::Result<Response> {
    let format = params.format();
    let height = query.sync(|q| q.height());

    let cache_params =
        CacheParams::resolve(&CacheStrategy::height_with(params.etag_suffix()), || {
            height.into()
        });

    if cache_params.matches_etag(&headers) {
        let mut response = (StatusCode::NOT_MODIFIED, "").into_response();
        response.headers_mut().insert_cors();
        return Ok(response);
    }

    let cache_key = format!(
        "single-{}{}{}",
        uri.path(),
        uri.query().unwrap_or(""),
        cache_params.etag_str()
    );
    let guard_res = cache.get_value_or_guard(&cache_key, Some(Duration::from_millis(50)));

    let mut response = if let GuardResult::Value(v) = guard_res {
        Response::new(Body::from(v))
    } else {
        match query
            .run(move |q| q.search_and_format_checked(params, MAX_WEIGHT))
            .await?
        {
            Output::CSV(s) => {
                if let GuardResult::Guard(g) = guard_res {
                    let _ = g.insert(s.clone().into());
                }
                s.into_response()
            }
            Output::Json(v) => {
                let json = v.to_vec();
                if let GuardResult::Guard(g) = guard_res {
                    let _ = g.insert(json.clone().into());
                }
                json.into_response()
            }
        }
    };

    let headers = response.headers_mut();
    headers.insert_cors();
    if let Some(etag) = &cache_params.etag {
        headers.insert_etag(etag);
    }
    headers.insert_cache_control(&cache_params.cache_control);

    match format {
        Format::CSV => {
            headers.insert_content_disposition_attachment();
            headers.insert_content_type_text_csv()
        }
        Format::JSON => headers.insert_content_type_application_json(),
    }

    Ok(response)
}
