use brk_types::OutputType;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct AddrHashPrefixParam {
    pub addr_type: OutputType,
    pub prefix: String,
}
