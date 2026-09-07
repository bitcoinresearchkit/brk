use bitcoin::hashes::Hash;
use brk_types::Txid;
#[cfg(feature = "storage")]
use vecdb::Formattable;

#[test]
fn txid_formatting_and_serialization_match_bitcoin() {
    for seed in 0..64u8 {
        let bytes = std::array::from_fn(|i| seed.wrapping_mul(37).wrapping_add(i as u8));
        let bitcoin = bitcoin::Txid::from_byte_array(bytes);
        let txid = Txid::from(bitcoin);
        let expected = bitcoin.to_string();
        assert_eq!(txid.to_string(), expected);
        assert_eq!(format!("{txid:>80}"), expected);
        let json = serde_json::to_string(&txid).unwrap();
        assert_eq!(json, serde_json::to_string(&bitcoin).unwrap());
        assert_eq!(serde_json::from_str::<Txid>(&json).unwrap(), txid);

        #[cfg(feature = "storage")]
        {
            let mut output = b"prefix:".to_vec();
            txid.write_to(&mut output);
            assert_eq!(output, format!("prefix:{expected}").into_bytes());
            output.clear();
            txid.fmt_json(&mut output);
            assert_eq!(output, json.as_bytes());
        }
    }
}
