#![doc = include_str!("../README.md")]

use brk_interface::{Interface, PaginatedIndexParam, PaginationParam, Params};
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
    interface: &'static Interface<'static>,
    tool_router: ToolRouter<MCP>,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tool_router]
impl MCP {
    pub fn new(interface: &'static Interface<'static>) -> Self {
        Self {
            interface,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "
Get the count of unique metrics.
")]
    async fn get_metric_count(&self) -> Result<CallToolResult, McpError> {
        info!("mcp: distinct_metric_count");
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.distinct_metric_count()).unwrap(),
        ]))
    }

    #[tool(description = "
Get the count of all metrics. (distinct metrics multiplied by the number of indexes supported by each one)
")]
    async fn get_vec_count(&self) -> Result<CallToolResult, McpError> {
        info!("mcp: total_metric_count");
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.total_metric_count()).unwrap(),
        ]))
    }

    #[tool(description = "
Get the list of all existing indexes and their accepted variants.
")]
    async fn get_indexes(&self) -> Result<CallToolResult, McpError> {
        info!("mcp: get_indexes");
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.get_indexes()).unwrap(),
        ]))
    }

    #[tool(description = "
Get a paginated list of all existing vec ids.
There are up to 1,000 values per page.
If the `page` param is omitted, it will default to the first page.
")]
    async fn get_vecids(
        &self,
        Parameters(pagination): Parameters<PaginationParam>,
    ) -> Result<CallToolResult, McpError> {
        info!("mcp: get_metrics");
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.get_metrics(pagination)).unwrap(),
        ]))
    }

    #[tool(description = "
Get a paginated list of all vec ids which support a given index.
There are up to 1,000 values per page.
If the `page` param is omitted, it will default to the first page.
")]
    async fn get_index_to_vecids(
        &self,
        Parameters(paginated_index): Parameters<PaginatedIndexParam>,
    ) -> Result<CallToolResult, McpError> {
        info!("mcp: get_index_to_vecids");
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.get_index_to_vecids(paginated_index)).unwrap(),
        ]))
    }

    #[tool(description = "
Get a list of all indexes supported by a given vec id.
The list will be empty if the vec id isn't correct.
")]
    async fn get_vecid_to_indexes(
        &self,
        Parameters(param): Parameters<IdParam>,
    ) -> Result<CallToolResult, McpError> {
        info!("mcp: get_vecid_to_indexes");
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.metric_to_indexes(param.id)).unwrap(),
        ]))
    }

    #[tool(description = "
Get one or multiple vecs depending on given parameters.
The response's format will depend on the given parameters, it will be:
- A value: If requested only one vec and the given range returns one value (for example: `from=-1`)
- A list: If requested only one vec and the given range returns multiple values (for example: `from=-1000&count=100` or `from=-444&to=-333`)
- A matrix: When multiple vecs are requested, even if they each return one value.
")]
    fn get_vecs(&self, Parameters(params): Parameters<Params>) -> Result<CallToolResult, McpError> {
        info!("mcp: get_vecs");
        Ok(CallToolResult::success(vec![Content::text(
            match self.interface.search_and_format(params) {
                Ok(output) => output.to_string(),
                Err(e) => format!("Error:\n{e}"),
            },
        )]))
    }

    #[tool(description = "
Get the running version of the Bitcoin Research Kit.
")]
    async fn get_version(&self) -> Result<CallToolResult, McpError> {
        info!("mcp: get_version");
        Ok(CallToolResult::success(vec![Content::text(format!(
            "v{VERSION}"
        ))]))
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
This server provides an interface to communicate with a running instance of the Bitcoin Research Kit (also called brk or BRK).

Multiple tools are at your disposal including ones to fetch all sorts of Bitcoin on-chain metrics and market prices.

If you're unsure which datasets are available, try out different tools before browsing the web. Each tool gives important information about BRK's capabilities.

Vectors can also be called 'Vecs', 'Arrays' or 'Datasets', they can all be used interchangeably.

An 'Index' (or indexes) is the timeframe of a dataset.

'VecId' (or vecids) are the name of the dataset and what it represents.
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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct IdParam {
    pub id: String,
}
