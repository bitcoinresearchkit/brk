use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use axum::{
    body::Body,
    extract::{self, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use brk_error::Result;
use quick_cache::sync::GuardResult;
use tracing::{error, info};

use crate::{
    AppState, EMBEDDED_WEBSITE, HeaderMapExtended, ModifiedState, ResponseExtended, WebsiteSource,
};

pub async fn file_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    path: extract::Path<String>,
) -> Response {
    any_handler(headers, state, Some(path.0))
}

pub async fn index_handler(headers: HeaderMap, State(state): State<AppState>) -> Response {
    any_handler(headers, state, None)
}

fn any_handler(headers: HeaderMap, state: AppState, path: Option<String>) -> Response {
    match &state.website {
        WebsiteSource::Disabled => unreachable!("routes not added when disabled"),
        WebsiteSource::Embedded => embedded_handler(&state, path),
        WebsiteSource::Filesystem(files_path) => {
            filesystem_handler(headers, &state, files_path, path)
        }
    }
}

/// Sanitize path to prevent traversal attacks
fn sanitize_path(path: &str) -> String {
    path.split('/')
        .filter(|c| !c.is_empty() && *c != "." && *c != "..")
        .collect::<Vec<_>>()
        .join("/")
}

/// Check if path requires revalidation (HTML files, service worker)
fn must_revalidate(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "html")
        || path
            .to_str()
            .is_some_and(|p| p.ends_with("service-worker.js"))
}

/// Build response with proper headers and caching
fn build_response(state: &AppState, path: &Path, content: Vec<u8>, cache_key: &str) -> Response {
    let must_revalidate = must_revalidate(path);

    // Use cache for non-HTML files in release mode
    let guard_res = if !cfg!(debug_assertions) && !must_revalidate {
        Some(
            state
                .cache
                .get_value_or_guard(&cache_key.to_owned(), Some(Duration::from_millis(50))),
        )
    } else {
        None
    };

    let mut response = if let Some(GuardResult::Value(v)) = guard_res {
        Response::new(Body::from(v))
    } else {
        if let Some(GuardResult::Guard(g)) = guard_res {
            let _ = g.insert(content.clone().into());
        }
        Response::new(Body::from(content))
    };

    let headers = response.headers_mut();
    headers.insert_cors();
    headers.insert_content_type(path);

    if cfg!(debug_assertions) || must_revalidate {
        headers.insert_cache_control_must_revalidate();
    } else {
        headers.insert_cache_control_immutable();
    }

    response
}

fn embedded_handler(state: &AppState, path: Option<String>) -> Response {
    let path = path.unwrap_or_else(|| "index.html".to_string());
    let sanitized = sanitize_path(&path);

    // Try to get file, with importmap hash stripping and SPA fallback
    let file = EMBEDDED_WEBSITE
        .get_file(&sanitized)
        .or_else(|| {
            strip_importmap_hash(Path::new(&sanitized))
                .and_then(|unhashed| EMBEDDED_WEBSITE.get_file(unhashed.to_str()?))
        })
        .or_else(|| {
            // If no extension, serve index.html (SPA routing)
            if Path::new(&sanitized).extension().is_none() {
                EMBEDDED_WEBSITE.get_file("index.html")
            } else {
                None
            }
        });

    let Some(file) = file else {
        let mut response: Response<Body> =
            (StatusCode::NOT_FOUND, "File not found".to_string()).into_response();
        response.headers_mut().insert_cors();
        return response;
    };

    build_response(
        state,
        Path::new(file.path()),
        file.contents().to_vec(),
        &file.path().to_string_lossy(),
    )
}

fn filesystem_handler(
    headers: HeaderMap,
    state: &AppState,
    files_path: &Path,
    path: Option<String>,
) -> Response {
    let path = if let Some(path) = path {
        let sanitized = sanitize_path(&path);
        let mut path = files_path.join(&sanitized);

        // Canonicalize and verify the path stays within the project root
        // (allows symlinks to modules/ which is outside the website directory)
        if let Ok(canonical) = path.canonicalize()
            && let Ok(canonical_base) = files_path.canonicalize()
        {
            let project_root = canonical_base.parent().and_then(|p| p.parent());
            let allowed = canonical.starts_with(&canonical_base)
                || project_root.is_some_and(|root| canonical.starts_with(root));
            if !allowed {
                let mut response: Response<Body> =
                    (StatusCode::FORBIDDEN, "Access denied".to_string()).into_response();
                response.headers_mut().insert_cors();
                return response;
            }
        }

        // Strip hash from import-mapped URLs
        if !path.exists()
            && let Some(unhashed) = strip_importmap_hash(&path)
            && unhashed.exists()
        {
            path = unhashed;
        }

        // SPA fallback
        if !path.exists() || path.is_dir() {
            if path.extension().is_some() {
                let mut response: Response<Body> = (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "File doesn't exist".to_string(),
                )
                    .into_response();
                response.headers_mut().insert_cors();
                return response;
            } else {
                path = files_path.join("index.html");
            }
        }

        path
    } else {
        files_path.join("index.html")
    };

    path_to_response(&headers, state, &path)
}

fn path_to_response(headers: &HeaderMap, state: &AppState, path: &Path) -> Response {
    match path_to_response_(headers, state, path) {
        Ok(response) => response,
        Err(error) => {
            let mut response =
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response();
            response.headers_mut().insert_cors();
            response
        }
    }
}

fn path_to_response_(headers: &HeaderMap, state: &AppState, path: &Path) -> Result<Response> {
    let (modified, date) = headers.check_if_modified_since(path)?;
    if !cfg!(debug_assertions) && modified == ModifiedState::NotModifiedSince {
        return Ok(Response::new_not_modified());
    }

    let content = fs::read(path).unwrap_or_else(|error| {
        error!("{error}");
        let path = path.to_str().unwrap();
        info!("Can't read file {path}");
        panic!("")
    });

    let cache_key = path.to_str().unwrap();
    let mut response = build_response(state, path, content, cache_key);
    response.headers_mut().insert_last_modified(date);

    Ok(response)
}

/// Strip importmap hash from filename: `foo.abc12345.js` -> `foo.js`
/// Hash is 8 hex characters between the name and extension.
fn strip_importmap_hash(path: &Path) -> Option<PathBuf> {
    let stem = path.file_stem()?.to_str()?;
    let ext = path.extension()?.to_str()?;

    // Only process js/mjs/css files
    if !matches!(ext, "js" | "mjs" | "css") {
        return None;
    }

    // Look for pattern: name.HASH where HASH is 8 hex chars
    let dot_pos = stem.rfind('.')?;
    let hash = &stem[dot_pos + 1..];

    if hash.len() == 8 && hash.chars().all(|c| c.is_ascii_hexdigit()) {
        let name = &stem[..dot_pos];
        let new_name = format!("{}.{}", name, ext);
        Some(path.with_file_name(new_name))
    } else {
        None
    }
}
