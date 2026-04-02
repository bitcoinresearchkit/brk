use crate::{TxOut, Txid, Vout};
use bitcoin::ScriptBuf;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};

/// Transaction input
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct TxIn {
    /// Transaction ID of the output being spent
    #[schemars(example = "0000000000000000000000000000000000000000000000000000000000000000")]
    pub txid: Txid,

    /// Output index being spent
    #[schemars(example = 0)]
    pub vout: Vout,

    /// Information about the previous output being spent
    #[schemars(example = None as Option<TxOut>)]
    pub prevout: Option<TxOut>,

    /// Signature script (hex, for non-SegWit inputs)
    #[schemars(rename = "scriptsig", with = "String")]
    pub script_sig: ScriptBuf,

    /// Signature script in assembly format
    #[schemars(rename = "scriptsig_asm", with = "String")]
    pub script_sig_asm: (),

    /// Witness data (hex-encoded stack items, present for SegWit inputs)
    pub witness: Vec<String>,

    /// Whether this input is a coinbase (block reward) input
    #[schemars(example = false)]
    pub is_coinbase: bool,

    /// Input sequence number
    #[schemars(example = 4294967293_u32)]
    pub sequence: u32,

    /// Inner redeemscript in assembly (for P2SH-wrapped SegWit: scriptsig + witness both present)
    #[schemars(rename = "inner_redeemscript_asm", with = "String")]
    pub inner_redeem_script_asm: (),

    /// Inner witnessscript in assembly (for P2WSH: last witness item decoded as script)
    #[schemars(rename = "inner_witnessscript_asm", with = "String")]
    pub inner_witness_script_asm: (),
}

impl Serialize for TxIn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let has_witness = !self.witness.is_empty();
        let has_scriptsig = !self.script_sig.is_empty();

        // P2SH-wrapped SegWit: both scriptsig and witness present
        let inner_redeem = if has_scriptsig && has_witness {
            self.script_sig
                .redeem_script()
                .map(|s| s.to_asm_string())
                .unwrap_or_default()
        } else {
            String::new()
        };

        // P2WSH: witness has >2 items, last is the witnessScript
        let inner_witness = if has_witness && !has_scriptsig && self.witness.len() > 2 {
            if let Some(last) = self.witness.last() {
                let bytes: Vec<u8> =
                    bitcoin::hex::FromHex::from_hex(last).unwrap_or_default();
                ScriptBuf::from(bytes).to_asm_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let has_inner_redeem = !inner_redeem.is_empty();
        let has_inner_witness = !inner_witness.is_empty();
        let field_count =
            7 + has_witness as usize + has_inner_redeem as usize + has_inner_witness as usize;

        let mut state = serializer.serialize_struct("TxIn", field_count)?;

        state.serialize_field("txid", &self.txid)?;
        state.serialize_field("vout", &self.vout)?;
        state.serialize_field("prevout", &self.prevout)?;
        state.serialize_field("scriptsig", &self.script_sig.to_hex_string())?;
        state.serialize_field("scriptsig_asm", &self.script_sig.to_asm_string())?;
        if has_witness {
            state.serialize_field("witness", &self.witness)?;
        }
        state.serialize_field("is_coinbase", &self.is_coinbase)?;
        state.serialize_field("sequence", &self.sequence)?;
        if has_inner_redeem {
            state.serialize_field("inner_redeemscript_asm", &inner_redeem)?;
        }
        if has_inner_witness {
            state.serialize_field("inner_witnessscript_asm", &inner_witness)?;
        }

        state.end()
    }
}
