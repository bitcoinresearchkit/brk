use std::sync::Arc;

use aide::{
    axum::{ApiRouter, routing::get_with},
    openapi::OpenApi,
};
use axum::{
    Extension,
    http::{HeaderMap, header},
    response::{Html, Redirect, Response},
    routing::get,
};

use crate::{
    api::{
        addresses::AddressRoutes, blocks::BlockRoutes, mempool::MempoolRoutes,
        metrics::ApiMetricsRoutes, mining::MiningRoutes, server::ServerRoutes,
        transactions::TxRoutes,
    },
    extended::{ResponseExtended, TransformResponseExtended},
};

use super::AppState;

mod addresses;
mod blocks;
mod mempool;
mod metrics;
mod mining;
mod openapi;
mod server;
mod transactions;

pub use openapi::*;

pub trait ApiRoutes {
    fn add_api_routes(self) -> Self;
}

impl ApiRoutes for ApiRouter<AppState> {
    fn add_api_routes(self) -> Self {
        self.add_addresses_routes()
            .add_block_routes()
            .add_mempool_routes()
            .add_mining_routes()
            .add_tx_routes()
            .add_metrics_routes()
            .add_server_routes()
            .route("/api/server", get(Redirect::temporary("/api#tag/server")))
            .api_route(
                "/openapi.json",
                get_with(
                    async |headers: HeaderMap,
                           Extension(api): Extension<Arc<OpenApi>>|
                           -> Response { Response::static_json(&headers, &*api) },
                    |op| {
                        op.id("get_openapi")
                            .server_tag()
                            .summary("OpenAPI specification")
                            .description("Full OpenAPI 3.1 specification for this API.")
                    },
                ),
            )
            .api_route(
                "/api.json",
                get_with(
                    async |headers: HeaderMap,
                           Extension(api): Extension<Arc<ApiJson>>|
                           -> Response {
                        Response::static_json(&headers, api.to_json())
                    },
                    |op| {
                        op.id("get_api")
                            .server_tag()
                            .summary("Compact OpenAPI specification")
                            .description(
                                "Compact OpenAPI specification optimized for LLM consumption. \
                                 Removes redundant fields while preserving essential API information. \
                                 Full spec available at `/openapi.json`.",
                            )
                            .ok_response::<serde_json::Value>()
                    },
                ),
            )
            .route("/api", get(Html::from(include_str!("./scalar.html"))))
            // Pre-compressed with: brotli -c -q 11 scalar.js > scalar.js.br
            .route("/scalar.js", get(|| async {
                (
                    [
                        (header::CONTENT_TYPE, "application/javascript"),
                        (header::CONTENT_ENCODING, "br"),
                    ],
                    include_bytes!("./scalar.js.br").as_slice(),
                )
            }))
            .route(
                "/api/{*path}",
                get(|| async { Redirect::permanent("/api") }),
            )
    }
}
