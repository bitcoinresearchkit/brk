use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Sats, TxStatus, Txid, Vout};

/// Unspent transaction output
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Utxo {
    pub txid: Txid,
    pub vout: Vout,
    pub status: TxStatus,
    pub value: Sats,
}
