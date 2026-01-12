use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::State,
    http::HeaderMap,
    response::Redirect,
    routing::get,
};
use brk_types::{MempoolBlock, MempoolInfo, RecommendedFees, Txid};

use crate::{CacheStrategy, extended::TransformResponseExtended};

use super::AppState;

pub trait MempoolRoutes {
    fn add_mempool_routes(self) -> Self;
}

impl MempoolRoutes for ApiRouter<AppState> {
    fn add_mempool_routes(self) -> Self {
        self
            .route("/api/mempool", get(Redirect::temporary("/api#tag/mempool")))
            .api_route(
                "/api/mempool/info",
                get_with(
                    async |headers: HeaderMap, State(state): State<AppState>| {
                        state.cached_json(&headers, CacheStrategy::MaxAge(5), |q| q.mempool_info()).await
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
                    async |headers: HeaderMap, State(state): State<AppState>| {
                        state.cached_json(&headers, CacheStrategy::MaxAge(5), |q| q.mempool_txids()).await
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
                "/api/v1/fees/recommended",
                get_with(
                    async |headers: HeaderMap, State(state): State<AppState>| {
                        state.cached_json(&headers, CacheStrategy::MaxAge(3), |q| q.recommended_fees()).await
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
                "/api/v1/fees/mempool-blocks",
                get_with(
                    async |headers: HeaderMap, State(state): State<AppState>| {
                        state.cached_json(&headers, CacheStrategy::MaxAge(5), |q| q.mempool_blocks()).await
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
    }
}
