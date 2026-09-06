use vecdb::Formattable;

#[test]
fn length_format_matches_json() {
    for length in [0, 1, 9, 10, 1000, usize::MAX] {
        let mut bytes = Vec::new();
        length.fmt_json(&mut bytes);
        assert_eq!(bytes, serde_json::to_vec(&length).unwrap());
    }
}
