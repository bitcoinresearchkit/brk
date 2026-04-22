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
    DataRangeFormat, IndexInfo, PaginatedSeries, Pagination, SearchQuery, SeriesCount, SeriesData,
    SeriesInfo, SeriesNameWithIndex, SeriesSelection,
};

use crate::{
    CacheStrategy,
    cache::CACHE_CONTROL,
    extended::TransformResponseExtended,
    params::SeriesParam,
};

use self::cost_basis::ApiCostBasisLegacyRoutes;
use super::AppState;

mod bulk;
mod cost_basis;
mod data;
pub mod legacy;

/// Maximum allowed request weight in bytes (320KB)
const MAX_WEIGHT: usize = 4 * 8 * 10_000;
/// Maximum allowed request weight for localhost (50MB)
const MAX_WEIGHT_LOCALHOST: usize = 50 * 1_000_000;

/// Returns the max weight for a request based on the client address.
/// Localhost requests get a generous limit, external requests get a stricter one.
fn max_weight(addr: &SocketAddr) -> usize {
    if addr.ip().is_loopback() {
        MAX_WEIGHT_LOCALHOST
    } else {
        MAX_WEIGHT
    }
}

pub trait ApiSeriesRoutes {
    fn add_series_routes(self) -> Self;
}

impl ApiSeriesRoutes for ApiRouter<AppState> {
    fn add_series_routes(self) -> Self {
        self.api_route(
            "/api/series",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::Static, &uri, |q| Ok(q.series_catalog().clone())).await
                },
                |op| op
                    .id("get_series_tree")
                    .series_tag()
                    .summary("Series catalog")
                    .description(
                        "Returns the complete hierarchical catalog of available series organized as a tree structure. \
                        Series are grouped by categories and subcategories."
                    )
                    .json_response::<TreeNode>()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/series/count",
            get_with(
                async |
                    uri: Uri,
                    headers: HeaderMap,
                    State(state): State<AppState>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, &uri, |q| Ok(q.series_count())).await
                },
                |op| op
                    .id("get_series_count")
                    .series_tag()
                    .summary("Series count")
                    .description("Returns the number of series available per index type.")
                    .json_response::<Vec<SeriesCount>>()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/series/indexes",
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
                    .series_tag()
                    .summary("List available indexes")
                    .description(
                        "Returns all available indexes with their accepted query aliases. Use any alias when querying series."
                    )
                    .json_response::<Vec<IndexInfo>>()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/series/list",
            get_with(
                async |
                    uri: Uri,
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Query(pagination): Query<Pagination>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, &uri, move |q| Ok(q.series_list(pagination))).await
                },
                |op| op
                    .id("list_series")
                    .series_tag()
                    .summary("Series list")
                    .description("Paginated flat list of all available series names. Use `page` query param for pagination.")
                    .json_response::<PaginatedSeries>()
                    .not_modified(),
            ),
        )
        .api_route(
            "/api/series/search",
            get_with(
                async |
                    uri: Uri,
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Query(query): Query<SearchQuery>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, &uri, move |q| Ok(q.search_series(&query))).await
                },
                |op| op
                    .id("search_series")
                    .series_tag()
                    .summary("Search series")
                    .description("Fuzzy search for series by name. Supports partial matches and typos.")
                    .json_response::<Vec<&str>>()
                    .not_modified()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/series/{series}",
            get_with(
                async |
                    uri: Uri,
                    headers: HeaderMap,
                    State(state): State<AppState>,
                    Path(path): Path<SeriesParam>
                | {
                    state.cached_json(&headers, CacheStrategy::Static, &uri, move |q| {
                        q.series_info(&path.series).ok_or_else(|| q.series_not_found_error(&path.series))
                    }).await
                },
                |op| op
                    .id("get_series_info")
                    .series_tag()
                    .summary("Get series info")
                    .description(
                        "Returns the supported indexes and value type for the specified series."
                    )
                    .json_response::<SeriesInfo>()
                    .not_modified()
                    .not_found()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/series/{series}/{index}",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       addr: Extension<SocketAddr>,
                       state: State<AppState>,
                       Path(path): Path<SeriesNameWithIndex>,
                       Query(range): Query<DataRangeFormat>|
                       -> Response {
                    data::handler(
                        uri,
                        headers,
                        addr,
                        Query(SeriesSelection::from((path.index, path.series, range))),
                        state,
                    )
                    .await
                    .into_response()
                },
                |op| op
                    .id("get_series")
                    .series_tag()
                    .summary("Get series data")
                    .description(
                        "Fetch data for a specific series at the given index. \
                        Use query parameters to filter by date range and format (json/csv)."
                    )
                    .json_response::<SeriesData>()
                    .csv_response()
                    .not_modified()
                    .not_found(),
            ),
        )
        .api_route(
            "/api/series/{series}/{index}/data",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       addr: Extension<SocketAddr>,
                       state: State<AppState>,
                       Path(path): Path<SeriesNameWithIndex>,
                       Query(range): Query<DataRangeFormat>|
                       -> Response {
                    data::raw_handler(
                        uri,
                        headers,
                        addr,
                        Query(SeriesSelection::from((path.index, path.series, range))),
                        state,
                    )
                    .await
                    .into_response()
                },
                |op| op
                    .id("get_series_data")
                    .series_tag()
                    .summary("Get raw series data")
                    .description(
                        "Returns just the data array without the SeriesData wrapper. \
                        Supports the same range and format parameters as the standard endpoint."
                    )
                    .json_response::<Vec<serde_json::Value>>()
                    .csv_response()
                    .not_modified()
                    .not_found(),
            ),
        )
        .api_route(
            "/api/series/{series}/{index}/latest",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       State(state): State<AppState>,
                       Path(path): Path<SeriesNameWithIndex>| {
                    state
                        .cached_json(&headers, CacheStrategy::Tip, &uri, move |q| {
                            q.latest(&path.series, path.index)
                        })
                        .await
                },
                |op| op
                    .id("get_series_latest")
                    .series_tag()
                    .summary("Get latest series value")
                    .description(
                        "Returns the single most recent value for a series, unwrapped (not inside a SeriesData object)."
                    )
                    .json_response::<serde_json::Value>()
                    .not_modified()
                    .not_found(),
            ),
        )
        .api_route(
            "/api/series/{series}/{index}/len",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       State(state): State<AppState>,
                       Path(path): Path<SeriesNameWithIndex>| {
                    state
                        .cached_json(&headers, CacheStrategy::Tip, &uri, move |q| {
                            q.len(&path.series, path.index)
                        })
                        .await
                },
                |op| op
                    .id("get_series_len")
                    .series_tag()
                    .summary("Get series data length")
                    .description("Returns the total number of data points for a series at the given index.")
                    .json_response::<usize>()
                    .not_modified()
                    .not_found(),
            ),
        )
        .api_route(
            "/api/series/{series}/{index}/version",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       State(state): State<AppState>,
                       Path(path): Path<SeriesNameWithIndex>| {
                    state
                        .cached_json(&headers, CacheStrategy::Tip, &uri, move |q| {
                            q.version(&path.series, path.index)
                        })
                        .await
                },
                |op| op
                    .id("get_series_version")
                    .series_tag()
                    .summary("Get series version")
                    .description("Returns the current version of a series. Changes when the series data is updated.")
                    .json_response::<brk_types::Version>()
                    .not_modified()
                    .not_found(),
            ),
        )
        .api_route(
            "/api/series/bulk",
            get_with(
                |uri, headers, addr, query, state| async move {
                    bulk::handler(uri, headers, addr, query, state).await.into_response()
                },
                |op| op
                    .id("get_series_bulk")
                    .series_tag()
                    .summary("Bulk series data")
                    .description(
                        "Fetch multiple series in a single request. Supports filtering by index and date range. \
                        Returns an array of SeriesData objects. For a single series, use `get_series` instead."
                    )
                    .json_response::<Vec<SeriesData>>()
                    .csv_response()
                    .not_modified(),
            ),
        )
        .add_cost_basis_legacy_routes()
    }
}
