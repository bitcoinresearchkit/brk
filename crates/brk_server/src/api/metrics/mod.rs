use std::net::SocketAddr;

use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    Extension,
    extract::{Path, Query, State},
    http::{HeaderMap, Uri},
    response::{IntoResponse, Response},
};
use brk_traversable::TreeNode;
use brk_types::{
    CostBasisCohortParam, CostBasisFormatted, CostBasisParams, CostBasisQuery, DataRangeFormat,
    Date, Index, IndexInfo, Metric, MetricCount, MetricData, MetricInfo, MetricParam,
    MetricSelection, MetricSelectionLegacy, MetricWithIndex, Metrics, PaginatedMetrics, Pagination,
    SearchQuery,
};

use crate::{CacheStrategy, Error, extended::TransformResponseExtended};

use super::AppState;

mod bulk;
mod data;
mod legacy;

/// Maximum allowed request weight in bytes (650KB)
const MAX_WEIGHT: usize = 65 * 10_000;
/// Maximum allowed request weight for localhost (50MB)
const MAX_WEIGHT_LOCALHOST: usize = 50 * 1_000_000;
/// Cache control header for metric data responses
const CACHE_CONTROL: &str = "public, max-age=1, must-revalidate";

/// Returns the max weight for a request based on the client address.
/// Localhost requests get a generous limit, external requests get a stricter one.
fn max_weight(addr: &SocketAddr) -> usize {
    if addr.ip().is_loopback() {
        MAX_WEIGHT_LOCALHOST
    } else {
        MAX_WEIGHT
    }
}

pub trait ApiMetricsRoutes {
    fn add_metrics_routes(self) -> Self;
}

