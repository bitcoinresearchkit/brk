use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, Uri},
    response::IntoResponse,
};
use brk_oracle::{HistogramEmaCompact, HistogramRaw};
use brk_types::{Day1, Dollars, Version};

use crate::{
    AppState,
    extended::TransformResponseExtended,
    params::{Empty, HeightOrDate, HeightOrDateParam},
};

pub trait OracleRoutes {
    fn add_oracle_routes(self) -> Self;
}

impl OracleRoutes for ApiRouter<AppState> {
    fn add_oracle_routes(self) -> Self {
        self.api_route(
            "/api/oracle/price",
            get_with(
                async |uri: Uri, headers: HeaderMap, _: Empty, State(state): State<AppState>| {
                    state
                        .respond_json(&headers, state.mempool_strategy(), &uri, |q| q.live_price())
                        .await
                },
                |op| {
                    op.id("get_oracle_price")
                        .oracle_tag()
                        .summary("Live BTC/USD price")
                        .description(
                            "Current BTC/USD price in dollars. Same value as \
                            `/api/mempool/price`. Confirmed per-height history is available at \
                            `/api/vecs/height-to-price`.",
                        )
                        .json_response::<Dollars>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/oracle/histogram/ema/live",
            get_with(
                async |uri: Uri, headers: HeaderMap, _: Empty, State(state): State<AppState>| {
                    state
                        .respond_json(&headers, state.mempool_strategy(), &uri, |q| {
                            q.live_histogram_ema()
                        })
                        .await
                },
                |op| {
                    op.id("get_oracle_histogram_ema_live")
                        .oracle_tag()
                        .summary("Live EMA histogram")
                        .description(
                            "Smoothed round-dollar payment histogram at the live tip: the \
                            committed EMA with the forming mempool block blended in. \
                            A flat array of log-scale bins.",
                        )
                        .json_response::<HistogramEmaCompact>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/oracle/histogram/ema/{point}",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       Path(path): Path<HeightOrDateParam>,
                       _: Empty,
                       State(state): State<AppState>| {
                    let version = Version::new(brk_oracle::VERSION);
                    match path.resolve() {
                        Ok(HeightOrDate::Date(date)) => {
                            let strategy = state.date_strategy(version, date);
                            state
                                .respond_json(&headers, strategy, &uri, move |q| {
                                    q.confirmed_histogram_ema_day(Day1::try_from(date)?)
                                })
                                .await
                        }
                        Ok(HeightOrDate::Height(height)) => {
                            let strategy = state.height_strategy(version, height);
                            state
                                .respond_json(&headers, strategy, &uri, move |q| {
                                    q.confirmed_histogram_ema(usize::from(height))
                                })
                                .await
                        }
                        Err(e) => e.into_response(),
                    }
                },
                |op| {
                    op.id("get_oracle_histogram_ema")
                        .oracle_tag()
                        .summary("EMA histogram at height or day")
                        .description(
                            "Smoothed round-dollar payment histogram for a confirmed point: a \
                            block height (`840000`) gives that block's EMA, a calendar date \
                            (`YYYY-MM-DD`) gives the average of that day's per-block EMAs. \
                            A flat array of log-scale bins.",
                        )
                        .json_response::<HistogramEmaCompact>()
                        .not_modified()
                        .bad_request()
                        .not_found()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/oracle/histogram/raw/live",
            get_with(
                async |uri: Uri, headers: HeaderMap, _: Empty, State(state): State<AppState>| {
                    state
                        .respond_json(&headers, state.mempool_strategy(), &uri, |q| {
                            q.live_histogram_raw()
                        })
                        .await
                },
                |op| {
                    op.id("get_oracle_histogram_raw_live")
                        .oracle_tag()
                        .summary("Live raw histogram")
                        .description(
                            "Unfiltered output histogram for the forming mempool block: every \
                            live output binned by value, with none of the round-dollar payment \
                            filters applied. A flat array of log-scale bins, all zero when no \
                            mempool is configured.",
                        )
                        .json_response::<HistogramRaw>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/oracle/histogram/raw/{point}",
            get_with(
                async |uri: Uri,
                       headers: HeaderMap,
                       Path(path): Path<HeightOrDateParam>,
                       _: Empty,
                       State(state): State<AppState>| {
                    let version = Version::new(brk_oracle::VERSION);

                    match path.resolve() {
                        Ok(HeightOrDate::Date(date)) => {
                            let strategy = state.date_strategy(version, date);
                            state
                                .respond_json(&headers, strategy, &uri, move |q| {
                                    q.confirmed_histogram_raw_day(Day1::try_from(date)?)
                                })
                                .await
                        }
                        Ok(HeightOrDate::Height(height)) => {
                            let strategy = state.height_strategy(version, height);
                            state
                                .respond_json(&headers, strategy, &uri, move |q| {
                                    q.confirmed_histogram_raw(usize::from(height))
                                })
                                .await
                        }
                        Err(e) => e.into_response(),
                    }
                },
                |op| {
                    op.id("get_oracle_histogram_raw")
                        .oracle_tag()
                        .summary("Raw histogram at height or day")
                        .description(
                            "Unfiltered output histogram for a confirmed point: a block height \
                            (`840000`) gives that block's outputs, coinbase included, binned by \
                            value with no payment filtering; a calendar date (`YYYY-MM-DD`) sums \
                            every block that day. A flat array of log-scale bins.",
                        )
                        .json_response::<HistogramRaw>()
                        .not_modified()
                        .bad_request()
                        .not_found()
                        .server_error()
                },
            ),
        )
    }
}
