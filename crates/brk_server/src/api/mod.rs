use std::sync::Arc;

use aide::{
    axum::{ApiRouter, routing::get_with},
    openapi::OpenApi,
};
use axum::{
    Extension,
    http::HeaderMap,
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
                "/api.json",
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
                "/api.trimmed.json",
                get_with(
                    async |headers: HeaderMap,
                           Extension(api_trimmed): Extension<Arc<String>>|
                           -> Response {
                        let value: serde_json::Value =
                            serde_json::from_str(&api_trimmed).unwrap();
                        Response::static_json(&headers, &value)
                    },
                    |op| {
                        op.id("get_openapi_trimmed")
                            .server_tag()
                            .summary("Trimmed OpenAPI specification")
                            .description(
                                "Compact OpenAPI specification optimized for LLM consumption. \
                                 Removes redundant fields while preserving essential API information.",
                            )
                            .ok_response::<serde_json::Value>()
                    },
                ),
            )
            .route("/api", get(Html::from(include_str!("./scalar.html"))))
            .route(
                "/api/{*path}",
                get(|| async { Redirect::permanent("/api") }),
            )
    }
}
