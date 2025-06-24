use axum::Router;
use brk_interface::Interface;
use brk_rmcp::transport::{
    StreamableHttpServerConfig,
    streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
};

use log::info;

use crate::MCP;

pub trait MCPRoutes {
    fn add_mcp_routes(self, interface: &'static Interface<'static>, mcp: bool) -> Self;
}

impl<T> MCPRoutes for Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn add_mcp_routes(self, interface: &'static Interface<'static>, mcp: bool) -> Self {
        if !mcp {
            return self;
        }

        let config = StreamableHttpServerConfig {
            // stateful_mode: false, // breaks Claude
            ..Default::default()
        };

        let service = StreamableHttpService::new(
            move || Ok(MCP::new(interface)),
            LocalSessionManager::default().into(),
            config,
        );

        info!("Setting MCP...");

        self.nest_service("/mcp", service)
    }
}
