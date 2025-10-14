use crate::{TxOut, Txid, Vout};
use bitcoin::{Script, ScriptBuf};
use bitcoincore_rpc::{Client, RpcApi};
use schemars::JsonSchema;
use serde::{Serialize, Serializer, ser::SerializeStruct};

#[derive(Debug, JsonSchema)]
/// Transaction input
pub struct TxIn {
    /// Transaction ID of the output being spent
    #[schemars(example = "0000000000000000000000000000000000000000000000000000000000000000")]
    pub txid: Txid,

    #[schemars(example = 0)]
    pub vout: Vout,

    /// Information about the previous output being spent
    #[schemars(example = None as Option<TxOut>)]
    pub prevout: Option<TxOut>,

    /// Signature script (for non-SegWit inputs)
    #[schemars(
        rename = "scriptsig",
        with = "String",
        example = "04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73"
    )]
    pub script_sig: ScriptBuf,

    /// Signature script in assembly format
    #[allow(dead_code)]
    #[schemars(
        rename = "scriptsig_asm",
        with = "String",
        example = "OP_PUSHBYTES_4 ffff001d OP_PUSHBYTES_1 04 OP_PUSHBYTES_69 5468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73"
    )]
    script_sig_asm: (),

    // /// Witness data (for SegWit inputs)
    // #[schemars(example = vec!["3045022100d0c9936990bf00bdba15f425f0f360a223d5cbf81f4bf8477fe6c6d838fb5fae02207e42a8325a4dd41702bf065aa6e0a1b7b0b8ee92a5e6c182da018b0afc82c40601".to_string()])]
    // pub witness: Vec<String>,
    //
    /// Whether this input is a coinbase (block reward) input
    #[schemars(example = false)]
    pub is_coinbase: bool,

    /// Input sequence number
    #[schemars(example = 429496729)]
    pub sequence: u32,

    /// Inner redeemscript in assembly format (for P2SH-wrapped SegWit)
    #[allow(dead_code)]
    #[schemars(
        rename = "inner_redeemscript_asm",
        with = "Option<String>",
        example = Some("OP_0 OP_PUSHBYTES_20 992a1f7420fc5285070d19c71ff2efb1e356ad2f".to_string())
    )]
    inner_redeem_script_asm: (),
}

impl TxIn {
    pub fn script_sig_asm(&self) -> String {
        self.script_sig.to_asm_string()
    }

    pub fn redeem_script(&self) -> Option<&Script> {
        self.script_sig.redeem_script()
    }
}

impl From<(bitcoin::TxIn, &Client)> for TxIn {
    fn from((txin, rpc): (bitcoin::TxIn, &Client)) -> Self {
        let txout_result = rpc
            .get_tx_out(
                &txin.previous_output.txid,
                txin.previous_output.vout,
                Some(true),
            )
            .unwrap();

        let is_coinbase = txout_result.as_ref().is_none_or(|r| r.coinbase);

        // txin.witness

        // txin.script_sig.redeem_script()

        Self {
            is_coinbase,
            prevout: txout_result.map(TxOut::from),
            txid: txin.previous_output.txid.into(),
            vout: txin.previous_output.vout.into(),
            script_sig: txin.script_sig,
            script_sig_asm: (),
            sequence: txin.sequence.into(),
            // witness:
            inner_redeem_script_asm: (),
        }
    }
}

impl Serialize for TxIn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TxIn", 8)?;

        state.serialize_field("txid", &self.txid)?;
        state.serialize_field("vout", &self.vout)?;
        state.serialize_field("prevout", &self.prevout)?;
        state.serialize_field("scriptsig", &self.script_sig.to_hex_string())?;
        state.serialize_field("scriptsig_asm", &self.script_sig_asm())?;
        state.serialize_field("is_coinbase", &self.is_coinbase)?;
        state.serialize_field("sequence", &self.sequence)?;
        state.serialize_field("inner_redeemscript_asm", &self.redeem_script())?;

        state.end()
    }
}
