use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
};
use brk_query::BLOCK_TXS_PAGE_SIZE;
use brk_types::{
    BlockHashParam, BlockHashStartIndex, BlockHashTxIndex, BlockInfo, BlockStatus, BlockTimestamp,
    HeightParam, TimestampParam, Transaction, Txid,
};

use crate::{CacheStrategy, extended::TransformResponseExtended};

use super::AppState;

pub trait BlockRoutes {
    fn add_block_routes(self) -> Self;
}

impl BlockRoutes for ApiRouter<AppState> {
    fn add_block_routes(self) -> Self {
        self.api_route(
                "/api/blocks",
                get_with(
                    async |headers: HeaderMap, State(state): State<AppState>| {
                        state
                            .cached_json(&headers, CacheStrategy::Height, move |q| q.blocks(None))
                            .await
                    },
                    |op| {
                        op.blocks_tag()
                            .summary("Recent blocks")
                            .description("Retrieve the last 10 blocks. Returns block metadata for each block.")
                            .ok_response::<Vec<BlockInfo>>()
                            .not_modified()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/block/{hash}",
                get_with(
                    async |headers: HeaderMap,
                           Path(path): Path<BlockHashParam>,
                           State(state): State<AppState>| {
                        state.cached_json(&headers, CacheStrategy::Height, move |q| q.block(&path.hash)).await
                    },
                    |op| {
                        op.blocks_tag()
                            .summary("Block information")
                            .description(
                                "Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count.",
                            )
                            .ok_response::<BlockInfo>()
                            .not_modified()
                            .bad_request()
                            .not_found()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/block/{hash}/status",
                get_with(
                    async |headers: HeaderMap,
                           Path(path): Path<BlockHashParam>,
                           State(state): State<AppState>| {
                        state.cached_json(&headers, CacheStrategy::Height, move |q| q.block_status(&path.hash)).await
                    },
                    |op| {
                        op.blocks_tag()
                            .summary("Block status")
                            .description(
                                "Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block.",
                            )
                            .ok_response::<BlockStatus>()
                            .not_modified()
                            .bad_request()
                            .not_found()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/block-height/{height}",
                get_with(
                    async |headers: HeaderMap,
                           Path(path): Path<HeightParam>,
                           State(state): State<AppState>| {
                        state.cached_json(&headers, CacheStrategy::Height, move |q| q.block_by_height(path.height)).await
                    },
                    |op| {
                        op.blocks_tag()
                            .summary("Block by height")
                            .description(
                                "Retrieve block information by block height. Returns block metadata including hash, timestamp, difficulty, size, weight, and transaction count.",
                            )
                            .ok_response::<BlockInfo>()
                            .not_modified()
                            .bad_request()
                            .not_found()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/blocks/{height}",
                get_with(
                    async |headers: HeaderMap,
                           Path(path): Path<HeightParam>,
                           State(state): State<AppState>| {
                        state.cached_json(&headers, CacheStrategy::Height, move |q| q.blocks(Some(path.height))).await
                    },
                    |op| {
                        op.blocks_tag()
                            .summary("Blocks from height")
                            .description(
                                "Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0.",
                            )
                            .ok_response::<Vec<BlockInfo>>()
                            .not_modified()
                            .bad_request()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/block/{hash}/txids",
                get_with(
                    async |headers: HeaderMap,
                           Path(path): Path<BlockHashParam>,
                           State(state): State<AppState>| {
                        state.cached_json(&headers, CacheStrategy::Height, move |q| q.block_txids(&path.hash)).await
                    },
                    |op| {
                        op.blocks_tag()
                            .summary("Block transaction IDs")
                            .description(
                                "Retrieve all transaction IDs in a block by block hash.",
                            )
                            .ok_response::<Vec<Txid>>()
                            .not_modified()
                            .bad_request()
                            .not_found()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/block/{hash}/txs/{start_index}",
                get_with(
                    async |headers: HeaderMap,
                           Path(path): Path<BlockHashStartIndex>,
                           State(state): State<AppState>| {
                        state.cached_json(&headers, CacheStrategy::Height, move |q| q.block_txs(&path.hash, path.start_index)).await
                    },
                    |op| {
                        op.blocks_tag()
                            .summary("Block transactions (paginated)")
                            .description(&format!(
                                "Retrieve transactions in a block by block hash, starting from the specified index. Returns up to {} transactions at a time.",
                                BLOCK_TXS_PAGE_SIZE
                            ))
                            .ok_response::<Vec<Transaction>>()
                            .not_modified()
                            .bad_request()
                            .not_found()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/block/{hash}/txid/{index}",
                get_with(
                    async |headers: HeaderMap,
                           Path(path): Path<BlockHashTxIndex>,
                           State(state): State<AppState>| {
                        state.cached_text(&headers, CacheStrategy::Height, move |q| q.block_txid_at_index(&path.hash, path.index).map(|t| t.to_string())).await
                    },
                    |op| {
                        op.blocks_tag()
                            .summary("Transaction ID at index")
                            .description(
                                "Retrieve a single transaction ID at a specific index within a block. Returns plain text txid.",
                            )
                            .ok_response::<Txid>()
                            .not_modified()
                            .bad_request()
                            .not_found()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/v1/mining/blocks/timestamp/{timestamp}",
                get_with(
                    async |headers: HeaderMap,
                           Path(path): Path<TimestampParam>,
                           State(state): State<AppState>| {
                        state.cached_json(&headers, CacheStrategy::Height, move |q| q.block_by_timestamp(path.timestamp)).await
                    },
                    |op| {
                        op.blocks_tag()
                            .summary("Block by timestamp")
                            .description("Find the block closest to a given UNIX timestamp.")
                            .ok_response::<BlockTimestamp>()
                            .not_modified()
                            .bad_request()
                            .not_found()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/block/{hash}/raw",
                get_with(
                    async |headers: HeaderMap,
                           Path(path): Path<BlockHashParam>,
                           State(state): State<AppState>| {
                        state.cached_bytes(&headers, CacheStrategy::Height, move |q| q.block_raw(&path.hash)).await
                    },
                    |op| {
                        op.blocks_tag()
                            .summary("Raw block")
                            .description(
                                "Returns the raw block data in binary format.",
                            )
                            .ok_response::<Vec<u8>>()
                            .not_modified()
                            .bad_request()
                            .not_found()
                            .server_error()
                    },
                ),
            )
    }
}
