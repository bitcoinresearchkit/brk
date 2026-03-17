use schemars::JsonSchema;
use serde::Deserialize;

use crate::Addr;

#[derive(Deserialize, JsonSchema)]
pub struct AddrParam {
    #[serde(rename = "address")]
    pub addr: Addr,
}
