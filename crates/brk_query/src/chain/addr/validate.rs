use bitcoin::hex::DisplayHex;
use brk_types::{AddressBytes, AddressValidation, OutputType};

/// Validate a Bitcoin address and return details
pub fn validate_address(address: &str) -> AddressValidation {
    let Ok(script) = AddressBytes::address_to_script(address) else {
        return AddressValidation::invalid();
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
        // Witness program is after the version byte and push opcode
        let program = if script.len() > 2 {
            Some(script.as_bytes()[2..].to_lower_hex_string())
        } else {
            None
        };
        (version, program)
    } else {
        (None, None)
    };

    AddressValidation {
        isvalid: true,
        address: Some(address.to_string()),
        script_pub_key: Some(script_hex),
        isscript: Some(is_script),
        iswitness: Some(is_witness),
        witness_version,
        witness_program,
    }
}
