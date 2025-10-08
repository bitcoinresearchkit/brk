use std::sync::Arc;

use aide::{
    axum::{ApiRouter, routing::get_with},
    openapi::OpenApi,
};
use axum::{
    Extension, Json,
    http::HeaderMap,
    response::{Html, Redirect, Response},
    routing::get,
};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    VERSION,
    api::{chain::ChainRoutes, metrics::ApiMetricsRoutes},
    extended::{HeaderMapExtended, ResponseExtended, TransformResponseExtended},
};

use super::AppState;

mod chain;
mod metrics;
mod openapi;

pub use openapi::*;

pub trait ApiRoutes {
    fn add_api_routes(self) -> Self;
}

#[derive(Debug, Serialize, JsonSchema)]
/// Server health status
struct Health {
    status: String,
    service: String,
    timestamp: String,
}

impl ApiRoutes for ApiRouter<AppState> {
    fn add_api_routes(self) -> Self {
        self.add_chain_routes()
            .add_metrics_routes()
            .route("/api/server", get(Redirect::temporary("/api#tag/server")))
            .api_route(
                "/version",
                get_with(
                    async || -> Json<&'static str> { Json(VERSION) },
                    |op| {
                        op.tag("Server")
                            .summary("API version")
                            .description("Returns the current version of the API server")
                            .with_ok_response::<String, _>(|res| res)
                    },
                ),
            )
            .api_route(
                "/health",
                get_with(
                    async || -> Json<Health> {
                        Json(Health {
                            status: "healthy".to_string(),
                            service: "brk-server".to_string(),
                            timestamp: jiff::Timestamp::now().to_string(),
                        })
                    },
                    |op| {
                        op.tag("Server")
                            .summary("Health check")
                            .description("Returns the health status of the API server")
                            .with_ok_response::<Health, _>(|res| res)
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

                        if headers
                            .get_if_none_match()
                            .is_some_and(|prev_etag| etag == prev_etag)
                        {
                            return Response::new_not_modified();
                        }

                        let mut response =
                            Response::new_json_from_bytes(sonic_rs::to_vec(&api).unwrap());

                        let headers = response.headers_mut();
                        headers.insert_cors();
                        headers.insert_etag(etag);

                        response
                    },
                ),
            )
            .route("/api", get(Html::from(include_str!("./scalar.html"))))
    }
}
