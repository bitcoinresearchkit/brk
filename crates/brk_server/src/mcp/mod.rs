use axum::Router;
use brk_interface::Interface;
use brk_rmcp::transport::{
    StreamableHttpServerConfig,
    streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
};

mod api;
use api::*;
use log::info;

use crate::AppState;

pub trait MCPRoutes {
    fn add_mcp_routes(self, interface: &'static Interface<'static>, mcp: bool) -> Self;
}

impl MCPRoutes for Router<AppState> {
    fn add_mcp_routes(self, interface: &'static Interface<'static>, mcp: bool) -> Self {
        if !mcp {
            return self;
        }

        let config = StreamableHttpServerConfig {
            // stateful_mode: false, // breaks Claude
            ..Default::default()
        };

        let service = StreamableHttpService::new(
            move || Ok(API::new(interface)),
            LocalSessionManager::default().into(),
            config,
        );

        info!("Setting MCP...");

        self.nest_service("/mcp", service)
    }
}
