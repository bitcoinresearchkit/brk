use std::sync::Arc;

use axum::Router;
use brk_rmcp::transport::{
    StreamableHttpServerConfig,
    streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
};
use log::info;

use crate::MCP;

/// Create an MCP service router.
pub fn mcp_router(base_url: String, openapi_json: Arc<String>) -> Router {
    info!("Setting up MCP...");

    let service = StreamableHttpService::new(
        move || Ok(MCP::new(base_url.clone(), openapi_json.as_str())),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig {
            stateful_mode: false,
            ..Default::default()
        },
    );

    Router::new().nest_service("/mcp", service)
}
