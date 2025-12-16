use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Redirect,
    routing::get,
};
use brk_types::{
    BlockCountParam, BlockFeesEntry, BlockRewardsEntry, BlockSizesWeights,
    DifficultyAdjustment, DifficultyAdjustmentEntry, HashrateSummary, PoolDetail, PoolInfo,
    PoolSlugParam, PoolsSummary, RewardStats, TimePeriodParam,
};

use crate::{CacheStrategy, extended::TransformResponseExtended};

use super::AppState;

pub trait MiningRoutes {
    fn add_mining_routes(self) -> Self;
}

impl MiningRoutes for ApiRouter<AppState> {
    fn add_mining_routes(self) -> Self {
        self.route(
            "/api/v1/mining",
            get(Redirect::temporary("/api#tag/mining")),
        )
        .api_route(
            "/api/v1/difficulty-adjustment",
            get_with(
                async |headers: HeaderMap, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::Height, |q| q.difficulty_adjustment()).await
                },
                |op| {
                    op.mining_tag()
                        .summary("Difficulty adjustment")
                        .description("Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.")
                        .ok_response::<DifficultyAdjustment>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/mining/pools",
            get_with(
                async |headers: HeaderMap, State(state): State<AppState>| {
                    // Pool list is static, only changes on code update
                    state.cached_json(&headers, CacheStrategy::Static, |q| Ok(q.all_pools())).await
                },
                |op| {
                    op.mining_tag()
                        .summary("List all mining pools")
                        .description("Get list of all known mining pools with their identifiers.")
                        .ok_response::<Vec<PoolInfo>>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/mining/pools/{time_period}",
            get_with(
                async |headers: HeaderMap, Path(path): Path<TimePeriodParam>, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::height_with(format!("{:?}", path.time_period)), move |q| q.mining_pools(path.time_period)).await
                },
                |op| {
                    op.mining_tag()
                        .summary("Mining pool statistics")
                        .description("Get mining pool statistics for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y")
                        .ok_response::<PoolsSummary>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/mining/pool/{slug}",
            get_with(
                async |headers: HeaderMap, Path(path): Path<PoolSlugParam>, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::height_with(path.slug), move |q| q.pool_detail(path.slug)).await
                },
                |op| {
                    op.mining_tag()
                        .summary("Mining pool details")
                        .description("Get detailed information about a specific mining pool including block counts and shares for different time periods.")
                        .ok_response::<PoolDetail>()
                        .not_modified()
                        .not_found()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/mining/hashrate",
            get_with(
                async |headers: HeaderMap, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::height_with("hashrate"), |q| q.hashrate(None)).await
                },
                |op| {
                    op.mining_tag()
                        .summary("Network hashrate (all time)")
                        .description("Get network hashrate and difficulty data for all time.")
                        .ok_response::<HashrateSummary>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/mining/hashrate/{time_period}",
            get_with(
                async |headers: HeaderMap, Path(path): Path<TimePeriodParam>, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::height_with(format!("hashrate-{:?}", path.time_period)), move |q| q.hashrate(Some(path.time_period))).await
                },
                |op| {
                    op.mining_tag()
                        .summary("Network hashrate")
                        .description("Get network hashrate and difficulty data for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y")
                        .ok_response::<HashrateSummary>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/mining/difficulty-adjustments",
            get_with(
                async |headers: HeaderMap, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::height_with("diff-adj"), |q| q.difficulty_adjustments(None)).await
                },
                |op| {
                    op.mining_tag()
                        .summary("Difficulty adjustments (all time)")
                        .description("Get historical difficulty adjustments. Returns array of [timestamp, height, difficulty, change_percent].")
                        .ok_response::<Vec<DifficultyAdjustmentEntry>>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/mining/difficulty-adjustments/{time_period}",
            get_with(
                async |headers: HeaderMap, Path(path): Path<TimePeriodParam>, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::height_with(format!("diff-adj-{:?}", path.time_period)), move |q| q.difficulty_adjustments(Some(path.time_period))).await
                },
                |op| {
                    op.mining_tag()
                        .summary("Difficulty adjustments")
                        .description("Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y. Returns array of [timestamp, height, difficulty, change_percent].")
                        .ok_response::<Vec<DifficultyAdjustmentEntry>>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/mining/blocks/fees/{time_period}",
            get_with(
                async |headers: HeaderMap, Path(path): Path<TimePeriodParam>, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::height_with(format!("fees-{:?}", path.time_period)), move |q| q.block_fees(path.time_period)).await
                },
                |op| {
                    op.mining_tag()
                        .summary("Block fees")
                        .description("Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y")
                        .ok_response::<Vec<BlockFeesEntry>>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/mining/blocks/rewards/{time_period}",
            get_with(
                async |headers: HeaderMap, Path(path): Path<TimePeriodParam>, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::height_with(format!("rewards-{:?}", path.time_period)), move |q| q.block_rewards(path.time_period)).await
                },
                |op| {
                    op.mining_tag()
                        .summary("Block rewards")
                        .description("Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y")
                        .ok_response::<Vec<BlockRewardsEntry>>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        // TODO: Disabled - dateindex doesn't have percentile fields (see block_fee_rates.rs)
        // .api_route(
        //     "/api/v1/mining/blocks/fee-rates/{time_period}",
        //     get_with(
        //         async |headers: HeaderMap, Path(path): Path<TimePeriodParam>, State(state): State<AppState>| {
        //             state.cached_json(&headers, CacheStrategy::height_with(format!("feerates-{:?}", path.time_period)), move |q| q.block_fee_rates(path.time_period)).await
        //         },
        //         |op| {
        //             op.mining_tag()
        //                 .summary("Block fee rates")
        //                 .description("Get block fee rate percentiles (min, 10th, 25th, median, 75th, 90th, max) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y")
        //                 .ok_response::<Vec<BlockFeeRatesEntry>>()
        //                 .not_modified()
        //                 .server_error()
        //         },
        //     ),
        // )
        .api_route(
            "/api/v1/mining/blocks/sizes-weights/{time_period}",
            get_with(
                async |headers: HeaderMap, Path(path): Path<TimePeriodParam>, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::height_with(format!("sizes-{:?}", path.time_period)), move |q| q.block_sizes_weights(path.time_period)).await
                },
                |op| {
                    op.mining_tag()
                        .summary("Block sizes and weights")
                        .description("Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y")
                        .ok_response::<BlockSizesWeights>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/mining/reward-stats/{block_count}",
            get_with(
                async |headers: HeaderMap, Path(path): Path<BlockCountParam>, State(state): State<AppState>| {
                    state.cached_json(&headers, CacheStrategy::height_with(format!("reward-stats-{}", path.block_count)), move |q| q.reward_stats(path.block_count)).await
                },
                |op| {
                    op.mining_tag()
                        .summary("Mining reward statistics")
                        .description("Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count.")
                        .ok_response::<RewardStats>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
    }
}
