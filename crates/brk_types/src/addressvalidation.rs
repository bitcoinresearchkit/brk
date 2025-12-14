use schemars::JsonSchema;
use serde::Serialize;

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
}
