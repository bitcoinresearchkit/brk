use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
};
use brk_types::{Dollars, MempoolInfo, MempoolRecentTx, ReplacementNode, Txid};

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
                        .respond_json(&headers, state.mempool_strategy(), &uri, |q| q.mempool_info())
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
                        .respond_json(&headers, state.mempool_strategy(), &uri, |q| q.mempool_txids())
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
                        .respond_json(&headers, state.mempool_strategy(), &uri, |q| q.mempool_recent())
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
            "/api/v1/replacements",
            get_with(
                async |uri: Uri, headers: HeaderMap, _: Empty, State(state): State<AppState>| {
                    state
                        .respond_json(&headers, state.mempool_strategy(), &uri, |q| {
                            q.recent_replacements(false)
                        })
                        .await
                },
                |op| {
                    op.id("get_replacements")
                        .mempool_tag()
                        .summary("Recent RBF replacements")
                        .description("Returns up to 25 most-recent RBF replacement trees across the whole mempool. Each entry has the same shape as `tx_rbf().replacements`.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-replacements)*")
                        .json_response::<Vec<ReplacementNode>>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/fullrbf/replacements",
            get_with(
                async |uri: Uri, headers: HeaderMap, _: Empty, State(state): State<AppState>| {
                    state
                        .respond_json(&headers, state.mempool_strategy(), &uri, |q| {
                            q.recent_replacements(true)
                        })
                        .await
                },
                |op| {
                    op.id("get_fullrbf_replacements")
                        .mempool_tag()
                        .summary("Recent full-RBF replacements")
                        .description("Like `/api/v1/replacements`, but limited to trees where at least one predecessor was non-signaling (full-RBF).\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-fullrbf-replacements)*")
                        .json_response::<Vec<ReplacementNode>>()
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
                        .respond_json(&headers, state.mempool_strategy(), &uri, |q| q.live_price())
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
