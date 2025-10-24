use aide::axum::ApiRouter;
use brk_query::Query;
use brk_rmcp::transport::{
    StreamableHttpServerConfig,
    streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
};

use log::info;

use crate::MCP;

pub trait MCPRoutes {
    fn add_mcp_routes(self, query: &Query, mcp: bool) -> Self;
}

impl<T> MCPRoutes for ApiRouter<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn add_mcp_routes(self, query: &Query, mcp: bool) -> Self {
        if !mcp {
            return self;
        }

        let query = query.clone();
        let service = StreamableHttpService::new(
            move || Ok(MCP::new(&query)),
            LocalSessionManager::default().into(),
            StreamableHttpServerConfig {
                stateful_mode: false,
                ..Default::default()
            },
        );

        info!("Setting MCP...");

        self.nest_service("/mcp", service)
    }
}
