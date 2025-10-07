use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::{HeaderMap, Uri},
    response::{IntoResponse, Response},
    routing::get,
};
use brk_interface::{
    Index, Indexes, PaginatedMetrics, PaginationParam, Params, ParamsDeprec, ParamsOpt,
};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    VERSION,
    extended::{HeaderMapExtended, ResponseExtended},
};

use super::AppState;

mod data;

pub trait ApiMetricsRoutes {
    fn add_api_metrics_routes(self) -> Self;
}

const TO_SEPARATOR: &str = "_to_";

#[derive(Debug, Serialize, JsonSchema)]
/// Metric count statistics - distinct metrics and total metric-index combinations
struct MetricCount {
    #[schemars(example = 3141)]
    /// Number of unique metrics available (e.g., realized_price, market_cap)
    distinct_metrics: usize,
    #[schemars(example = 21000)]
    /// Total number of metric-index combinations across all timeframes
    total_endpoints: usize,
}

impl ApiMetricsRoutes for ApiRouter<AppState> {
    fn add_api_metrics_routes(self) -> Self {
        self.api_route(
            "/api/metrics/count",
            get_with(
                async |State(app_state): State<AppState>| -> Json<MetricCount> {
                    Json(MetricCount {
                        distinct_metrics: app_state.interface.distinct_metric_count(),
                        total_endpoints: app_state.interface.total_metric_count(),
                    })
                },
                |op| {
                    op.tag("Metrics")
                        .summary("Metric count")
                        .description("Current metric count")
                },
            ),
        )
        .api_route(
            "/api/metrics/indexes",
            get_with(
                async |State(app_state): State<AppState>| -> Json<&Indexes> {
                    Json(app_state.interface.get_indexes())
                },
                |op| {
                    op.tag("Metrics")
                        .summary("Metric indexes")
                        .description("Available metric indexes and their accepted variants")
                },
            ),
        )
        .api_route(
            "/api/metrics/list",
            get_with(
                async |State(app_state): State<AppState>,
                       Query(pagination): Query<PaginationParam>|
                       -> Json<PaginatedMetrics> {
                    Json(app_state.interface.get_metrics(pagination))
                },
                |op| {
                    op.tag("Metrics")
                        .summary("Metrics list")
                        .description("Paginated list of available metrics")
                },
            ),
        )
        .route(
            "/api/metrics/catalog",
            get(
                async |headers: HeaderMap, State(app_state): State<AppState>| -> Response {
                    let etag = VERSION;

                    if headers
                        .get_if_none_match()
                        .is_some_and(|prev_etag| etag == prev_etag)
                    {
                        return Response::new_not_modified();
                    }

                    let mut response =
                        Json(app_state.interface.get_metrics_catalog()).into_response();

                    let headers = response.headers_mut();
                    headers.insert_cors();
                    headers.insert_etag(etag);

                    response
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
