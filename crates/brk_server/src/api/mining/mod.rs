use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::{Redirect, Response},
    routing::get,
};
use brk_types::{
    BlockCountPath, BlockFeeRatesEntry, BlockFeesEntry, BlockRewardsEntry, BlockSizesWeights,
    DifficultyAdjustment, DifficultyAdjustmentEntry, HashrateSummary, PoolDetail, PoolInfo,
    PoolSlugPath, PoolsSummary, RewardStats, TimePeriodPath,
};

use crate::{
    VERSION,
    extended::{HeaderMapExtended, ResponseExtended, ResultExtended, TransformResponseExtended},
};

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
                    let etag = format!("{VERSION}-{}", state.get_height().await);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state
                        .get_difficulty_adjustment()
                        .await
                        .to_json_response(&etag)
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
                    let etag = format!("{VERSION}-pools");
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state.get_all_pools().await.to_json_response(&etag)
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
            "/api/v1/mining/pools/:time_period",
            get_with(
                async |headers: HeaderMap, Path(path): Path<TimePeriodPath>, State(state): State<AppState>| {
                    let etag = format!("{VERSION}-{}-{:?}", state.get_height().await, path.time_period);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state
                        .get_mining_pools(path.time_period)
                        .await
                        .to_json_response(&etag)
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
            "/api/v1/mining/pool/:slug",
            get_with(
                async |headers: HeaderMap, Path(path): Path<PoolSlugPath>, State(state): State<AppState>| {
                    let etag = format!("{VERSION}-{}-{:?}", state.get_height().await, path.slug);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state
                        .get_pool_detail(path.slug)
                        .await
                        .to_json_response(&etag)
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
                    let etag = format!("{VERSION}-hashrate-{}", state.get_height().await);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state
                        .get_hashrate(None)
                        .await
                        .to_json_response(&etag)
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
            "/api/v1/mining/hashrate/:time_period",
            get_with(
                async |headers: HeaderMap, Path(path): Path<TimePeriodPath>, State(state): State<AppState>| {
                    let etag = format!("{VERSION}-hashrate-{}-{:?}", state.get_height().await, path.time_period);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state
                        .get_hashrate(Some(path.time_period))
                        .await
                        .to_json_response(&etag)
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
                    let etag = format!("{VERSION}-diff-adj-{}", state.get_height().await);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state
                        .get_difficulty_adjustments(None)
                        .await
                        .to_json_response(&etag)
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
            "/api/v1/mining/difficulty-adjustments/:time_period",
            get_with(
                async |headers: HeaderMap, Path(path): Path<TimePeriodPath>, State(state): State<AppState>| {
                    let etag = format!("{VERSION}-diff-adj-{}-{:?}", state.get_height().await, path.time_period);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state
                        .get_difficulty_adjustments(Some(path.time_period))
                        .await
                        .to_json_response(&etag)
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
            "/api/v1/mining/blocks/fees/:time_period",
            get_with(
                async |headers: HeaderMap, Path(path): Path<TimePeriodPath>, State(state): State<AppState>| {
                    let etag = format!("{VERSION}-fees-{}-{:?}", state.get_height().await, path.time_period);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state
                        .get_block_fees(path.time_period)
                        .await
                        .to_json_response(&etag)
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
            "/api/v1/mining/blocks/rewards/:time_period",
            get_with(
                async |headers: HeaderMap, Path(path): Path<TimePeriodPath>, State(state): State<AppState>| {
                    let etag = format!("{VERSION}-rewards-{}-{:?}", state.get_height().await, path.time_period);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state
                        .get_block_rewards(path.time_period)
                        .await
                        .to_json_response(&etag)
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
        .api_route(
            "/api/v1/mining/blocks/fee-rates/:time_period",
            get_with(
                async |headers: HeaderMap, Path(path): Path<TimePeriodPath>, State(state): State<AppState>| {
                    let etag = format!("{VERSION}-feerates-{}-{:?}", state.get_height().await, path.time_period);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state
                        .get_block_fee_rates(path.time_period)
                        .await
                        .to_json_response(&etag)
                },
                |op| {
                    op.mining_tag()
                        .summary("Block fee rates")
                        .description("Get block fee rate percentiles (min, 10th, 25th, median, 75th, 90th, max) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y")
                        .ok_response::<Vec<BlockFeeRatesEntry>>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
        .api_route(
            "/api/v1/mining/blocks/sizes-weights/:time_period",
            get_with(
                async |headers: HeaderMap, Path(path): Path<TimePeriodPath>, State(state): State<AppState>| {
                    let etag = format!("{VERSION}-sizes-{}-{:?}", state.get_height().await, path.time_period);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state
                        .get_block_sizes_weights(path.time_period)
                        .await
                        .to_json_response(&etag)
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
            "/api/v1/mining/reward-stats/:block_count",
            get_with(
                async |headers: HeaderMap, Path(path): Path<BlockCountPath>, State(state): State<AppState>| {
                    let etag = format!("{VERSION}-reward-stats-{}-{}", state.get_height().await, path.block_count);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state
                        .get_reward_stats(path.block_count)
                        .await
                        .to_json_response(&etag)
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
