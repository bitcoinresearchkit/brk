use super::*;
#[test]
fn reads_real_formatter_output_and_rejects_broken_access_lines() {
    let r = Record::parse("2026-09-04 00:06:25 - info  200 /api/urpd/all 36.613583ms")
        .unwrap()
        .unwrap();
    assert_eq!(r.nanos, 36_613_583);
    assert_eq!(duration("59.916µs"), Some(59_916));
    assert_eq!(duration("1.083ns"), Some(1));
    assert_eq!(duration("1.25s"), Some(1_250_000_000));
    assert_eq!(duration("NaNs"), None);
    assert_eq!(duration("-1ms"), None);
    assert!(
        Record::parse("2026-09-04 00:06:25 - info  Computed indexer in 30ms")
            .unwrap()
            .is_none()
    );
    assert!(Record::parse("2026-09-04 00:06:25 - info  200 /api truncated").is_err());
}
