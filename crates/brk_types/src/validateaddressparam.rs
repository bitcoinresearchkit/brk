use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct ValidateAddressParam {
    /// Bitcoin address to validate (can be any string)
    pub address: String,
}
