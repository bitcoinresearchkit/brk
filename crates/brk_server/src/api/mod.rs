use std::sync::Arc;

use aide::{
    axum::{ApiRouter, routing::get_with},
    openapi::{Info, OpenApi, Tag},
};
use axum::{Extension, Json, response::Html, routing::get};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    VERSION,
    api::{chain::ChainRoutes, metrics::ApiMetricsRoutes},
};

use super::AppState;

mod chain;
mod metrics;

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
            .add_api_metrics_routes()
            .api_route(
                "/version",
                get_with(
                    async || -> Json<&'static str> { Json(VERSION) },
                    |op| {
                        op.tag("Server")
                            .summary("API version")
                            .description("Returns the current version of the API server")
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
                    },
                ),
            )
            .route(
                "/api.json",
                get(
                    async |Extension(api): Extension<Arc<OpenApi>>| -> Json<Arc<OpenApi>> {
                        Json(api)
                    },
                ),
            )
            .route("/api", get(Html::from(include_str!("./scalar.html"))))
    }
}

pub fn create_openapi() -> OpenApi {
    let tags = vec![
        Tag {
            name: "Chain".to_string(),
            description: Some(
                "Explore Bitcoin blockchain data: addresses, transactions, blocks, balances, and UTXOs."
                    .to_string()
            ),
            ..Default::default()
        },
        Tag {
            name: "Metrics".to_string(),
            description: Some(
                "Access Bitcoin network metrics and time-series data. Query historical and real-time \
                statistics across various blockchain dimensions and aggregation levels."
                    .to_string()
            ),
            ..Default::default()
        },
        Tag {
            name: "Server".to_string(),
            description: Some(
                "Metadata and utility endpoints for API status, health checks, and system information."
                    .to_string()
            ),
                ..Default::default()
            },
    ];

    OpenApi {
        info: Info {
            title: "Bitcoin Research Kit API".to_string(),
            description: Some(
                "API for querying Bitcoin blockchain data including addresses, transactions, and chain statistics. This API provides low-level access to indexed blockchain data with advanced analytics capabilities.\n\n\
                ⚠️ **Early Development**: This API is in very early stages of development. Breaking changes may occur without notice. For a more stable experience, self-host or use the hosting service."
                    .to_string(),
            ),
            version: format!("v{VERSION}"),
            ..Info::default()
        },
        tags,
        ..OpenApi::default()
    }
}
