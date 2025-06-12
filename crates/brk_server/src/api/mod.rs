use std::collections::BTreeMap;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use brk_query::{Params, ParamsOpt};

use super::AppState;

mod explorer;
mod query;

pub use query::Bridge;

pub trait ApiRoutes {
    fn add_api_routes(self) -> Self;
}

impl ApiRoutes for Router<AppState> {
    fn add_api_routes(self) -> Self {
        self.route("/api/query", get(query::handler))
            .route("/api/vecs/id-count", get(id_count_handler))
            .route("/api/vecs/index-count", get(index_count_handler))
            .route("/api/vecs/variant-count", get(variant_count_handler))
            .route("/api/vecs/ids", get(ids_handler))
            .route("/api/vecs/indexes", get(indexes_handler))
            .route("/api/vecs/variants", get(variants_handler))
            .route("/api/vecs/id-to-indexes", get(id_to_indexes_handler))
            .route("/api/vecs/index-to-ids", get(index_to_ids_handler))
            .route("/api/{variant}", get(variant_handler))
            .route(
                "/api",
                get(|| async {
                    Redirect::temporary(
                        "https://github.com/bitcoinresearchkit/brk/tree/main/crates/brk_server#api",
                    )
                }),
            )
    }
}

pub async fn ids_handler(State(app_state): State<AppState>) -> Response {
    Json(
        app_state
            .query
            .vec_trees
            .id_to_index_to_vec
            .keys()
            .collect::<Vec<_>>(),
    )
    .into_response()
}

pub async fn variant_count_handler(State(app_state): State<AppState>) -> Response {
    Json(
        app_state
            .query
            .vec_trees
            .index_to_id_to_vec
            .values()
            .map(|tree| tree.len())
            .sum::<usize>(),
    )
    .into_response()
}

pub async fn id_count_handler(State(app_state): State<AppState>) -> Response {
    Json(app_state.query.vec_trees.id_to_index_to_vec.keys().count()).into_response()
}

pub async fn index_count_handler(State(app_state): State<AppState>) -> Response {
    Json(app_state.query.vec_trees.index_to_id_to_vec.keys().count()).into_response()
}

pub async fn indexes_handler(State(app_state): State<AppState>) -> Response {
    Json(
        app_state
            .query
            .vec_trees
            .index_to_id_to_vec
            .keys()
            .map(|i| (i.to_string().to_lowercase(), i.possible_values()))
            .collect::<BTreeMap<_, _>>(),
    )
    .into_response()
}

pub async fn variants_handler(State(app_state): State<AppState>) -> Response {
    Json(
        app_state
            .query
            .vec_trees
            .index_to_id_to_vec
            .iter()
            .flat_map(|(index, id_to_vec)| {
                let index_ser = index.serialize_long();
                id_to_vec
                    .keys()
                    .map(|id| format!("{}-to-{}", index_ser, id))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
    .into_response()
}

pub async fn id_to_indexes_handler(State(app_state): State<AppState>) -> Response {
    Json(app_state.query.vec_trees.serialize_id_to_index_to_vec()).into_response()
}

pub async fn index_to_ids_handler(State(app_state): State<AppState>) -> Response {
    Json(app_state.query.vec_trees.serialize_index_to_id_to_vec()).into_response()
}

const TO_SEPARATOR: &str = "-to-";

pub async fn variant_handler(
    headers: HeaderMap,
    Path(variant): Path<String>,
    Query(params_opt): Query<ParamsOpt>,
    state: State<AppState>,
) -> Response {
    let variant = variant.replace("_", "-");
    let mut split = variant.split(TO_SEPARATOR);
    let params = Params::from((
        (
            split.next().unwrap().to_string(),
            split.collect::<Vec<_>>().join(TO_SEPARATOR),
        ),
        params_opt,
    ));
    query::handler(headers, Query(params), state).await
}
