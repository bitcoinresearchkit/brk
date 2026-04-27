use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::State,
    http::{HeaderMap, Uri},
};
use brk_types::{MempoolBlock, RecommendedFees};

use crate::{AppState, extended::TransformResponseExtended};

pub trait FeesRoutes {
    fn add_fees_routes(self) -> Self;
}

impl FeesRoutes for ApiRouter<AppState> {
    fn add_fees_routes(self) -> Self {
        self.api_route(
            "/api/v1/fees/mempool-blocks",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, state.mempool_cache(), &uri, |q| {
                            q.mempool_blocks()
                        })
                        .await
                },
                |op| {
                    op.id("get_mempool_blocks")
                        .fees_tag()
                        .summary("Projected mempool blocks")
                        .description("Get projected blocks from the mempool for fee estimation.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)*")
                        .json_response::<Vec<MempoolBlock>>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/fees/recommended",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, state.mempool_cache(), &uri, |q| {
                            q.recommended_fees()
                        })
                        .await
                },
                |op| {
                    op.id("get_recommended_fees")
                        .fees_tag()
                        .summary("Recommended fees")
                        .description("Get recommended fee rates for different confirmation targets.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees)*")
                        .json_response::<RecommendedFees>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/fees/precise",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, state.mempool_cache(), &uri, |q| {
                            q.recommended_fees()
                        })
                        .await
                },
                |op| {
                    op.id("get_precise_fees")
                        .fees_tag()
                        .summary("Precise recommended fees")
                        .description("Get recommended fee rates with up to 3 decimal places.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-recommended-fees-precise)*")
                        .json_response::<RecommendedFees>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
    }
}
