use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Uri},
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use brk_query::{DataRangeFormat, MetricSelection, MetricSelectionLegacy, PaginatedMetrics, Pagination};
use brk_traversable::TreeNode;
use brk_types::{Index, IndexInfo, Limit, Metric, MetricCount, Metrics};

use crate::{CacheStrategy, extended::TransformResponseExtended};

use super::AppState;

mod data;

pub trait ApiMetricsRoutes {
    fn add_metrics_routes(self) -> Self;
}

impl ApiMetricsRoutes for ApiRouter<AppState> {
    fn add_metrics_routes(self) -> Self {
        self
            .route("/api/metric", get(Redirect::temporary("/api/metrics")))
            .route("/api/metrics", get(Redirect::temporary("/api#tag/metrics")))
            .api_route(
            "/api/metrics/count",
            get_with(
                async |
                    headers: HeaderMap,
                    State(state): State<AppState>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, |q| Ok(q.metric_count())).await
                },
                |op| op
                    .metrics_tag()
                    .summary("Metric count")
                    .description("Current metric count")
                    .ok_response::<Vec<MetricCount>>()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/metrics/indexes",
            get_with(
                async |
                    headers: HeaderMap,
                    State(state): State<AppState>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, |q| Ok(q.get_indexes().to_vec())).await
                },
                |op| op
                    .metrics_tag()
                    .summary("List available indexes")
                    .description(
                        "Returns all available indexes with their accepted query aliases. Use any alias when querying metrics."
                    )
                    .ok_response::<Vec<IndexInfo>>()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/metrics/list",
            get_with(
                async |
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Query(pagination): Query<Pagination>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, move |q| Ok(q.get_metrics(pagination))).await
                },
                |op| op
                    .metrics_tag()
                    .summary("Metrics list")
                    .description("Paginated list of available metrics")
                    .ok_response::<PaginatedMetrics>()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/metrics/catalog",
            get_with(
                async |headers: HeaderMap, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::Static, |q| Ok(q.get_metrics_catalog().clone())).await
                },
                |op| op
                    .metrics_tag()
                    .summary("Metrics catalog")
                    .description(
                        "Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories. Best viewed in an interactive JSON viewer (e.g., Firefox's built-in JSON viewer) for easy navigation of the nested structure."
                    )
                    .ok_response::<TreeNode>()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/metrics/search/{metric}",
            get_with(
                async |
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Path(metric): Path<Metric>,
                    Query(limit): Query<Limit>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, move |q| Ok(q.match_metric(&metric, limit))).await
                },
                |op| op
                    .metrics_tag()
                    .summary("Search metrics")
                    .description("Fuzzy search for metrics by name. Supports partial matches and typos.")
                    .ok_response::<Vec<String>>()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/metric/{metric}",
            get_with(
                async |
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Path(metric): Path<Metric>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, move |q| {
                        if let Some(indexes) = q.metric_to_indexes(metric.clone()) {
                            return Ok(indexes.clone())
                        }
                        Err(brk_error::Error::String(
                            if let Some(first) = q.match_metric(&metric, Limit::MIN).first() {
                                format!("Could not find '{metric}', did you mean '{first}' ?")
                            } else {
                                format!("Could not find '{metric}'.")
                            }
                        ))
                    }).await
                },
                |op| op
                    .metrics_tag()
                    .summary("Get supported indexes for a metric")
                    .description(
                        "Returns the list of indexes are supported by the specified metric. \
                        For example, `realized_price` might be available on dateindex, weekindex, and monthindex."
                    )
                    .ok_response::<Vec<Index>>()
                    .not_modified()
                    .not_found(),
            ),
        )
        // WIP
        .route("/api/metrics/bulk", get(data::handler))
        .route(
            "/api/metric/{metric}/{index}",
            get(
                async |uri: Uri,
                       headers: HeaderMap,
                       state: State<AppState>,
                       Path((metric, index)): Path<(Metric, Index)>,
                       Query(range): Query<DataRangeFormat>|
                       -> Response {
                    data::handler(
                        uri,
                        headers,
                        Query(MetricSelection::from((index, metric, range))),
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
                       Query(params): Query<MetricSelectionLegacy>,
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
                       Query(range): Query<DataRangeFormat>,
                       state: State<AppState>|
                       -> Response {
                    let separator = "_to_";
                    let variant = variant.replace("-", "_");
                    let mut split = variant.split(separator);

                    let ser_index = split.next().unwrap();
                    let Ok(index) = Index::try_from(ser_index) else {
                        return format!("Index {ser_index} doesn't exist").into_response();
                    };

                    let params = MetricSelection::from((
                        index,
                        Metrics::from(split.collect::<Vec<_>>().join(separator)),
                        range,
                    ));
                    data::handler(uri, headers, Query(params), state).await
                },
            ),
        )
    }
}
