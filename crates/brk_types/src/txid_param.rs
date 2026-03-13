use schemars::JsonSchema;
use serde::Deserialize;

use crate::Txid;

#[derive(Deserialize, JsonSchema)]
pub struct TxidParam {
    pub txid: Txid,
}
