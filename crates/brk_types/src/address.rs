use std::fmt;

use bitcoin::{ScriptBuf, opcodes, script::Builder};
use brk_error::Error;
use derive_deref::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, Serializer};

use crate::AddressBytes;

use super::OutputType;

#[derive(Debug, Deref, Deserialize, JsonSchema)]
pub struct Address {
    /// Bitcoin address string
    #[schemars(example = &"04678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5f")]
    pub address: String,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.address)
    }
}

impl From<String> for Address {
    fn from(address: String) -> Self {
        Self { address }
    }
}

impl TryFrom<&ScriptBuf> for Address {
    type Error = Error;
    fn try_from(script: &ScriptBuf) -> Result<Self, Self::Error> {
        Self::try_from(&AddressBytes::try_from(script)?)
    }
}

impl TryFrom<(&ScriptBuf, OutputType)> for Address {
    type Error = Error;
    fn try_from(tuple: (&ScriptBuf, OutputType)) -> Result<Self, Self::Error> {
        Self::try_from(&AddressBytes::try_from(tuple)?)
    }
}

impl TryFrom<&AddressBytes> for Address {
    type Error = Error;
    fn try_from(bytes: &AddressBytes) -> Result<Self, Self::Error> {
        let address = match bytes {
            AddressBytes::P2PK65(_) => Self::from(bytes_to_hex(bytes.as_slice())),
            AddressBytes::P2PK33(_) => Self::from(bytes_to_hex(bytes.as_slice())),
            AddressBytes::P2PKH(b) => Self::try_from(
                &Builder::new()
                    .push_opcode(opcodes::all::OP_DUP)
                    .push_opcode(opcodes::all::OP_HASH160)
                    .push_slice(****b)
                    .push_opcode(opcodes::all::OP_EQUALVERIFY)
                    .push_opcode(opcodes::all::OP_CHECKSIG)
                    .into_script(),
            )?,
            AddressBytes::P2SH(b) => Self::try_from(
                &Builder::new()
                    .push_opcode(opcodes::all::OP_HASH160)
                    .push_slice(****b)
                    .push_opcode(opcodes::all::OP_EQUAL)
                    .into_script(),
            )?,
            AddressBytes::P2WPKH(b) => {
                Self::try_from(&Builder::new().push_int(0).push_slice(****b).into_script())?
            }
            AddressBytes::P2WSH(b) => {
                Self::try_from(&Builder::new().push_int(0).push_slice(****b).into_script())?
            }
            AddressBytes::P2TR(b) => {
                Self::try_from(&Builder::new().push_int(1).push_slice(****b).into_script())?
            }
            AddressBytes::P2A(b) => {
                Self::try_from(&Builder::new().push_int(1).push_slice(****b).into_script())?
            }
        };
        Ok(address)
    }
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex_string = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write;
        write!(&mut hex_string, "{:02x}", byte).unwrap();
    }
    hex_string
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.address)
    }
}
