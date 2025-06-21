use axum::Router;
use brk_interface::Interface;
use rmcp::transport::{
    StreamableHttpServerConfig,
    streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
};

mod api;
use api::*;

use crate::AppState;

pub trait MCPRoutes {
    fn add_mcp_routes(self, interface: &'static Interface<'static>) -> Self;
}

impl MCPRoutes for Router<AppState> {
    fn add_mcp_routes(self, interface: &'static Interface<'static>) -> Self {
        let config = StreamableHttpServerConfig {
            // stateful_mode: false,
            ..Default::default()
        };

        let service = StreamableHttpService::new(
            move || Ok(API::new(interface)),
            LocalSessionManager::default().into(),
            config,
        );

        self.nest_service("/mcp", service)
    }
}
