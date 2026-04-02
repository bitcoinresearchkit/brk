use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Sats, Transaction, Txid, VSize};

/// Simplified mempool transaction for the `/api/mempool/recent` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MempoolRecentTx {
    /// Transaction ID
    pub txid: Txid,
    /// Transaction fee (sats)
    pub fee: Sats,
    /// Virtual size (vbytes)
    pub vsize: VSize,
    /// Total output value (sats)
    pub value: Sats,
}

impl From<(&Txid, &Transaction)> for MempoolRecentTx {
    fn from((txid, tx): (&Txid, &Transaction)) -> Self {
        Self {
            txid: txid.clone(),
            fee: tx.fee,
            vsize: tx.vsize(),
            value: tx.output.iter().map(|o| o.value).sum(),
        }
    }
}
