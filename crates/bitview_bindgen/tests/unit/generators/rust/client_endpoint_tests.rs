#[test]
fn generated_endpoint_matches_checked_in_client() {
    let mut emitted = String::new();
    super::generate_endpoint(&mut emitted);
    let emitted = super::super::format_rust(emitted).expect("endpoint formatting succeeds");
    let client = include_str!("../../../../../bitview_client/src/generated.rs");
    let compact = |text: &str| text.split_whitespace().collect::<String>();
    assert!(compact(client).contains(&compact(&emitted)));
}
