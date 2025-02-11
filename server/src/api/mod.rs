use std::time::Instant;

use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use color_eyre::eyre::eyre;
use reqwest::StatusCode;
use serde::Deserialize;
use structs::{Format, Index};

use crate::{log_result, traits::HeaderMapExtended};

use super::AppState;

// mod handlers;
pub mod structs;

pub const VECS_URL_PREFIX: &str = "/api/vecs";

pub trait ApiRoutes {
    fn add_api_routes(self) -> Self;
}

impl ApiRoutes for Router<AppState> {
    fn add_api_routes(self) -> Self {
        self.route(VECS_URL_PREFIX, get(handler))
    }
}

#[derive(Debug, Deserialize)]
pub struct DatasetParams {
    pub i: String,
    pub v: String,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub format: Option<String>,
}

pub async fn handler(headers: HeaderMap, query: Query<DatasetParams>, State(app_state): State<AppState>) -> Response {
    let instant = Instant::now();

    let path = format!(
        "{VECS_URL_PREFIX}?i={}&v={}{}{}",
        query.i,
        query.v,
        query.from.map_or("".to_string(), |from| format!("&from={from}")),
        query.to.map_or("".to_string(), |to| format!("&to={to}")),
    );

    match req_to_response_res(headers, query, app_state) {
        Ok(response) => {
            log_result(response.status(), &path, instant);
            response
        }
        Err(error) => {
            let mut response = (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response();
            log_result(response.status(), &path, instant);
            response.headers_mut().insert_cors();
            response
        }
    }
}

fn req_to_response_res(
    headers: HeaderMap,
    Query(DatasetParams { format, from, i, to, v }): Query<DatasetParams>,
    AppState { vecs, .. }: AppState,
) -> color_eyre::Result<Response> {
    let format = Format::try_from(format).ok();

    let indexes = i
        .to_lowercase()
        .split(",")
        .flat_map(|s| Index::try_from(s).ok())
        .collect::<Vec<_>>();

    if indexes.len() > 1 {
        return Err(eyre!("Multiple indexes aren't supported"));
    } else if indexes.is_empty() {
        return Err(eyre!("Unknown index"));
    }

    let values = v
        .to_lowercase()
        .split(",")
        .flat_map(|s| vecs.get(&s.replace("_", "-")))
        .flat_map(|i_to_v| i_to_v.get(indexes.first().unwrap()))
        .map(|vec| vec.collect_range(from, to).unwrap())
        .collect::<Vec<_>>();

    if values.len() == 1 {
        let values = values.first().unwrap();
        if values.len() == 1 {
            let value = values.first().unwrap();
            Ok(Json(value).into_response())
        } else {
            Ok(Json(values).into_response())
        }
    } else {
        Ok(Json(values).into_response())
    }
}
