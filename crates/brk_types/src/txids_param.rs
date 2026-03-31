use schemars::JsonSchema;
use serde::Deserialize;

use crate::Txid;

#[derive(Deserialize, JsonSchema)]
pub struct TxidsParam {
    #[serde(rename = "txId[]")]
    pub txids: Vec<Txid>,
}
