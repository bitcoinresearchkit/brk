use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Sats, Transaction, Txid, VSize};

/// Simplified mempool transaction for the recent transactions endpoint
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MempoolRecentTx {
    pub txid: Txid,
    pub fee: Sats,
    pub vsize: VSize,
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
