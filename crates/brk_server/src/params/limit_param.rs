use schemars::JsonSchema;
use serde::Deserialize;

use brk_types::Limit;

#[derive(Deserialize, JsonSchema)]
pub struct LimitParam {
    #[serde(default)]
    pub limit: Limit,
}
