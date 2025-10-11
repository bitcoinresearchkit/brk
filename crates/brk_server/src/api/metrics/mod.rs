use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use brk_interface::{PaginatedMetrics, PaginationParam, Params, ParamsDeprec, ParamsOpt};
use brk_structs::{Index, IndexInfo, MetricCount, MetricPath, MetricSearchQuery};
use brk_traversable::TreeNode;

use crate::{
    VERSION,
    extended::{HeaderMapExtended, ResponseExtended, TransformResponseExtended},
};

use super::AppState;

mod data;

pub trait ApiMetricsRoutes {
    fn add_metrics_routes(self) -> Self;
}

impl ApiMetricsRoutes for ApiRouter<AppState> {
    fn add_metrics_routes(self) -> Self {
        self
            .route("/api/metrics", get(Redirect::temporary("/api#tag/metrics")))
            .api_route(
            "/api/metrics/count",
            get_with(
                async |
                    headers: HeaderMap,
                    State(state): State<AppState>
                | {
                    let etag = VERSION;
                    if headers.has_etag(etag) {
                        return Response::new_not_modified();
                    }
                    Response::new_json(state.metric_count(), etag)
                },
                |op| op.tag("Metrics")
                    .summary("Metric count")
                    .description("Current metric count")
                    .with_ok_response::<Vec<MetricCount>, _>(|res| res)
                    .with_not_modified(),
            ),
        )
        .api_route(
            "/api/metrics/indexes",
            get_with(
                async |
                    headers: HeaderMap,
                    State(state): State<AppState>
                | {
                    let etag = VERSION;
                    if headers.has_etag(etag) {
                        return Response::new_not_modified();
                    }
                    Response::new_json(state.get_indexes(), etag)
                },
                |op| op.tag("Metrics")
                    .summary("List available indexes")
                    .description(
                        "Returns all available indexes with their accepted query aliases. Use any alias when querying metrics."
                    )
                    .with_ok_response::<Vec<IndexInfo>, _>(|res| res)
                    .with_not_modified(),
            ),
        )
        .api_route(
            "/api/metrics/list",
            get_with(
                async |
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Query(pagination): Query<PaginationParam>
                | {
                    let etag = VERSION;
                    if headers.has_etag(etag) {
                        return Response::new_not_modified();
                    }
                    Response::new_json(state.get_metrics(pagination), etag)
                },
                |op| op.tag("Metrics")
                    .summary("Metrics list")
                    .description("Paginated list of available metrics")
                    .with_ok_response::<PaginatedMetrics, _>(|res| res)
                    .with_not_modified(),
            ),
        )
        .api_route(
            "/api/metrics/catalog",
            get_with(
                async |headers: HeaderMap, State(state): State<AppState>| -> Response {
                    let etag = VERSION;
                    if headers.has_etag(etag) {
                        return Response::new_not_modified();
                    }
                    Response::new_json(state.get_metrics_catalog(), etag)
                },
                |op| op.tag("Metrics")
                    .summary("Metrics catalog")
                    .description(
                        "Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories. Best viewed in an interactive JSON viewer (e.g., Firefox's built-in JSON viewer) for easy navigation of the nested structure."
                    )
                    .with_ok_response::<TreeNode, _>(|res| res)
                    .with_not_modified(),
            ),
        )
        .api_route(
            "/api/metrics/search",
            get_with(
                async |
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Query(query): Query<MetricSearchQuery>
                | {
                    let etag = VERSION;
                    if headers.has_etag(etag) {
                        return Response::new_not_modified();
                    }
                    Response::new_json(state.match_metric(query), etag)
                },
                |op| op.tag("Metrics")
                    .summary("Search metrics")
                    .description("Fuzzy search for metrics by name. Supports partial matches and typos.")
                    .with_ok_response::<Vec<String>, _>(|res| res)
                    .with_not_modified(),
            ),
        )
        .api_route(
            "/api/metrics/{metric}",
            get_with(
                async |
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Path(MetricPath { metric }): Path<MetricPath>
                | {
                    let etag = VERSION;
                    if headers.has_etag(etag) {
                        return Response::new_not_modified();
                    }
                    if let Some(indexes) = state.metric_to_indexes(metric.clone()) {
                        return Response::new_json(indexes, etag)
                    }
                    let value  = if let Some(first) = state.match_metric(MetricSearchQuery {
                        q: metric.clone(),
                        limit: 1,
                    }).first() {
                        format!("Could not find '{metric}', did you mean '{first}' ?")
                    } else {
                        format!("Could not find '{metric}'.")
                    };
                    Response::new_json_with(StatusCode::NOT_FOUND, value, etag)
                },
                |op| op.tag("Metrics")
                    .summary("Get supported indexes for a metric")
                    .description(
                        "Returns the list of indexes are supported by the specified metric. \
                        For example, `realized_price` might be available on dateindex, weekindex, and monthindex."
                    )
                    .with_ok_response::<Vec<Index>, _>(|res| res)
                    .with_not_modified()
                    .with_not_found(),
            ),
        )
        // WIP
        .route("/api/metrics/bulk", get(data::handler))
        // WIP
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
                    let separator = "_to_";
                    let variant = variant.replace("-", "_");
                    let mut split = variant.split(separator);

                    let ser_index = split.next().unwrap();
                    let Ok(index) = Index::try_from(ser_index) else {
                        return format!("Index {ser_index} doesn't exist").into_response();
                    };

                    let params = Params::from((
                        (index, split.collect::<Vec<_>>().join(separator)),
                        params_opt,
                    ));
                    data::handler(uri, headers, Query(params), state).await
                },
            ),
        )
    }
}
