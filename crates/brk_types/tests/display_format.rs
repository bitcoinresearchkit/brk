#![cfg(feature = "storage")]

use std::fmt::Display;

use bitcoin::absolute::LockTime;
use brk_types::{
    Date, EmptyAddrData, FundedAddrData, OutPoint, OutputType, P2ABytes, P2PK33Bytes, P2PK65Bytes,
    P2PKHBytes, P2SHBytes, P2TRBytes, P2WPKHBytes, P2WSHBytes, PoolSlug, RawLockTime, Sats,
    SupplyState, TxIndex, Vout,
};
use vecdb::Formattable;

fn check(value: &(impl Display + Formattable)) {
    let expected = value.to_string();
    let mut output = b"prefix:".to_vec();
    value.write_to(&mut output);
    assert_eq!(output, format!("prefix:{expected}").into_bytes());
    let mut text = String::from("prefix:");
    value.fmt_into(&mut text);
    assert_eq!(text, format!("prefix:{expected}"));
    output.clear();
    value.fmt_json(&mut output);
    assert_eq!(output, format!("\"{expected}\"").into_bytes());
    let mut csv = String::from("prefix:");
    value.fmt_csv(&mut csv).unwrap();
    let expected_csv = if expected.contains(',') {
        format!("prefix:\"{expected}\"")
    } else {
        format!("prefix:{expected}")
    };
    assert_eq!(csv, expected_csv);
}

#[test]
fn display_backed_formatters_keep_exact_bytes() {
    for seed in [0, 1, 127, 255] {
        check(&P2ABytes::from(&[seed; 2][..]));
        check(&P2PK33Bytes::from(&[seed; 33][..]));
        check(&P2PK65Bytes::from(&[seed; 65][..]));
        check(&P2PKHBytes::from(&[seed; 20][..]));
        check(&P2SHBytes::from(&[seed; 20][..]));
        check(&P2TRBytes::from(&[seed; 32][..]));
        check(&P2WPKHBytes::from(&[seed; 20][..]));
        check(&P2WSHBytes::from(&[seed; 32][..]));
    }
    check(&Date::new(2024, 2, 29));
    check(&EmptyAddrData::default());
    check(&FundedAddrData::default());
    check(&SupplyState {
        utxo_count: 17,
        value: Sats::new(123_456_789),
    });
    check(&OutPoint::new(TxIndex::new(123), Vout::from(7u32)));
    check(&OutPoint::COINBASE);
    check(&PoolSlug::Unknown);
    check(&PoolSlug::OneThash);
    for output_type in [
        OutputType::P2PK65,
        OutputType::P2PK33,
        OutputType::P2PKH,
        OutputType::P2MS,
        OutputType::P2SH,
        OutputType::OpReturn,
        OutputType::P2WPKH,
        OutputType::P2WSH,
        OutputType::P2TR,
        OutputType::P2A,
        OutputType::Empty,
        OutputType::Unknown,
    ] {
        check(&output_type);
    }
}

#[test]
fn lock_time_conversion_and_display_match_bitcoin_at_boundaries() {
    for n in [0, 1, 499_999_999, 500_000_000, 500_000_001, u32::MAX] {
        let original = LockTime::from_consensus(n);
        let raw = RawLockTime::from(original);
        assert_eq!(LockTime::from(raw), original);
        assert_eq!(raw.to_string(), original.to_string());
        check(&raw);
    }
}
