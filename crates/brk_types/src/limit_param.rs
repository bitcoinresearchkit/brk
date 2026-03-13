use schemars::JsonSchema;
use serde::Deserialize;

use crate::Limit;

#[derive(Deserialize, JsonSchema)]
pub struct LimitParam {
    #[serde(default)]
    pub limit: Limit,
}
