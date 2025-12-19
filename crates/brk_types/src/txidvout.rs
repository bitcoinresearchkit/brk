use schemars::JsonSchema;
use serde::Deserialize;

use crate::{Txid, Vout};

/// Transaction output reference (txid + output index)
#[derive(Deserialize, JsonSchema)]
pub struct TxidVout {
    /// Transaction ID
    pub txid: Txid,
    /// Output index
    pub vout: Vout,
}
