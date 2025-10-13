use crate::{TxPrevout, Txid, Vout};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize, JsonSchema)]
/// Transaction input
pub struct TxInput {
    /// Transaction ID of the output being spent
    pub txid: Txid,

    #[schemars(example = 0)]
    pub vout: Vout,

    /// Information about the previous output being spent
    pub prevout: Option<TxPrevout>,

    /// Signature script (for non-SegWit inputs)
    #[schemars(example = "")]
    pub scriptsig: String,

    /// Signature script in assembly format
    #[schemars(example = "")]
    pub scriptsig_asm: String,

    /// Witness data (for SegWit inputs)
    #[schemars(example = vec!["3045022100d0c9936990bf00bdba15f425f0f360a223d5cbf81f4bf8477fe6c6d838fb5fae02207e42a8325a4dd41702bf065aa6e0a1b7b0b8ee92a5e6c182da018b0afc82c40601".to_string()])]
    pub witness: Vec<String>,

    /// Whether this input is a coinbase (block reward) input
    #[schemars(example = false)]
    pub is_coinbase: bool,

    /// Input sequence number
    #[schemars(example = 429496729)]
    pub sequence: u32,

    /// Inner redeemscript in assembly format (for P2SH-wrapped SegWit)
    #[schemars(example = Some("OP_0 OP_PUSHBYTES_20 992a1f7420fc5285070d19c71ff2efb1e356ad2f".to_string()))]
    pub inner_redeemscript_asm: Option<String>,
}
