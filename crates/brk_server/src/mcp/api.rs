use brk_interface::{Interface, Pagination, Params};
use rmcp::{
    Error as McpError, RoleServer, ServerHandler,
    model::{
        CallToolResult, Content, Implementation, InitializeRequestParam, InitializeResult,
        ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    service::RequestContext,
    tool,
};

#[derive(Clone)]
pub struct API {
    interface: &'static Interface<'static>,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tool(tool_box)]
impl API {
    pub fn new(interface: &'static Interface<'static>) -> Self {
        Self { interface }
    }

    #[tool(description = "
Get the count of all existing indexes
")]
    async fn get_index_count(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.get_index_count()).unwrap(),
        ]))
    }

    #[tool(description = "
Get the count of all existing vec ids
")]
    async fn get_vecid_count(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.get_vecid_count()).unwrap(),
        ]))
    }

    #[tool(description = "
Get the count of all existing vecs
")]
    async fn get_variant_count(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.get_vec_count()).unwrap(),
        ]))
    }

    #[tool(description = "
Get the list of all existing indexes
")]
    async fn get_indexes(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.get_indexes()).unwrap(),
        ]))
    }

    #[tool(description = "
Get an object which has all existing indexes as keys and a list of their accepted variants as values
")]
    async fn get_accepted_indexes(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.get_accepted_indexes()).unwrap(),
        ]))
    }

    #[tool(description = "
Get the list of all existing vec ids
")]
    async fn get_vecids(
        &self,
        #[tool(aggr)] pagination: Pagination,
    ) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.get_vecids(pagination)).unwrap(),
        ]))
    }

    #[tool(description = "
Get an object which has all existing indexes as keys and a list of ids of vecs which support that index as values
")]
    async fn get_indexes_to_vecids(
        &self,
        #[tool(aggr)] pagination: Pagination,
    ) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.get_indexes_to_vecids(pagination)).unwrap(),
        ]))
    }

    #[tool(description = "
Get an object which has all existing vec ids as keys and a list of indexes supported by that vec id as values
")]
    async fn get_vecids_to_indexes(
        &self,
        #[tool(aggr)] pagination: Pagination,
    ) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.get_vecids_to_indexes(pagination)).unwrap(),
        ]))
    }

    #[tool(description = "Get one or multiple vecs depending on given parameters")]
    fn get_vecs(&self, #[tool(aggr)] params: Params) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![
            Content::json(self.interface.search_and_format(params).unwrap()).unwrap(),
        ]))
    }

    #[tool(description = "
Get the running version of the Bitcoin Research Kit
")]
    async fn get_version(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(format!(
            "v{VERSION}"
        ))]))
    }
}

#[tool(tool_box)]
impl ServerHandler for API {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "
This server provides an interface to communicate with a running instance of the Bitcoin Research Kit (brk).
Multiple tools are at your disposal including ones to fetch all sorts of Bitcoin on-chain data.
Arrays are also called Vectors (or Vecs).
"
                .to_string(),
            ),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        if let Some(http_request_part) = context.extensions.get::<axum::http::request::Parts>() {
            let initialize_headers = &http_request_part.headers;
            let initialize_uri = &http_request_part.uri;
            tracing::info!(?initialize_headers, %initialize_uri, "initialize from http server");
        }
        Ok(self.get_info())
    }
}
