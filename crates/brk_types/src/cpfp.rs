use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{FeeRate, Sats, Txid, Weight};

/// CPFP (Child Pays For Parent) information for a transaction
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CpfpInfo {
    pub ancestors: Vec<CpfpEntry>,
    pub descendants: Vec<CpfpEntry>,
    #[serde(rename = "effectiveFeePerVsize")]
    pub effective_fee_per_vsize: FeeRate,
}

/// A transaction in a CPFP relationship
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CpfpEntry {
    pub txid: Txid,
    pub weight: Weight,
    pub fee: Sats,
}
