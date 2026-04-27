use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
};
use brk_types::{Dollars, MempoolInfo, MempoolRecentTx, Txid};

use crate::{AppState, extended::TransformResponseExtended, params::Empty};

pub trait MempoolRoutes {
    fn add_mempool_routes(self) -> Self;
}

impl MempoolRoutes for ApiRouter<AppState> {
    fn add_mempool_routes(self) -> Self {
        self.api_route(
            "/api/mempool",
            get_with(
                async |uri: Uri, headers: HeaderMap, _: Empty, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, state.mempool_cache(), &uri, |q| q.mempool_info())
                        .await
                },
                |op| {
                    op.id("get_mempool")
                        .mempool_tag()
                        .summary("Mempool statistics")
                        .description("Get current mempool statistics including transaction count, total vsize, total fees, and fee histogram.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool)*")
                        .json_response::<MempoolInfo>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/mempool/txids",
            get_with(
                async |uri: Uri, headers: HeaderMap, _: Empty, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, state.mempool_cache(), &uri, |q| q.mempool_txids())
                        .await
                },
                |op| {
                    op.id("get_mempool_txids")
                        .mempool_tag()
                        .summary("Mempool transaction IDs")
                        .description("Get all transaction IDs currently in the mempool.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-transaction-ids)*")
                        .json_response::<Vec<Txid>>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/mempool/recent",
            get_with(
                async |uri: Uri, headers: HeaderMap, _: Empty, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, state.mempool_cache(), &uri, |q| q.mempool_recent())
                        .await
                },
                |op| {
                    op.id("get_mempool_recent")
                        .mempool_tag()
                        .summary("Recent mempool transactions")
                        .description("Get the last 10 transactions to enter the mempool.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-recent)*")
                        .json_response::<Vec<MempoolRecentTx>>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/mempool/price",
            get_with(
                async |uri: Uri, headers: HeaderMap, _: Empty, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, state.mempool_cache(), &uri, |q| q.live_price())
                        .await
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
                        .json_response::<Dollars>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
    }
}