impl ApiMetricsRoutes for ApiRouter<AppState> {
    fn add_metrics_routes(self) -> Self {
        self.api_route(
            "/api/metrics",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::Static, &uri, |q| Ok(q.metrics_catalog().clone())).await
                },
                |op| op
                    .id("get_metrics_tree")
                    .metrics_tag()
                    .summary("Metrics catalog")
                    .description(
                        "Returns the complete hierarchical catalog of available metrics organized as a tree structure. \
                        Metrics are grouped by categories and subcategories."
                    )
                    .ok_response::<TreeNode>()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/metrics/count",
            get_with(
                async |
                    uri: Uri,
                    headers: HeaderMap,
                    State(state): State<AppState>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, &uri, |q| Ok(q.metric_count())).await
                },
                |op| op
                    .id("get_metrics_count")
                    .metrics_tag()
                    .summary("Metric count")
                    .description("Returns the number of metrics available per index type.")
                    .ok_response::<Vec<MetricCount>>()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/metrics/indexes",
            get_with(
                async |
                    uri: Uri,
                    headers: HeaderMap,
                    State(state): State<AppState>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, &uri, |q| Ok(q.indexes().to_vec())).await
                },
                |op| op
                    .id("get_indexes")
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
                    uri: Uri,
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Query(pagination): Query<Pagination>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, &uri, move |q| Ok(q.metrics(pagination))).await
                },
                |op| op
                    .id("list_metrics")
                    .metrics_tag()
                    .summary("Metrics list")
                    .description("Paginated flat list of all available metric names. Use `page` query param for pagination.")
                    .ok_response::<PaginatedMetrics>()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/metrics/search",
            get_with(
                async |
                    uri: Uri,
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Query(query): Query<SearchQuery>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, &uri, move |q| Ok(q.search_metrics(&query))).await
                },
                |op| op
                    .id("search_metrics")
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
                    uri: Uri,
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Path(path): Path<MetricParam>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, &uri, move |q| {
                        q.metric_info(&path.metric).ok_or_else(|| q.metric_not_found_error(&path.metric))
                    }).await
                },
                |op| op
                    .id("get_metric_info")
                    .metrics_tag()
                    .summary("Get metric info")
                    .description(
                        "Returns the supported indexes and value type for the specified metric."
                    )
                    .ok_response::<MetricInfo>()
                    .not_modified()
                    .not_found()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/metric/{metric}/{index}/latest",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       State(state): State<AppState>,
                       Path(path): Path<MetricWithIndex>| {
                    state
                        .cached_json(&headers, CacheStrategy::Height, &uri, move |q| {
                            q.latest(&path.metric, path.index)
                        })
                        .await
                },
                |op| op
                    .id("get_metric_latest")
                    .metrics_tag()
                    .summary("Get latest metric value")
                    .description(
                        "Returns the single most recent value for a metric, unwrapped (not inside a MetricData object)."
                    )
                    .ok_response::<serde_json::Value>()
                    .not_found(),
            ),
        )
        .api_route(
            "/api/metric/{metric}/{index}/data",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       addr: Extension<SocketAddr>,
                       state: State<AppState>,
                       Path(path): Path<MetricWithIndex>,
                       Query(range): Query<DataRangeFormat>|
                       -> Response {
                    data::raw_handler(
                        uri,
                        headers,
                        addr,
                        Query(MetricSelection::from((path.index, path.metric, range))),
                        state,
                    )
                    .await
                    .into_response()
                },
                |op| op
                    .id("get_metric_data")
                    .metrics_tag()
                    .summary("Get raw metric data")
                    .description(
                        "Returns just the data array without the MetricData wrapper. \
                        Supports the same range and format parameters as the standard endpoint."
                    )
                    .ok_response::<Vec<serde_json::Value>>()
                    .csv_response()
                    .not_modified()
                    .not_found(),
            ),
        )
        .api_route(
            "/api/metric/{metric}/{index}",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       addr: Extension<SocketAddr>,
                       state: State<AppState>,
                       Path(path): Path<MetricWithIndex>,
                       Query(range): Query<DataRangeFormat>|
                       -> Response {
                    data::handler(
                        uri,
                        headers,
                        addr,
                        Query(MetricSelection::from((path.index, path.metric, range))),
                        state,
                    )
                    .await
                    .into_response()
                },
                |op| op
                    .id("get_metric")
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
                |uri, headers, addr, query, state| async move {
                    bulk::handler(uri, headers, addr, query, state).await.into_response()
                },
                |op| op
                    .id("get_metrics")
                    .metrics_tag()
                    .summary("Bulk metric data")
                    .description(
                        "Fetch multiple metrics in a single request. Supports filtering by index and date range. \
                        Returns an array of MetricData objects. For a single metric, use `get_metric` instead."
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
                       addr: Extension<SocketAddr>,
                       Path(variant): Path<String>,
                       Query(range): Query<DataRangeFormat>,
                       state: State<AppState>|
                       -> Response {
                    let separator = "_to_";
                    let variant = variant.replace("-", "_");
                    let mut split = variant.split(separator);

                    let ser_index = split.next().unwrap();
                    let Ok(index) = Index::try_from(ser_index) else {
                        return Error::not_found(
                            format!("Index '{ser_index}' doesn't exist")
                        ).into_response();
                    };

                    let params = MetricSelection::from((
                        index,
                        Metrics::from(split.collect::<Vec<_>>().join(separator)),
                        range,
                    ));
                    legacy::handler(uri, headers, addr, Query(params), state)
                        .await
                        .into_response()
                },
                |op| op
                    .metrics_tag()
                    .summary("Legacy variant endpoint")
                    .description(
                        "**DEPRECATED** - Use `/api/metric/{metric}/{index}` instead.\n\n\
                        Sunset date: 2027-01-01. May be removed earlier in case of abuse.\n\n\
                        Legacy endpoint for querying metrics by variant path (e.g., `day1_to_price`). \
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
                       addr: Extension<SocketAddr>,
                       Query(params): Query<MetricSelectionLegacy>,
                       state: State<AppState>|
                       -> Response {
                    let params: MetricSelection = params.into();
                    legacy::handler(uri, headers, addr, Query(params), state)
                        .await
                        .into_response()
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
        // Cost basis distribution endpoints
        .api_route(
            "/api/metrics/cost-basis",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, CacheStrategy::Static, &uri, |q| q.cost_basis_cohorts())
                        .await
                },
                |op| {
                    op.id("get_cost_basis_cohorts")
                        .metrics_tag()
                        .summary("Available cost basis cohorts")
                        .description("List available cohorts for cost basis distribution.")
                        .ok_response::<Vec<String>>()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/metrics/cost-basis/{cohort}/dates",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       Path(params): Path<CostBasisCohortParam>,
                       State(state): State<AppState>| {
                    state
                        .cached_json(&headers, CacheStrategy::Height, &uri, move |q| {
                            q.cost_basis_dates(&params.cohort)
                        })
                        .await
                },
                |op| {
                    op.id("get_cost_basis_dates")
                        .metrics_tag()
                        .summary("Available cost basis dates")
                        .description("List available dates for a cohort's cost basis distribution.")
                        .ok_response::<Vec<Date>>()
                        .not_found()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/metrics/cost-basis/{cohort}/{date}",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       Path(params): Path<CostBasisParams>,
                       Query(query): Query<CostBasisQuery>,
                       State(state): State<AppState>| {
                    state
                        .cached_json(&headers, CacheStrategy::Static, &uri, move |q| {
                            q.cost_basis_formatted(
                                &params.cohort,
                                params.date,
                                query.bucket,
                                query.value,
                            )
                        })
                        .await
                },
                |op| {
                    op.id("get_cost_basis")
                        .metrics_tag()
                        .summary("Cost basis distribution")
                        .description(
                            "Get the cost basis distribution for a cohort on a specific date.\n\n\
                            Query params:\n\
                            - `bucket`: raw (default), lin200, lin500, lin1000, log10, log50, log100\n\
                            - `value`: supply (default, in BTC), realized (USD), unrealized (USD)",
                        )
                        .ok_response::<CostBasisFormatted>()
                        .not_found()
                        .server_error()
                },
            ),
        )
    }
}
