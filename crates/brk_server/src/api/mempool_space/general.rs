use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Query, State},
    http::{HeaderMap, Uri},
};
use brk_types::{DifficultyAdjustment, HistoricalPrice, Prices, Timestamp};

use crate::{
    AppState, CacheStrategy,
    extended::TransformResponseExtended,
    params::OptionalTimestampParam,
};

pub trait GeneralRoutes {
    fn add_general_routes(self) -> Self;
}

impl GeneralRoutes for ApiRouter<AppState> {
    fn add_general_routes(self) -> Self {
        self.api_route(
            "/api/v1/difficulty-adjustment",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, CacheStrategy::Tip, &uri, |q| {
                            q.difficulty_adjustment()
                        })
                        .await
                },
                |op| {
                    op.id("get_difficulty_adjustment")
                        .general_tag()
                        .summary("Difficulty adjustment")
                        .description("Get current difficulty adjustment progress and estimates.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-difficulty-adjustment)*")
                        .json_response::<DifficultyAdjustment>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/prices",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, state.mempool_cache(), &uri, |q| {
                            Ok(Prices {
                                time: Timestamp::now(),
                                usd: q.live_price()?,
                            })
                        })
                        .await
                },
                |op| {
                    op.id("get_prices")
                        .general_tag()
                        .summary("Current BTC price")
                        .description("Returns bitcoin latest price (on-chain derived, USD only).\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-price)*")
                        .json_response::<Prices>()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/historical-price",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       Query(params): Query<OptionalTimestampParam>,
                       State(state): State<AppState>| {
                    state
                        .cached_json(&headers, CacheStrategy::Tip, &uri, move |q| {
                            q.historical_price(params.timestamp)
                        })
                        .await
                },
                |op| {
                    op.id("get_historical_price")
                        .general_tag()
                        .summary("Historical price")
                        .description("Get historical BTC/USD price. Optionally specify a UNIX timestamp to get the price at that time.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-historical-price)*")
                        .json_response::<HistoricalPrice>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
    }
}
