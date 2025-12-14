use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::{Redirect, Response},
    routing::get,
};
use brk_query::BLOCK_TXS_PAGE_SIZE;
use brk_types::{
    BlockHashPath, BlockHashStartIndexPath, BlockHashTxIndexPath, BlockInfo, BlockStatus,
    BlockTimestamp, Height, HeightPath, StartHeightPath, TimestampPath, Transaction, Txid,
};

use crate::{
    VERSION,
    extended::{HeaderMapExtended, ResponseExtended, ResultExtended, TransformResponseExtended},
};

use super::AppState;

pub trait BlockRoutes {
    fn add_block_routes(self) -> Self;
}

impl BlockRoutes for ApiRouter<AppState> {
    fn add_block_routes(self) -> Self {
        self.route("/api/block", get(Redirect::temporary("/api/blocks")))
            .route(
                "/api/blocks",
                get(Redirect::temporary("/api#tag/blocks")),
            )
            .api_route(
                "/api/block/{hash}",
                get_with(
                    async |headers: HeaderMap,
                           Path(path): Path<BlockHashPath>,
                           State(state): State<AppState>| {
                        let etag = format!("{VERSION}-{}", state.get_height().await);
                        if headers.has_etag(&etag) {
                            return Response::new_not_modified();
                        }
                        state.get_block(path.hash).await.to_json_response(&etag)
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
                           Path(path): Path<BlockHashPath>,
                           State(state): State<AppState>| {
                        let etag = format!("{VERSION}-{}", state.get_height().await);
                        if headers.has_etag(&etag) {
                            return Response::new_not_modified();
                        }
                        state
                            .get_block_status(path.hash)
                            .await
                            .to_json_response(&etag)
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
                           Path(path): Path<HeightPath>,
                           State(state): State<AppState>| {
                        let etag = format!("{VERSION}-{}", state.get_height().await);
                        if headers.has_etag(&etag) {
                            return Response::new_not_modified();
                        }
                        state
                            .get_block_by_height(Height::from(path.height))
                            .await
                            .to_json_response(&etag)
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
                "/api/blocks/{start_height}",
                get_with(
                    async |headers: HeaderMap,
                           Path(path): Path<StartHeightPath>,
                           State(state): State<AppState>| {
                        let etag = format!("{VERSION}-{}", state.get_height().await);
                        if headers.has_etag(&etag) {
                            return Response::new_not_modified();
                        }
                        let start_height = path.start_height.map(Height::from);
                        state.get_blocks(start_height).await.to_json_response(&etag)
                    },
                    |op| {
                        op.blocks_tag()
                            .summary("Recent blocks")
                            .description(
                                "Retrieve the last 10 blocks, optionally starting from a specific height. Returns block metadata for each block.",
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
                           Path(path): Path<BlockHashPath>,
                           State(state): State<AppState>| {
                        let etag = format!("{VERSION}-{}", state.get_height().await);
                        if headers.has_etag(&etag) {
                            return Response::new_not_modified();
                        }
                        state.get_block_txids(path.hash).await.to_json_response(&etag)
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
                           Path(path): Path<BlockHashStartIndexPath>,
                           State(state): State<AppState>| {
                        let etag = format!("{VERSION}-{}", state.get_height().await);
                        if headers.has_etag(&etag) {
                            return Response::new_not_modified();
                        }
                        state.get_block_txs(path.hash, path.start_index).await.to_json_response(&etag)
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
                           Path(path): Path<BlockHashTxIndexPath>,
                           State(state): State<AppState>| {
                        let etag = format!("{VERSION}-{}", state.get_height().await);
                        if headers.has_etag(&etag) {
                            return Response::new_not_modified();
                        }
                        state.get_block_txid_at_index(path.hash, path.index).await.to_display_response(&etag)
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
                           Path(path): Path<TimestampPath>,
                           State(state): State<AppState>| {
                        let etag = format!("{VERSION}-{}", state.get_height().await);
                        if headers.has_etag(&etag) {
                            return Response::new_not_modified();
                        }
                        state
                            .get_block_by_timestamp(path.timestamp)
                            .await
                            .to_json_response(&etag)
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
                           Path(path): Path<BlockHashPath>,
                           State(state): State<AppState>| {
                        let etag = format!("{VERSION}-{}", state.get_height().await);
                        if headers.has_etag(&etag) {
                            return Response::new_not_modified();
                        }
                        state.get_block_raw(path.hash).await.to_bytes_response(&etag)
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
