use importmap::ImportMap;
use std::path::Path;

#[test]
fn transform_preserves_css_order_deduplicates_and_emits_sorted_modules() {
    let map = ImportMap::scan(
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures")),
        "",
    )
    .unwrap();
    let css = &map["/style.css"];
    let js = &map["/modules/app.js"];
    let html = "<head>\n  <!-- IMPORTMAP -->\n  <link rel='stylesheet' href='/style.css'>\n  <link rel=\"stylesheet\" href=\"/style.css\">\n  <link rel=\"stylesheet\" href=\"/missing.css\">\n  <!-- /IMPORTMAP -->\n</head>";
    let expected = format!(
        "<head>\n  <!-- IMPORTMAP -->\n  <link rel=\"stylesheet\" href=\"{css}\">\n  <script type=\"importmap\">\n  {{\n    \"imports\": {{\n      \"/modules/app.js\": \"{js}\"\n    }}\n  }}\n  </script>\n  <link rel=\"modulepreload\" href=\"{js}\">\n  <!-- /IMPORTMAP -->\n</head>"
    );
    assert_eq!(map.transform_html(html).unwrap(), expected);
}

#[test]
fn empty_and_malformed_markers_are_handled_without_panicking() {
    let map = ImportMap::scan(
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures")),
        "",
    )
    .unwrap();
    for html in [
        "",
        "<!-- IMPORTMAP -->",
        "<!-- /IMPORTMAP -->",
        "<!-- /IMPORTMAP --><!-- IMPORTMAP -->",
    ] {
        assert!(map.transform_html(html).is_none());
    }
    assert_eq!(
        ImportMap::empty()
            .transform_html("<!-- IMPORTMAP -->old<!-- /IMPORTMAP -->")
            .unwrap(),
        "<!-- IMPORTMAP -->\n\n<!-- /IMPORTMAP -->"
    );
}
