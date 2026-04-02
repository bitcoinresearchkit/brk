use schemars::JsonSchema;
use serde::Deserialize;

use brk_types::Addr;

#[derive(Deserialize, JsonSchema)]
pub struct AddrParam {
    #[serde(rename = "address")]
    pub addr: Addr,
}
