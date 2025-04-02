use std::collections::BTreeMap;

use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Redirect, Response},
    routing::get,
};

use super::AppState;

mod explorer;
mod query;

pub use query::DTS;

pub trait ApiRoutes {
    fn add_api_routes(self) -> Self;
}

impl ApiRoutes for Router<AppState> {
    fn add_api_routes(self) -> Self {
        self.route(
            "/api",
            get(|| async {
                Redirect::permanent(
                    "https://github.com/bitcoinresearchkit/brk/tree/main/crates/brk_server#api",
                )
            }),
        )
        .route("/api/query", get(query::handler))
        .route("/api/vecs/ids", get(vecids_handler))
        .route("/api/vecs/indexes", get(vecindexes_handler))
        .route("/api/vecs/id-to-indexes", get(vecid_to_vecindexes_handler))
        .route("/api/vecs/index-to-ids", get(vecindex_to_vecids_handler))
    }
}

pub async fn vecids_handler(State(app_state): State<AppState>) -> Response {
    axum::Json(
        app_state
            .query
            .vec_trees
            .id_to_index_to_vec
            .keys()
            .collect::<Vec<_>>(),
    )
    .into_response()
}

pub async fn vecindexes_handler(State(app_state): State<AppState>) -> Response {
    axum::Json(
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

pub async fn vecid_to_vecindexes_handler(State(app_state): State<AppState>) -> Response {
    axum::Json(app_state.query.vec_trees.serialize_id_to_index_to_vec()).into_response()
}

pub async fn vecindex_to_vecids_handler(State(app_state): State<AppState>) -> Response {
    axum::Json(app_state.query.vec_trees.serialize_index_to_id_to_vec()).into_response()
}
