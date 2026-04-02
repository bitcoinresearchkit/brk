use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{FeeRate, Sats, Txid, VSize, Weight};

/// CPFP (Child Pays For Parent) information for a transaction
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CpfpInfo {
    /// Ancestor transactions in the CPFP chain
    pub ancestors: Vec<CpfpEntry>,
    /// Best (highest fee rate) descendant, if any
    pub best_descendant: Option<CpfpEntry>,
    /// Descendant transactions in the CPFP chain
    pub descendants: Vec<CpfpEntry>,
    /// Effective fee rate considering CPFP relationships (sat/vB)
    pub effective_fee_per_vsize: FeeRate,
    /// Transaction fee (sats)
    pub fee: Sats,
    /// Adjusted virtual size (accounting for sigops)
    pub adjusted_vsize: VSize,
}

/// A transaction in a CPFP relationship
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CpfpEntry {
    /// Transaction ID
    pub txid: Txid,
    /// Transaction weight
    pub weight: Weight,
    /// Transaction fee (sats)
    pub fee: Sats,
}
