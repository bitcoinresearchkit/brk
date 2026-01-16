use std::{env, path::Path};

fn main() {
    let is_dev = env::var("PROFILE").is_ok_and(|p| p == "debug");

    // Generate importmap for website (updates index.html in place)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let website_path = Path::new(&manifest_dir).join("../../website");

    println!("cargo:rerun-if-changed=../../website");

    if website_path.exists() {
        // Skip importmap hashing in dev mode (files change often)
        let map = if is_dev {
            importmap::ImportMap::empty()
        } else {
            importmap::ImportMap::scan(&website_path, "")
                .unwrap_or_else(|_| importmap::ImportMap::empty())
        };

        let _ = map.update_html_file(&website_path.join("index.html"));
    }
}
