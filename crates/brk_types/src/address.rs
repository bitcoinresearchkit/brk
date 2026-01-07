use std::{fmt, str::FromStr};

use bitcoin::ScriptBuf;
use brk_error::Error;
use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, Serializer};

use crate::AddressBytes;

use super::OutputType;

/// Bitcoin address string
#[derive(Debug, Deref, Deserialize, JsonSchema)]
#[serde(transparent)]
#[schemars(
    example = &"04678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5f",
    example = &"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
    example = &"bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq"
)]
pub struct Address(String);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for Address {
    #[inline]
    fn from(address: String) -> Self {
        Self(address)
    }
}

impl TryFrom<&ScriptBuf> for Address {
    type Error = Error;
    fn try_from(script: &ScriptBuf) -> Result<Self, Self::Error> {
        Self::try_from((script, OutputType::from(script)))
    }
}

impl TryFrom<(&ScriptBuf, OutputType)> for Address {
    type Error = Error;
    fn try_from((script, outputtype): (&ScriptBuf, OutputType)) -> Result<Self, Self::Error> {
        if outputtype.is_address() {
            Ok(Self(script.to_hex_string()))
        } else {
            Err(Error::InvalidAddress)
        }
    }
}

impl TryFrom<&AddressBytes> for Address {
    type Error = Error;
    fn try_from(bytes: &AddressBytes) -> Result<Self, Self::Error> {
        // P2PK addresses are represented as raw pubkey hex, not as a script
        let address = match bytes {
            AddressBytes::P2PK65(_) | AddressBytes::P2PK33(_) => {
                Self::from(bytes_to_hex(bytes.as_slice()))
            }
            _ => Self::try_from(&bytes.to_script_pubkey())?,
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
        serializer.collect_str(&self.0)
    }
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let _ = AddressBytes::address_to_script(s)?;
        Ok(Self(s.to_string()))
    }
}

impl Address {
    /// Get the script for this address
    pub fn script(&self) -> Result<ScriptBuf, Error> {
        AddressBytes::address_to_script(&self.0)
    }
}
