use axum::{
    Json,
    extract::{Query as AxumQuery, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use brk_query::{Format, Index, Output, Params};

use crate::traits::{HeaderMapExtended, ModifiedState, ResponseExtended};

use super::AppState;

mod dts;

pub use dts::*;

pub async fn handler(
    headers: HeaderMap,
    query: AxumQuery<Params>,
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
    AxumQuery(Params {
        format,
        from,
        index,
        to,
        values,
    }): AxumQuery<Params>,
    AppState { query, .. }: AppState,
) -> color_eyre::Result<Response> {
    let index = Index::try_from(index.as_str())?;

    let vecs = query.search(
        index,
        &values.iter().map(|v| v.as_str()).collect::<Vec<_>>(),
    );

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

    let output = query.format(vecs, from, to, format)?;

    let mut response = match output {
        Output::CSV(s) => s.into_response(),
        Output::TSV(s) => s.into_response(),
        Output::Json(v) => match v {
            brk_query::Value::Single(v) => Json(v).into_response(),
            brk_query::Value::List(l) => Json(l).into_response(),
            brk_query::Value::Matrix(m) => Json(m).into_response(),
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
