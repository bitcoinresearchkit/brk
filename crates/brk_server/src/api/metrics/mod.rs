use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Uri},
    response::{IntoResponse, Response},
};
use brk_query::{
    DataRangeFormat, MetricSelection, MetricSelectionLegacy, PaginatedMetrics, Pagination,
};
use brk_traversable::TreeNode;
use brk_types::{
    Index, IndexInfo, LimitParam, Metric, MetricCount, MetricData, MetricParam, MetricWithIndex,
    Metrics,
};

use crate::{CacheStrategy, extended::TransformResponseExtended};

use super::AppState;

mod bulk;
mod data;
mod legacy;

/// Maximum allowed request weight in bytes (650KB)
const MAX_WEIGHT: usize = 65 * 10_000;

pub trait ApiMetricsRoutes {
    fn add_metrics_routes(self) -> Self;
}

impl ApiMetricsRoutes for ApiRouter<AppState> {
    fn add_metrics_routes(self) -> Self {
        self.api_route(
            "/api/metrics",
            get_with(
                async |headers: HeaderMap, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::Static, |q| Ok(q.metrics_catalog().clone())).await
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
                    state.cached_json(&headers, CacheStrategy::Static, |q| Ok(q.indexes().to_vec())).await
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
                    state.cached_json(&headers, CacheStrategy::Static, move |q| Ok(q.metrics(pagination))).await
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
            "/api/metrics/search/{metric}",
            get_with(
                async |
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Path(path): Path<MetricParam>,
                    Query(query): Query<LimitParam>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, move |q| Ok(q.match_metric(&path.metric, query.limit))).await
                },
                |op| op
                    .metrics_tag()
                    .summary("Search metrics")
                    .description("Fuzzy search for metrics by name. Supports partial matches and typos.")
                    .ok_response::<Vec<Metric>>()
                    .not_modified()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/metric/{metric}",
            get_with(
                async |
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Path(path): Path<MetricParam>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, move |q| {
                        if let Some(indexes) = q.metric_to_indexes(path.metric.clone()) {
                            return Ok(indexes.clone())
                        }
                        Err(q.metric_not_found_error(&path.metric))
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
                    .not_found()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/metric/{metric}/{index}",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       state: State<AppState>,
                       Path(path): Path<MetricWithIndex>,
                       Query(range): Query<DataRangeFormat>|
                       -> Response {
                    data::handler(
                        uri,
                        headers,
                        Query(MetricSelection::from((path.index, path.metric, range))),
                        state,
                    )
                    .await
                },
                |op| op
                    .metrics_tag()
                    .summary("Get metric data")
                    .description(
                        "Fetch data for a specific metric at the given index. \
                        Use query parameters to filter by date range and format (json/csv)."
                    )
                    .ok_response::<MetricData>()
                    .csv_response()
                    .not_modified()
                    .not_found(),
            ),
        )
        .api_route(
            "/api/metrics/bulk",
            get_with(
                bulk::handler,
                |op| op
                    .metrics_tag()
                    .summary("Bulk metric data")
                    .description(
                        "Fetch multiple metrics in a single request. Supports filtering by index and date range. \
                        Returns an array of MetricData objects."
                    )
                    .ok_response::<Vec<MetricData>>()
                    .csv_response()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/vecs/{variant}",
            get_with(
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
                    legacy::handler(uri, headers, Query(params), state).await
                },
                |op| op
                    .metrics_tag()
                    .summary("Legacy variant endpoint")
                    .description(
                        "**DEPRECATED** - Use `/api/metric/{metric}/{index}` instead.\n\n\
                        Sunset date: 2027-01-01. May be removed earlier in case of abuse.\n\n\
                        Legacy endpoint for querying metrics by variant path (e.g., `dateindex_to_price`). \
                        Returns raw data without the MetricData wrapper."
                    )
                    .deprecated()
                    .ok_response::<serde_json::Value>()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/vecs/query",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       Query(params): Query<MetricSelectionLegacy>,
                       state: State<AppState>|
                       -> Response {
                    legacy::handler(uri, headers, Query(params.into()), state).await
                },
                |op| op
                    .metrics_tag()
                    .summary("Legacy query endpoint")
                    .description(
                        "**DEPRECATED** - Use `/api/metric/{metric}/{index}` or `/api/metrics/bulk` instead.\n\n\
                        Sunset date: 2027-01-01. May be removed earlier in case of abuse.\n\n\
                        Legacy endpoint for querying metrics. Returns raw data without the MetricData wrapper."
                    )
                    .deprecated()
                    .ok_response::<serde_json::Value>()
                    .not_modified(),
            ),
        )
    }
}
