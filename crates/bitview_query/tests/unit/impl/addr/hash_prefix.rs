use super::*;

#[test]
fn prefixes_are_only_hexadecimal_nibbles() {
    for prefix in ["", "+f", "-1", "0x1", "fffffffffffffffff", "é"] {
        assert!(AddrHashPrefix::parse(prefix).is_err(), "{prefix}");
    }
    for prefix in ["0", "f", "Ab", "FFFFFFFFFFFFFFFF"] {
        let parsed = AddrHashPrefix::parse(prefix).unwrap();
        assert_eq!(parsed.text, prefix.to_ascii_lowercase());
    }
    assert!(
        AddrHashPrefix::parse("ffffffffffffffff")
            .unwrap()
            .upper
            .is_none()
    );
}
