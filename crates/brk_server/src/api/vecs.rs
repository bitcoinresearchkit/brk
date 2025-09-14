use std::time::Duration;

use axum::{
    Json,
    body::Body,
    extract::{Query, State},
    http::{HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use brk_error::{Error, Result};
use brk_interface::{Format, Output, Params};
use quick_cache::sync::GuardResult;
use vecdb::Stamp;

use crate::{HeaderMapExtended, ResponseExtended};

use super::AppState;

const MAX_WEIGHT: usize = 65 * 10_000;

pub async fn handler(
    uri: Uri,
    headers: HeaderMap,
    query: Query<Params>,
    State(app_state): State<AppState>,
) -> Response {
    match req_to_response_res(uri, headers, query, app_state) {
        Ok(response) => response,
        Err(error) => {
            let mut response =
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response();
            response.headers_mut().insert_cors();
            response
        }
    }
}

fn req_to_response_res(
    uri: Uri,
    headers: HeaderMap,
    Query(params): Query<Params>,
    AppState {
        interface, cache, ..
    }: AppState,
) -> Result<Response> {
    let vecs = interface.search(&params)?;

    if vecs.is_empty() {
        return Ok(Json(vec![] as Vec<usize>).into_response());
    }

    let from = params.from();
    let to = params.to();
    let format = params.format();

    // TODO: From and to should be capped here

    let weight = vecs
        .iter()
        .map(|(_, v)| v.range_weight(from, to))
        .sum::<usize>();

    if weight > MAX_WEIGHT {
        return Err(Error::String(format!(
            "Request is too heavy, max weight is {MAX_WEIGHT} bytes"
        )));
    }

    // TODO: height should be from vec, but good enough for now
    let etag = vecs
        .first()
        .unwrap()
        .1
        .etag(Stamp::from(interface.get_height()), to);

    if headers
        .get_if_none_match()
        .is_some_and(|prev_etag| etag == prev_etag)
    {
        return Ok(Response::new_not_modified());
    }

    let guard_res = cache.get_value_or_guard(
        &format!("{}{}{etag}", uri.path(), uri.query().unwrap_or("")),
        Some(Duration::from_millis(50)),
    );

    let mut response = if let GuardResult::Value(v) = guard_res {
        Response::new(Body::from(v))
    } else {
        match interface.format(vecs, &params.rest)? {
            Output::CSV(s) | Output::TSV(s) | Output::MD(s) => {
                if let GuardResult::Guard(g) = guard_res {
                    g.insert(s.clone().into())
                        .map_err(|_| Error::QuickCacheError)?;
                }
                s.into_response()
            }
            Output::Json(v) => {
                let json = serde_json::to_vec(&v)?;
                if let GuardResult::Guard(g) = guard_res {
                    g.insert(json.clone().into())
                        .map_err(|_| Error::QuickCacheError)?;
                }
                json.into_response()
            }
        }
    };

    let headers = response.headers_mut();

    headers.insert_cors();

    headers.insert_etag(&etag);
    headers.insert_cache_control_must_revalidate();

    match format {
        Some(format) => {
            headers.insert_content_disposition_attachment();
            match format {
                Format::CSV => headers.insert_content_type_text_csv(),
                Format::MD => headers.insert_content_type_text_plain(),
                Format::TSV => headers.insert_content_type_text_tsv(),
                Format::JSON => headers.insert_content_type_application_json(),
            }
        }
        None => headers.insert_content_type_application_json(),
    };

    Ok(response)
}
