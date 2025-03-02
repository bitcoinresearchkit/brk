use std::time::Instant;

use axum::{
    Json,
    extract::{Query as AxumQuery, State},
    http::{HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use brk_query::{Format, Index, Params};
use color_eyre::eyre::eyre;
use serde_json::Value;

use crate::{log_result, traits::HeaderMapExtended};

use super::AppState;

mod dts;

pub use dts::*;

pub async fn handler(
    headers: HeaderMap,
    uri: Uri,
    query: AxumQuery<Params>,
    State(app_state): State<AppState>,
) -> Response {
    let instant = Instant::now();

    let path = uri.path();

    match req_to_response_res(headers, query, app_state) {
        Ok(response) => {
            log_result(response.status(), path, instant);
            response
        }
        Err(error) => {
            let mut response = (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response();
            log_result(response.status(), path, instant);
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
    let indexes = index
        .to_lowercase()
        .split(",")
        .flat_map(|s| Index::try_from(s).ok())
        .collect::<Vec<_>>();

    if indexes.len() > 1 {
        return Err(eyre!("Multiple indexes aren't supported"));
    } else if indexes.is_empty() {
        return Err(eyre!("Unknown index"));
    }

    let ids = values
        .into_iter()
        .map(|v| v.to_lowercase())
        .flat_map(|v| v.split(",").map(|v| v.to_owned()).collect::<Vec<_>>())
        .map(|s| {
            let opt = query.vecid_to_index_to_vec.get(&s.replace("_", "-"));
            (s, opt)
        })
        .filter(|(_, opt)| opt.is_some())
        .map(|(id, vec)| (id, vec.unwrap()))
        .collect::<Vec<_>>();

    if ids.is_empty() {
        return Ok(Json(()).into_response());
    }

    let values = ids
        .iter()
        .flat_map(|(_, i_to_v)| i_to_v.get(indexes.first().unwrap()))
        .map(|vec| -> brk_vec::Result<Vec<Value>> { vec.collect_range_values(from, to) })
        .collect::<brk_vec::Result<Vec<_>>>()?;

    if ids.is_empty() {
        return Ok(Json(()).into_response());
    }

    let ids_last_i = ids.len() - 1;

    let mut response = match format {
        Some(Format::CSV) | Some(Format::TSV) => {
            let delimiter = if format == Some(Format::CSV) { ',' } else { '\t' };

            let mut csv = ids
                .into_iter()
                .map(|(id, _)| id)
                .collect::<Vec<_>>()
                .join(&delimiter.to_string());

            csv.push('\n');

            let values_len = values.first().unwrap().len();

            (0..values_len).for_each(|i| {
                let mut line = "".to_string();
                values.iter().enumerate().for_each(|(id_i, v)| {
                    line += &v.get(i).unwrap().to_string();
                    if id_i == ids_last_i {
                        line.push('\n');
                    } else {
                        line.push(delimiter);
                    }
                });
                csv += &line;
            });

            csv.into_response()
        }
        Some(Format::JSON) | None => {
            if values.len() == 1 {
                let values = values.first().unwrap();
                if values.len() == 1 {
                    let value = values.first().unwrap();
                    Json(value).into_response()
                } else {
                    Json(values).into_response()
                }
            } else {
                Json(values).into_response()
            }
        }
    };

    let headers = response.headers_mut();

    headers.insert_cors();
    // headers.insert_last_modified(date_modified);

    match format {
        Some(format) => {
            headers.insert_content_disposition_attachment();
            match format {
                Format::CSV => headers.insert_content_type_text_csv(),
                Format::TSV => headers.insert_content_type_text_tsv(),
                Format::JSON => headers.insert_content_type_application_json(),
            }
        }
        _ => headers.insert_content_type_application_json(),
    };

    Ok(response)
}
