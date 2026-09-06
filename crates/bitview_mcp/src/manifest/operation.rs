use rmcp::model::Tool;
use serde::Deserialize;

use super::HttpOperation;

#[derive(Debug, Clone, Deserialize)]
pub struct Operation {
    pub tool: Tool,
    pub http: HttpOperation,
}
