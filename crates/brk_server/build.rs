use std::{env, path::Path};

fn main() {
    let is_dev = env::var("PROFILE").is_ok_and(|p| p == "debug");

    // Generate importmap for website (updates index.html in place)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Use ./website (symlink in repo, real dir in published crate)
    let website_path = Path::new(&manifest_dir).join("website");

    println!("cargo:rerun-if-changed=website");
    println!("cargo::warning=build.rs: website_path={website_path:?}, exists={}", website_path.exists());

    if website_path.exists() {
        // Skip importmap hashing in dev mode (files change often)
        let map = if is_dev {
            println!("cargo::warning=build.rs: dev mode, skipping importmap");
            importmap::ImportMap::empty()
        } else {
            match importmap::ImportMap::scan(&website_path, "") {
                Ok(map) => {
                    println!("cargo::warning=build.rs: importmap scanned {} entries", map.imports.len());
                    map
                }
                Err(e) => {
                    println!("cargo::warning=build.rs: importmap scan failed: {e}");
                    importmap::ImportMap::empty()
                }
            }
        };

        let index_path = website_path.join("index.html");
        if let Err(e) = map.update_html_file(&index_path) {
            println!("cargo::warning=build.rs: failed to update index.html: {e}");
        }
    } else {
        println!("cargo::warning=build.rs: website path does not exist!");
    }
}
