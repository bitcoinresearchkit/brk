use super::validate_hex;

#[test]
fn invalid_hex_is_rejected_without_rpc_work() {
    for body in ["", " \n", "0", "xyz", "0g", "00 11", "éé"] {
        assert!(validate_hex(body).is_err(), "{body:?}");
    }
    assert!(matches!(validate_hex(" \nAA00ff\t"), Ok("AA00ff")));
}
