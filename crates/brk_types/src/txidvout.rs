use schemars::JsonSchema;
use serde::Deserialize;

use crate::{Txid, Vout};

#[derive(Deserialize, JsonSchema)]
pub struct TxidVout {
    pub txid: Txid,
    pub vout: Vout,
}
