use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::{HeaderMap, Uri},
    response::{IntoResponse, Response},
    routing::get,
};
use brk_interface::{Index, PaginatedIndexParam, PaginationParam, Params, ParamsDeprec, ParamsOpt};

use super::AppState;

mod data;

pub trait ApiMetricsRoutes {
    fn add_api_metrics_routes(self) -> Self;
}

const TO_SEPARATOR: &str = "_to_";

impl ApiMetricsRoutes for Router<AppState> {
    fn add_api_metrics_routes(self) -> Self {
        self.route(
            "/api/metrics/count",
            get(async |State(app_state): State<AppState>| -> Response {
                Json(sonic_rs::json!({
                    "distinct": app_state.interface.distinct_metric_count(),
                    "total": app_state.interface.total_metric_count(),
                }))
                .into_response()
            }),
        )
        .route(
            "/api/metrics/indexes",
            get(async |State(app_state): State<AppState>| -> Response {
                Json(app_state.interface.get_indexes()).into_response()
            }),
        )
        .route(
            "/api/metrics/list",
            get(
                async |State(app_state): State<AppState>,
                       Query(pagination): Query<PaginationParam>|
                       -> Response {
                    Json(app_state.interface.get_metrics(pagination)).into_response()
                },
            ),
        )
        // TODO:
        // .route(
        //     "/api/metrics/search",
        //     get(
        //         async |State(app_state): State<AppState>,
        //                Query(pagination): Query<PaginationParam>|
        //                -> Response {
        //             Json(app_state.interface.get_metrics(pagination)).into_response()
        //         },
        //     ),
        // )
        .route(
            "/api/metrics/{metric}",
            get(
                async |State(app_state): State<AppState>, Path(metric): Path<String>| -> Response {
                    // If not found do fuzzy search but here or in interface ?
                    Json(app_state.interface.metric_to_indexes(metric)).into_response()
                },
            ),
        )
        .route("/api/metrics/bulk", get(data::handler))
        .route(
            "/api/metrics/{metric}/{index}",
            get(
                async |uri: Uri,
                       headers: HeaderMap,
                       state: State<AppState>,
                       Path((metric, index)): Path<(String, Index)>,
                       Query(params_opt): Query<ParamsOpt>|
                       -> Response {
                    data::handler(
                        uri,
                        headers,
                        Query(Params::from(((index, metric), params_opt))),
                        state,
                    )
                    .await
                },
            ),
        )
        // !!!
        // DEPRECATED: Do not use
        // !!!
        .route(
            "/api/vecs/query",
            get(
                async |uri: Uri,
                       headers: HeaderMap,
                       Query(params): Query<ParamsDeprec>,
                       state: State<AppState>|
                       -> Response {
                    data::handler(uri, headers, Query(params.into()), state).await
                },
            ),
        )
        // !!!
        // DEPRECATED: Do not use
        // !!!
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

                    let ser_index = split.next().unwrap();
                    let Ok(index) = Index::try_from(ser_index) else {
                        return format!("Index {ser_index} doesn't exist").into_response();
                    };

                    let params = Params::from((
                        (index, split.collect::<Vec<_>>().join(TO_SEPARATOR)),
                        params_opt,
                    ));
                    data::handler(uri, headers, Query(params), state).await
                },
            ),
        )
    }
}
