use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::{HeaderMap, Uri},
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use brk_interface::{IdParam, Index, PaginatedIndexParam, PaginationParam, Params, ParamsOpt};

use super::AppState;

mod explorer;
mod interface;

pub trait ApiRoutes {
    fn add_api_routes(self) -> Self;
}

const TO_SEPARATOR: &str = "_to_";

impl ApiRoutes for Router<AppState> {
    fn add_api_routes(self) -> Self {
        self.route(
            "/api/vecs/index-count",
            get(async |State(app_state): State<AppState>| -> Response {
                Json(app_state.interface.get_index_count()).into_response()
            }),
        )
        .route(
            "/api/vecs/id-count",
            get(async |State(app_state): State<AppState>| -> Response {
                Json(app_state.interface.get_vecid_count()).into_response()
            }),
        )
        .route(
            "/api/vecs/vec-count",
            get(async |State(app_state): State<AppState>| -> Response {
                Json(app_state.interface.get_vec_count()).into_response()
            }),
        )
        .route(
            "/api/vecs/indexes",
            get(async |State(app_state): State<AppState>| -> Response {
                Json(app_state.interface.get_indexes()).into_response()
            }),
        )
        .route(
            "/api/vecs/accepted-indexes",
            get(async |State(app_state): State<AppState>| -> Response {
                Json(app_state.interface.get_accepted_indexes()).into_response()
            }),
        )
        .route(
            "/api/vecs/ids",
            get(
                async |State(app_state): State<AppState>,
                       Query(pagination): Query<PaginationParam>|
                       -> Response {
                    Json(app_state.interface.get_vecids(pagination)).into_response()
                },
            ),
        )
        .route(
            "/api/vecs/index-to-ids",
            get(
                async |State(app_state): State<AppState>,
                       Query(paginated_index): Query<PaginatedIndexParam>|
                       -> Response {
                    Json(app_state.interface.get_index_to_vecids(paginated_index)).into_response()
                },
            ),
        )
        .route(
            "/api/vecs/id-to-indexes",
            get(
                async |State(app_state): State<AppState>,
                       Query(param): Query<IdParam>|
                       -> Response {
                    Json(app_state.interface.get_vecid_to_indexes(param.id)).into_response()
                },
            ),
        )
        // .route("/api/vecs/variants", get(variants_handler))
        .route("/api/vecs/query", get(interface::handler))
        .route(
            "/api/vecs/{variant}",
            get(
                async |uri: Uri,
                       headers: HeaderMap,
                       Path(variant): Path<String>,
                       Query(params_opt): Query<ParamsOpt>,
                       state: State<AppState>|
                       -> Response {
                    let variant = variant.replace("-", "_");
                    let mut split = variant.split(TO_SEPARATOR);

                    if let Ok(index) = Index::try_from(split.next().unwrap()) {
                        let params = Params::from((
                            (index, split.collect::<Vec<_>>().join(TO_SEPARATOR)),
                            params_opt,
                        ));
                        interface::handler(uri, headers, Query(params), state).await
                    } else {
                        "Bad variant".into_response()
                    }
                },
            ),
        )
        .route(
            "/health",
            get(|| async {
                Json(serde_json::json!({
                    "status": "healthy",
                    "service": "brk-server",
                    "timestamp": jiff::Timestamp::now().to_string()
                }))
            }),
        )
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
