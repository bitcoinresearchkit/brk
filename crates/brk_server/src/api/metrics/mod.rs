use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use brk_interface::{
    MetricCount, PaginatedMetrics, PaginationParam, Params, ParamsDeprec, ParamsOpt,
};
use brk_structs::{Index, IndexInfo};
use brk_traversable::TreeNode;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    VERSION,
    extended::{HeaderMapExtended, ResponseExtended, TransformResponseExtended},
};

use super::AppState;

mod data;

pub trait ApiMetricsRoutes {
    fn add_metrics_routes(self) -> Self;
}

#[derive(Deserialize, JsonSchema)]
struct MetricPath {
    /// Metric name
    #[schemars(example = &"price_close", example = &"market_cap", example = &"realized_price")]
    metric: String,
}

const TO_SEPARATOR: &str = "_to_";

impl ApiMetricsRoutes for ApiRouter<AppState> {
    fn add_metrics_routes(self) -> Self {
        self
            .route("/api/metrics", get(Redirect::temporary("/api#tag/metrics")))
            .api_route(
            "/api/metrics/count",
            get_with(
                async |State(app_state): State<AppState>| {
                    Json(app_state.interface.metric_count())
                },
                |op| {
                    op.tag("Metrics")
                        .summary("Metric count")
                        .description("Current metric count")
                        .with_ok_response::<Vec<MetricCount>, _>(|res| res)
                        .with_not_modified()
                },
            ),
        )
        .api_route(
            "/api/metrics/indexes",
            get_with(
                async |State(app_state): State<AppState>| {
                    Json(app_state.interface.get_indexes())
                },
                |op| {
                    op.tag("Metrics")
                        .summary("List available indexes")
                        .description(
                            "Returns all available indexes with their accepted query aliases. Use any alias when querying metrics."
                        )
                        .with_ok_response::<Vec<IndexInfo>, _>(|res| res)
                        .with_not_modified()
                },
            ),
        )
        .api_route(
            "/api/metrics/list",
            get_with(
                async |State(app_state): State<AppState>,
                       Query(pagination): Query<PaginationParam>| {
                    Json(app_state.interface.get_metrics(pagination))
                },
                |op| {
                    op.tag("Metrics")
                        .summary("Metrics list")
                        .description("Paginated list of available metrics")
                        .with_ok_response::<PaginatedMetrics, _>(|res| res)
                        .with_not_modified()
                },
            ),
        )
        .api_route(
            "/api/metrics/catalog",
            get_with(
                async |headers: HeaderMap, State(app_state): State<AppState>| -> Response {
                    let etag = VERSION;

                    if headers
                        .get_if_none_match()
                        .is_some_and(|prev_etag| etag == prev_etag)
                    {
                        return Response::new_not_modified();
                    }

                    let bytes = sonic_rs::to_vec(&app_state.interface.get_metrics_catalog()).unwrap();

                    let mut response = Response::builder()
                        .header("content-type", "application/json")
                        .body(bytes.into())
                        .unwrap();

                    let headers = response.headers_mut();
                    headers.insert_cors();
                    headers.insert_etag(etag);

                    response
                },
                |op| {
                    op.tag("Metrics")
                    .summary("Metrics catalog")
                    .description(
                        "Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories. Best viewed in an interactive JSON viewer (e.g., Firefox's built-in JSON viewer) for easy navigation of the nested structure."
                    )
                    .with_ok_response::<TreeNode, _>(|res| res)
                    .with_not_modified()
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
        .api_route(
            "/api/metrics/{metric}",
            get_with(
                async |
                    State(app_state): State<AppState>,
                    Path(MetricPath { metric }): Path<MetricPath>
                | {
                    match app_state.interface.metric_to_indexes(metric) {
                        Some(indexes) => Json(indexes).into_response(),
                        None => StatusCode::NOT_FOUND.into_response()
                    }
                },
                |op| {
                    op.tag("Metrics")
                        .summary("Get supported indexes for a metric")
                        .description(
                            "Returns the list of indexes are supported by the specified metric. \
                            For example, `realized_price` might be available on dateindex, weekindex, and monthindex."
                        )
                        .with_ok_response::<Vec<Index>, _>(|res| res)
                        .with_not_modified()
                        .with_not_found()
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
