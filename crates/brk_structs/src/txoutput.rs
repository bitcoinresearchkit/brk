use crate::Sats;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize, JsonSchema)]
/// Transaction output
pub struct TxOutput {
    /// Script pubkey (locking script)
    #[schemars(example = "00143b064c595a95f977f00352d6e917501267cacdc6")]
    pub scriptpubkey: String,

    /// Script pubkey in assembly format
    #[schemars(example = "OP_0 OP_PUSHBYTES_20 3b064c595a95f977f00352d6e917501267cacdc6")]
    pub scriptpubkey_asm: String,

    /// Script type (p2pk, p2pkh, p2sh, p2wpkh, p2wsh, p2tr, op_return, etc.)
    #[schemars(example = &"v0_p2wpkh")]
    pub scriptpubkey_type: String,

    /// Bitcoin address (if applicable, None for OP_RETURN)
    #[schemars(example = Some("bc1q8vryck26jhuh0uqr2ttwj96szfnu4nwxfmu39y".to_string()))]
    pub scriptpubkey_address: Option<String>,

    /// Value of the output in satoshis
    #[schemars(example = Sats::new(7782))]
    pub value: Sats,
}
