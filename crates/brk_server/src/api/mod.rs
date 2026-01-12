use std::{borrow::Cow, sync::Arc};

use aide::{
    axum::{ApiRouter, routing::get_with},
    openapi::OpenApi,
};
use axum::{
    Extension, Json,
    extract::State,
    http::HeaderMap,
    response::{Html, Redirect, Response},
    routing::get,
};
use brk_types::Health;

use crate::{
    CacheStrategy, VERSION,
    api::{
        addresses::AddressRoutes, blocks::BlockRoutes, mempool::MempoolRoutes,
        metrics::ApiMetricsRoutes, mining::MiningRoutes, transactions::TxRoutes,
    },
    extended::{HeaderMapExtended, ResponseExtended, TransformResponseExtended},
};

use super::AppState;

mod addresses;
mod blocks;
mod mempool;
mod metrics;
mod mining;
mod openapi;
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
            .route("/api/server", get(Redirect::temporary("/api#tag/server")))
            .api_route(
                "/version",
                get_with(
                    async |headers: HeaderMap, State(state): State<AppState>| {
                        state
                            .cached_json(&headers, CacheStrategy::Static, |_| {
                                Ok(env!("CARGO_PKG_VERSION"))
                            })
                            .await
                    },
                    |op| {
                        op.id("get_version")
                            .server_tag()
                            .summary("API version")
                            .description("Returns the current version of the API server")
                            .ok_response::<String>()
                            .not_modified()
                    },
                ),
            )
            .api_route(
                "/health",
                get_with(
                    async || -> Json<Health> {
                        Json(Health {
                            status: Cow::Borrowed("healthy"),
                            service: Cow::Borrowed("brk"),
                            timestamp: jiff::Timestamp::now().to_string(),
                        })
                    },
                    |op| {
                        op.id("get_health")
                            .server_tag()
                            .summary("Health check")
                            .description("Returns the health status of the API server")
                            .ok_response::<Health>()
                    },
                ),
            )
            .route(
                "/api.json",
                get(
                    async |headers: HeaderMap,
                           Extension(api): Extension<Arc<OpenApi>>|
                           -> Response {
                        let etag = VERSION;

                        if headers.has_etag(etag) {
                            return Response::new_not_modified();
                        }

                        Response::new_json(&api, etag)
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
