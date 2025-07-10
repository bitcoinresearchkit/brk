use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use brk_interface::{Format, Output, Params};
use color_eyre::eyre::eyre;

use crate::traits::{HeaderMapExtended, ResponseExtended};

use super::AppState;

mod bridge;

pub use bridge::*;

const MAX_WEIGHT: usize = 320_000;

pub async fn handler(
    headers: HeaderMap,
    query: Query<Params>,
    State(app_state): State<AppState>,
) -> Response {
    match req_to_response_res(headers, query, app_state) {
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
    headers: HeaderMap,
    Query(params): Query<Params>,
    AppState { interface, .. }: AppState,
) -> color_eyre::Result<Response> {
    let vecs = interface.search(&params);

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
        return Err(eyre!("Request is too heavy, max weight is {MAX_WEIGHT}"));
    }

    // TODO: height should be from vec, but good enough for now
    let etag = vecs.first().unwrap().1.etag(interface.get_height(), to);

    if headers
        .get_if_none_match()
        .is_some_and(|prev_etag| etag == prev_etag)
    {
        return Ok(Response::new_not_modified());
    }

    let output = interface.format(vecs, &params.rest)?;

    let mut response = match output {
        Output::CSV(s) => s.into_response(),
        Output::TSV(s) => s.into_response(),
        Output::Json(v) => match v {
            brk_interface::Value::Single(v) => Json(v).into_response(),
            brk_interface::Value::List(v) => Json(v).into_response(),
            brk_interface::Value::Matrix(v) => Json(v).into_response(),
        },
        Output::MD(s) => s.into_response(),
    };

    let headers = response.headers_mut();

    headers.insert_cors();

    headers.insert_etag(&etag);

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
        _ => headers.insert_content_type_application_json(),
    };

    Ok(response)
}
