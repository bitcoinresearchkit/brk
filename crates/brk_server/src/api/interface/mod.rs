use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use brk_core::DateIndex;
use brk_interface::{Format, Output, Params};
use brk_vec::{CollectableVec, StoredVec};
use color_eyre::eyre::eyre;

use crate::traits::{HeaderMapExtended, ModifiedState, ResponseExtended};

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

    let weight = vecs
        .iter()
        .map(|(_, v)| {
            let len = v.len();
            let count = StoredVec::<DateIndex, usize>::range_count(from, to, len);
            count * v.value_type_to_size_of()
        })
        .sum::<usize>();

    if weight > MAX_WEIGHT {
        return Err(eyre!("Request is too heavy, max weight is {MAX_WEIGHT}"));
    }

    let mut date_modified_opt = None;

    if to.is_none() {
        let not_modified = vecs
            .iter()
            .map(|(_, vec)| headers.check_if_modified_since_(vec.modified_time()?))
            .all(|res| {
                res.ok().is_some_and(|(modified, date_modified)| {
                    if date_modified_opt.is_none_or(|dm| dm > date_modified) {
                        date_modified_opt.replace(date_modified);
                    }
                    modified == ModifiedState::NotModifiedSince
                })
            });

        if not_modified {
            return Ok(Response::new_not_modified());
        }
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

    if let Some(date_modified) = date_modified_opt {
        headers.insert_last_modified(date_modified);
    }

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
