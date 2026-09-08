use super::*;

#[test]
fn attachment_names_cannot_change_header_syntax() {
    for (name, expected) in [
        (
            "price_close-Day1.csv",
            "attachment; filename=\"price_close-Day1.csv\"",
        ),
        ("a b.csv", "attachment; filename=\"a b.csv\""),
        ("a\"b", "attachment; filename*=UTF-8''%61%22%62"),
        ("a\\b", "attachment; filename*=UTF-8''%61%5C%62"),
        ("a\r\n\0", "attachment; filename*=UTF-8''%61%0D%0A%00"),
        ("é", "attachment; filename*=UTF-8''%C3%A9"),
    ] {
        let mut headers = HeaderMap::new();
        headers.insert_content_disposition_attachment(name);
        assert_eq!(headers[header::CONTENT_DISPOSITION], expected);
        assert_eq!(headers.len(), 1);
    }
}
