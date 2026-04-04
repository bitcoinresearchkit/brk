use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{FeeRate, Sats, Txid, VSize, Weight};

/// CPFP (Child Pays For Parent) information for a transaction
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CpfpInfo {
    /// Ancestor transactions in the CPFP chain
    pub ancestors: Vec<CpfpEntry>,
    /// Best (highest fee rate) descendant, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_descendant: Option<CpfpEntry>,
    /// Descendant transactions in the CPFP chain
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub descendants: Vec<CpfpEntry>,
    /// Effective fee rate considering CPFP relationships (sat/vB)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_fee_per_vsize: Option<FeeRate>,
    /// Transaction fee (sats)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<Sats>,
    /// Adjusted virtual size (accounting for sigops)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjusted_vsize: Option<VSize>,
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
