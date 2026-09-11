use super::generate_base_client;

#[test]
fn checked_in_base_client_matches_generator() {
    let mut output = String::new();
    generate_base_client(&mut output);
    let client = include_str!("../../../../../../modules/bitview-client/index.js");
    assert!(
        client.contains(&output),
        "regenerate the JavaScript base client"
    );
}
