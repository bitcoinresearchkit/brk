use schemars::JsonSchema;
use serde::Deserialize;

use crate::Address;

#[derive(Deserialize, JsonSchema)]
pub struct AddressParam {
    pub address: Address,
}
