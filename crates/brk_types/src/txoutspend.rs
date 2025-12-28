use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{TxStatus, Txid, Vin};

/// Status of an output indicating whether it has been spent
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TxOutspend {
    /// Whether the output has been spent
    pub spent: bool,

    /// Transaction ID of the spending transaction (only present if spent)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<Txid>,

    /// Input index in the spending transaction (only present if spent)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vin: Option<Vin>,

    /// Status of the spending transaction (only present if spent)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TxStatus>,
}

impl TxOutspend {
    pub const UNSPENT: Self = Self {
        spent: false,
        txid: None,
        vin: None,
        status: None,
    };
}
