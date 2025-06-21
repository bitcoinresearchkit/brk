use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use brk_interface::{Index, Pagination, Params, ParamsOpt};

use super::AppState;

mod explorer;
mod interface;

pub use interface::Bridge;

pub trait ApiRoutes {
    fn add_api_routes(self) -> Self;
}

const TO_SEPARATOR: &str = "-to-";

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
                       Query(pagination): Query<Pagination>|
                       -> Response {
                    Json(app_state.interface.get_vecids(pagination)).into_response()
                },
            ),
        )
        .route(
            "/api/vecs/indexes-to-ids",
            get(
                async |State(app_state): State<AppState>,
                       Query(pagination): Query<Pagination>|
                       -> Response {
                    Json(app_state.interface.get_indexes_to_vecids(pagination)).into_response()
                },
            ),
        )
        .route(
            "/api/vecs/ids-to-indexes",
            get(
                async |State(app_state): State<AppState>,
                       Query(pagination): Query<Pagination>|
                       -> Response {
                    Json(app_state.interface.get_vecids_to_indexes(pagination)).into_response()
                },
            ),
        )
        // .route("/api/vecs/variants", get(variants_handler))
        .route("/api/vecs/query", get(interface::handler))
        .route(
            "/api/vecs/{variant}",
            get(
                async |headers: HeaderMap,
                       Path(variant): Path<String>,
                       Query(params_opt): Query<ParamsOpt>,
                       state: State<AppState>|
                       -> Response {
                    let variant = variant.replace("_", "-");
                    let mut split = variant.split(TO_SEPARATOR);
                    let params = Params::from((
                        (
                            Index::try_from(split.next().unwrap()).unwrap(),
                            split.collect::<Vec<_>>().join(TO_SEPARATOR),
                        ),
                        params_opt,
                    ));
                    interface::handler(headers, Query(params), state).await
                },
            ),
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

// pub async fn variants_handler(State(app_state): State<AppState>) -> Response {
//     Json(
//         app_state
//             .query
//             .vec_trees
//             .index_to_id_to_vec
//             .iter()
//             .flat_map(|(index, id_to_vec)| {
//                 let index_ser = index.serialize_long();
//                 id_to_vec
//                     .keys()
//                     .map(|id| format!("{}-to-{}", index_ser, id))
//                     .collect::<Vec<_>>()
//             })
//             .collect::<Vec<_>>(),
//     )
//     .into_response()
// }
