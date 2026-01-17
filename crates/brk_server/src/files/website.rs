use std::{
    fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use importmap::ImportMap;
use tracing::{error, info};

use crate::{EMBEDDED_WEBSITE, Error, Result};

/// Cached index.html with importmap injected
static INDEX_HTML: OnceLock<String> = OnceLock::new();

/// Source for serving the website
#[derive(Debug, Clone)]
pub enum Website {
    Disabled,
    Default,
    Filesystem(PathBuf),
}

impl Website {
    pub fn is_enabled(&self) -> bool {
        !matches!(self, Self::Disabled)
    }

    /// Returns the filesystem path if available, None means use embedded
    pub fn filesystem_path(&self) -> Option<PathBuf> {
        match self {
            Self::Disabled => None,
            Self::Default => {
                if cfg!(debug_assertions) {
                    let local = PathBuf::from("./website");
                    local.exists().then_some(local)
                } else {
                    None
                }
            }
            Self::Filesystem(p) => Some(p.clone()),
        }
    }

    /// Get file content by path (handles hash-stripping, SPA fallback, importmap)
    pub fn get_file(&self, path: &str) -> Result<Vec<u8>> {
        match self.filesystem_path() {
            None => self.get_embedded(path),
            Some(base) => self.get_filesystem(&base, path),
        }
    }

    /// Log which website source is being used (call once at startup)
    pub fn log(&self) {
        match self {
            Self::Disabled => info!("Website: disabled"),
            Self::Default => {
                if let Some(p) = self.filesystem_path() {
                    info!("Website: filesystem ({})", p.display());
                } else {
                    info!("Website: embedded");
                }
            }
            Self::Filesystem(p) => info!("Website: filesystem ({})", p.display()),
        }
    }

    fn get_index(&self) -> Result<Vec<u8>> {
        // Debug mode: no importmap, no cache
        if cfg!(debug_assertions) {
            return match self.filesystem_path() {
                Some(base) => {
                    fs::read(base.join("index.html")).map_err(|e| Error::not_found(e.to_string()))
                }
                None => {
                    let file = EMBEDDED_WEBSITE
                        .get_file("index.html")
                        .expect("index.html must exist in embedded website");
                    Ok(file.contents().to_vec())
                }
            };
        }

        // Release mode: cache with importmap
        let html = INDEX_HTML.get_or_init(|| match self.filesystem_path() {
            None => {
                let file = EMBEDDED_WEBSITE
                    .get_file("index.html")
                    .expect("index.html must exist in embedded website");

                let html =
                    std::str::from_utf8(file.contents()).expect("index.html must be valid UTF-8");

                let importmap = ImportMap::scan_embedded(&EMBEDDED_WEBSITE, "");
                importmap
                    .transform_html(html)
                    .unwrap_or_else(|| html.to_string())
            }
            Some(base) => {
                let html =
                    fs::read_to_string(base.join("index.html")).expect("index.html must exist");

                match ImportMap::scan(&base, "") {
                    Ok(importmap) => importmap.transform_html(&html).unwrap_or(html),
                    Err(e) => {
                        error!("Failed to scan for importmap: {e}");
                        html
                    }
                }
            }
        });

        Ok(html.as_bytes().to_vec())
    }

    fn get_embedded(&self, path: &str) -> Result<Vec<u8>> {
        // Index.html
        if path.is_empty() || path == "index.html" {
            return self.get_index();
        }

        // Try direct lookup, then with hash stripped
        let file = EMBEDDED_WEBSITE.get_file(path).or_else(|| {
            ImportMap::strip_hash(Path::new(path))
                .and_then(|unhashed| EMBEDDED_WEBSITE.get_file(unhashed.to_str()?))
        });

        if let Some(file) = file {
            return Ok(file.contents().to_vec());
        }

        // SPA fallback: no extension -> index.html
        if Path::new(path).extension().is_none() {
            return self.get_index();
        }

        Err(Error::not_found("File not found"))
    }

    fn get_filesystem(&self, base: &Path, path: &str) -> Result<Vec<u8>> {
        // Index.html
        if path.is_empty() {
            return self.get_index();
        }

        let mut file_path = base.join(path);

        // Try with hash stripped
        if !file_path.exists()
            && let Some(unhashed) = ImportMap::strip_hash(&file_path)
            && unhashed.exists()
        {
            file_path = unhashed;
        }

        // SPA fallback or missing file
        if !file_path.exists() || file_path.is_dir() {
            if file_path.extension().is_some() {
                return Err(Error::not_found("File not found"));
            }
            return self.get_index();
        }

        // Explicit index.html request
        if file_path.file_name().is_some_and(|n| n == "index.html") {
            return self.get_index();
        }

        fs::read(&file_path).map_err(|e| {
            error!("{e}");
            Error::not_found("File not found")
        })
    }
}
