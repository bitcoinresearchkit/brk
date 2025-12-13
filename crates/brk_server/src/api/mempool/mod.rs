use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::State,
    http::HeaderMap,
    response::{Redirect, Response},
    routing::get,
};
use brk_types::{MempoolInfo, RecommendedFees, Txid};

use crate::{
    VERSION,
    extended::{HeaderMapExtended, ResponseExtended, ResultExtended, TransformResponseExtended},
};

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
                        let etag = format!("{VERSION}-{}", state.get_height().await);
                        if headers.has_etag(&etag) {
                            return Response::new_not_modified();
                        }
                        state.get_mempool_info().await.to_json_response(&etag)
                    },
                    |op| {
                        op.mempool_tag()
                            .summary("Mempool statistics")
                            .description("Get current mempool statistics including transaction count, total vsize, and total fees.")
                            .ok_response::<MempoolInfo>()
                            .not_modified()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/mempool/txids",
                get_with(
                    async |headers: HeaderMap, State(state): State<AppState>| {
                        let etag = format!("{VERSION}-{}", state.get_height().await);
                        if headers.has_etag(&etag) {
                            return Response::new_not_modified();
                        }
                        state.get_mempool_txids().await.to_json_response(&etag)
                    },
                    |op| {
                        op.mempool_tag()
                            .summary("Mempool transaction IDs")
                            .description("Get all transaction IDs currently in the mempool.")
                            .ok_response::<Vec<Txid>>()
                            .not_modified()
                            .server_error()
                    },
                ),
            )
            .api_route(
                "/api/v1/fees/recommended",
                get_with(
                    async |headers: HeaderMap, State(state): State<AppState>| {
                        let etag = format!("{VERSION}-{}", state.get_height().await);
                        if headers.has_etag(&etag) {
                            return Response::new_not_modified();
                        }
                        state.get_recommended_fees().await.to_json_response(&etag)
                    },
                    |op| {
                        op.mempool_tag()
                            .summary("Recommended fees")
                            .description("Get recommended fee rates for different confirmation targets based on current mempool state.")
                            .ok_response::<RecommendedFees>()
                            .not_modified()
                            .server_error()
                    },
                ),
            )
    }
}
