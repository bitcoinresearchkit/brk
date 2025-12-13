use crate::{Address, AddressBytes, OutputType, Sats};
use bitcoin::ScriptBuf;
use schemars::JsonSchema;
use serde::{Serialize, Serializer, ser::SerializeStruct};

#[derive(Debug, Clone, JsonSchema)]
/// Transaction output
pub struct TxOut {
    /// Script pubkey (locking script)
    #[serde(
        rename = "scriptpubkey",
        serialize_with = "serialize_with_script_pubkey"
    )]
    #[schemars(
        with = "String",
        example = "00143b064c595a95f977f00352d6e917501267cacdc6"
    )]
    pub script_pubkey: ScriptBuf,

    /// Script pubkey in assembly format
    #[allow(dead_code)]
    #[serde(skip, rename = "scriptpubkey_asm")]
    #[schemars(
        with = "String",
        example = "OP_0 OP_PUSHBYTES_20 3b064c595a95f977f00352d6e917501267cacdc6"
    )]
    script_pubkey_asm: (),

    /// Script type (p2pk, p2pkh, p2sh, p2wpkh, p2wsh, p2tr, op_return, etc.)
    #[allow(dead_code)]
    #[serde(skip, rename = "scriptpubkey_type")]
    #[schemars(with = "OutputType", example = &"v0_p2wpkh")]
    script_pubkey_type: (),

    /// Bitcoin address (if applicable, None for OP_RETURN)
    #[allow(dead_code)]
    #[serde(skip, rename = "scriptpubkey_address")]
    #[schemars(with = "Option<Address>", example = Some("bc1q8vryck26jhuh0uqr2ttwj96szfnu4nwxfmu39y".to_string()))]
    script_pubkey_address: (),

    /// Value of the output in satoshis
    #[schemars(example = Sats::new(7782))]
    pub value: Sats,
}

impl TxOut {
    pub fn address(&self) -> Option<Address> {
        Address::try_from(&self.script_pubkey).ok()
    }

    pub fn address_bytes(&self) -> Option<AddressBytes> {
        AddressBytes::try_from(&self.script_pubkey).ok()
    }

    pub fn type_(&self) -> OutputType {
        OutputType::from(&self.script_pubkey)
    }

    pub fn script_pubkey_asm(&self) -> String {
        self.script_pubkey.to_asm_string()
    }
}

impl From<bitcoin::TxOut> for TxOut {
    #[inline]
    fn from(txout: bitcoin::TxOut) -> Self {
        Self {
            script_pubkey: txout.script_pubkey,
            value: txout.value.into(),
            script_pubkey_asm: (),
            script_pubkey_address: (),
            script_pubkey_type: (),
        }
    }
}

impl From<(ScriptBuf, Sats)> for TxOut {
    #[inline]
    fn from((script, value): (ScriptBuf, Sats)) -> Self {
        Self {
            script_pubkey: script,
            script_pubkey_address: (),
            script_pubkey_asm: (),
            script_pubkey_type: (),
            value,
        }
    }
}

impl Serialize for TxOut {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TxOut", 5)?;
        state.serialize_field("scriptpubkey", &self.script_pubkey.to_hex_string())?;
        state.serialize_field("scriptpubkey_asm", &self.script_pubkey_asm())?;
        state.serialize_field("scriptpubkey_type", &self.type_())?;
        state.serialize_field("scriptpubkey_address", &self.address())?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}
