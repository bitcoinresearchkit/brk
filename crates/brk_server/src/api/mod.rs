use std::sync::Arc;

use aide::{
    axum::ApiRouter,
    openapi::OpenApi,
};
use axum::{
    Extension,
    http::HeaderMap,
    response::{Html, Redirect, Response},
    routing::get,
};

use crate::{
    VERSION,
    api::{
        addresses::AddressRoutes, blocks::BlockRoutes, mempool::MempoolRoutes,
        metrics::ApiMetricsRoutes, mining::MiningRoutes, server::ServerRoutes,
        transactions::TxRoutes,
    },
    extended::{HeaderMapExtended, ResponseExtended},
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
