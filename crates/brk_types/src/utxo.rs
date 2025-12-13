use schemars::JsonSchema;
use serde::Serialize;

use crate::{Sats, TxStatus, Txid, Vout};

/// Unspent transaction output
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Utxo {
    pub txid: Txid,
    pub vout: Vout,
    pub status: TxStatus,
    pub value: Sats,
}
