use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Query, State},
    http::{HeaderMap, Uri},
};
use brk_types::{
    Dollars, HistoricalPrice, MempoolBlock, MempoolInfo, MempoolRecentTx, OptionalTimestampParam,
    Prices, RecommendedFees, Timestamp, Txid,
};

use crate::{CacheStrategy, extended::TransformResponseExtended};

use super::AppState;

pub trait MempoolRoutes {
    fn add_mempool_routes(self) -> Self;
}

impl MempoolRoutes for ApiRouter<AppState> {
    fn add_mempool_routes(self) -> Self {
        self
            .api_route(
                "/api/mempool",
                get_with(
                    async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                        state.cached_json(&headers, state.mempool_cache(), &uri, |q| q.mempool_info()).await
                    },
                    |op| {
                        op.id("get_mempool")
                            .mempool_tag()
                            .summary("Mempool statistics")
                            .description("Get current mempool statistics including transaction count, total vsize, and total fees.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool)*")
                            .ok_response::<MempoolInfo>()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/mempool/txids",
                get_with(
                    async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                        state.cached_json(&headers, state.mempool_cache(), &uri, |q| q.mempool_txids()).await
                    },
                    |op| {
                        op.id("get_mempool_txids")
                            .mempool_tag()
                            .summary("Mempool transaction IDs")
                            .description("Get all transaction IDs currently in the mempool.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-transaction-ids)*")
                            .ok_response::<Vec<Txid>>()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/mempool/recent",
                get_with(
                    async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                        state.cached_json(&headers, state.mempool_cache(), &uri, |q| q.mempool_recent()).await
                    },
                    |op| {
                        op.id("get_mempool_recent")
                            .mempool_tag()
                            .summary("Recent mempool transactions")
                            .description("Get the last 10 transactions to enter the mempool.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-recent)*")
                            .ok_response::<Vec<MempoolRecentTx>>()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/v1/prices",
                get_with(
                    async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                        state.cached_json(&headers, state.mempool_cache(), &uri, |q| {
                            Ok(Prices {
                                time: Timestamp::now(),
                                usd: q.live_price()?,
                            })
                        }).await
                    },
                    |op| {
                        op.id("get_prices")
                            .mempool_tag()
                            .summary("Current BTC price")
                            .description("Returns bitcoin latest price (on-chain derived, USD only).\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-price)*")
                            .ok_response::<Prices>()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/mempool/price",
                get_with(
                    async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                        state.cached_json(&headers, state.mempool_cache(), &uri, |q| q.live_price()).await
                    },
                    |op| {
                        op.id("get_live_price")
                            .mempool_tag()
                            .summary("Live BTC/USD price")
                            .description(
                                "Returns the current BTC/USD price in dollars, derived from \
                                on-chain round-dollar output patterns in the last 12 blocks \
                                plus mempool.",
                            )
                            .ok_response::<Dollars>()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/v1/fees/recommended",
                get_with(
                    async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                        state.cached_json(&headers, state.mempool_cache(), &uri, |q| q.recommended_fees()).await
                    },
                    |op| {
                        op.id("get_recommended_fees")
                            .mempool_tag()
                            .summary("Recommended fees")
                            .description("Get recommended fee rates for different confirmation targets based on current mempool state.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees)*")
                            .ok_response::<RecommendedFees>()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/v1/fees/precise",
                get_with(
                    async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                        state.cached_json(&headers, state.mempool_cache(), &uri, |q| q.recommended_fees()).await
                    },
                    |op| {
                        op.id("get_precise_fees")
                            .mempool_tag()
                            .summary("Precise recommended fees")
                            .description("Get recommended fee rates with up to 3 decimal places, including sub-sat feerates.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees-precise)*")
                            .ok_response::<RecommendedFees>()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/v1/fees/mempool-blocks",
                get_with(
                    async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                        state.cached_json(&headers, state.mempool_cache(), &uri, |q| q.mempool_blocks()).await
                    },
                    |op| {
                        op.id("get_mempool_blocks")
                            .mempool_tag()
                            .summary("Projected mempool blocks")
                            .description("Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)*")
                            .ok_response::<Vec<MempoolBlock>>()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/v1/historical-price",
                get_with(
                    async |uri: Uri, headers: HeaderMap, Query(params): Query<OptionalTimestampParam>, State(state): State<AppState>| {
                        state.cached_json(&headers, CacheStrategy::Height, &uri, move |q| q.historical_price(params.timestamp)).await
                    },
                    |op| {
                        op.id("get_historical_price")
                            .mempool_tag()
                            .summary("Historical price")
                            .description("Get historical BTC/USD price. Optionally specify a UNIX timestamp to get the price at that time.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-historical-price)*")
                            .ok_response::<HistoricalPrice>()
                            .not_modified()
                            .server_error()
                    },
                ),
            )
    }
}
