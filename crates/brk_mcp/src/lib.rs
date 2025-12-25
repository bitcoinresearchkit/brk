#![doc = include_str!("../README.md")]

use std::sync::Arc;

use brk_rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use log::info;
use schemars::JsonSchema;
use serde::Deserialize;

pub mod route;

#[derive(Clone)]
pub struct MCP {
    base_url: Arc<String>,
    openapi_json: Arc<String>,
    tool_router: ToolRouter<MCP>,
}

/// Parameters for fetching from the REST API.
#[derive(Deserialize, JsonSchema)]
pub struct FetchParams {
    /// API path (e.g., "/api/blocks" or "/api/metrics/list")
    pub path: String,
    /// Optional query string (e.g., "page=0" or "from=-1&to=-10")
    pub query: Option<String>,
}

#[tool_router]
impl MCP {
    pub fn new(base_url: impl Into<String>, openapi_json: impl Into<String>) -> Self {
        Self {
            base_url: Arc::new(base_url.into()),
            openapi_json: Arc::new(openapi_json.into()),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get the OpenAPI specification describing all available REST API endpoints.")]
    async fn get_openapi(&self) -> Result<CallToolResult, McpError> {
        info!("mcp: get_openapi");
        Ok(CallToolResult::success(vec![Content::text(
            self.openapi_json.as_str(),
        )]))
    }

    #[tool(description = "Call a REST API endpoint. Use get_openapi first to discover available endpoints.")]
    async fn fetch(
        &self,
        Parameters(params): Parameters<FetchParams>,
    ) -> Result<CallToolResult, McpError> {
        info!("mcp: fetch {}", params.path);

        let url = match &params.query {
            Some(q) if !q.is_empty() => format!("{}{}?{}", self.base_url, params.path, q),
            _ => format!("{}{}", self.base_url, params.path),
        };

        match minreq::get(&url).send() {
            Ok(response) => {
                let body = response.as_str().unwrap_or("").to_string();
                Ok(CallToolResult::success(vec![Content::text(body)]))
            }
            Err(e) => Err(McpError::internal_error(
                format!("HTTP request failed: {e}"),
                None,
            )),
        }
    }
}

#[tool_handler]
impl ServerHandler for MCP {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "
Bitcoin Research Kit (BRK) - Bitcoin on-chain metrics and market data.

Workflow:
1. Call get_openapi to get the full API specification
2. Use fetch to call any endpoint described in the spec

Example: fetch with path=\"/api/metrics/list\" to list metrics.
"
                .to_string(),
            ),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
}
