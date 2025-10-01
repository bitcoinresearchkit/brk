use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::{HeaderMap, Uri},
    response::{IntoResponse, Response},
    routing::get,
};
use brk_interface::{Index, PaginatedIndexParam, PaginationParam, Params, ParamsOpt};

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
                Json(app_state.interface.get_accepted_indexes()).into_response()
            }),
        )
        .route(
            "/api/vecs/metrics",
            get(
                async |State(app_state): State<AppState>,
                       Query(pagination): Query<PaginationParam>|
                       -> Response {
                    Json(app_state.interface.get_metrics(pagination)).into_response()
                },
            ),
        )
        .route(
            "/api/vecs/index-to-metrics",
            get(
                async |State(app_state): State<AppState>,
                       Query(paginated_index): Query<PaginatedIndexParam>|
                       -> Response {
                    Json(app_state.interface.get_index_to_vecids(paginated_index)).into_response()
                },
            ),
        )
        .route(
            "/api/metrics/{metric}",
            get(
                async |State(app_state): State<AppState>, Path(metric): Path<String>| -> Response {
                    // If not found do fuzzy search but here or in interface ?
                    Json(app_state.interface.metric_to_indexes(metric)).into_response()
                },
            ),
        )
        .route(
            "/api/metrics/{metric}/{index}",
            get(
                async |State(app_state): State<AppState>,
                       Path((metric, index)): Path<(String, Index)>|
                       -> Response {
                    // If not found do fuzzy search but here or in interface ?
                    Json(
                        format!("{metric}/{index}"), // app_state
                                                     //     .interface
                                                     //     .metric_to_indexes(metric.replace("-", "_")),
                    )
                    .into_response()
                },
            ),
        )
        // DEPRECATED
        .route("/api/vecs/query", get(data::handler))
        // DEPRECATED
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
                        data::handler(uri, headers, Query(params), state).await
                    } else {
                        "Bad variant".into_response()
                    }
                },
            ),
        )
    }
}
