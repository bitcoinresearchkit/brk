use bitcoin::hex::DisplayHex;
use schemars::JsonSchema;
use serde::Serialize;

use crate::{AddressBytes, OutputType};

/// Address validation result
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AddressValidation {
    /// Whether the address is valid
    pub isvalid: bool,

    /// The validated address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// The scriptPubKey in hex
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: Option<String>,

    /// Whether this is a script address (P2SH)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isscript: Option<bool>,

    /// Whether this is a witness address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iswitness: Option<bool>,

    /// Witness version (0 for P2WPKH/P2WSH, 1 for P2TR)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_version: Option<u8>,

    /// Witness program in hex
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_program: Option<String>,
}

impl AddressValidation {
    /// Returns an invalid validation result
    pub fn invalid() -> Self {
        Self {
            isvalid: false,
            address: None,
            script_pub_key: None,
            isscript: None,
            iswitness: None,
            witness_version: None,
            witness_program: None,
        }
    }

    /// Validate a Bitcoin address string and return details
    pub fn from_address(address: &str) -> Self {
        let Ok(script) = AddressBytes::address_to_script(address) else {
            return Self::invalid();
        };

        let output_type = OutputType::from(&script);
        let script_hex = script.as_bytes().to_lower_hex_string();

        let is_script = matches!(output_type, OutputType::P2SH);
        let is_witness = matches!(
            output_type,
            OutputType::P2WPKH | OutputType::P2WSH | OutputType::P2TR | OutputType::P2A
        );

        let (witness_version, witness_program) = if is_witness {
            let version = script.witness_version().map(|v| v.to_num());
            let program = if script.len() > 2 {
                Some(script.as_bytes()[2..].to_lower_hex_string())
            } else {
                None
            };
            (version, program)
        } else {
            (None, None)
        };

        Self {
            isvalid: true,
            address: Some(address.to_string()),
            script_pub_key: Some(script_hex),
            isscript: Some(is_script),
            iswitness: Some(is_witness),
            witness_version,
            witness_program,
        }
    }
}
