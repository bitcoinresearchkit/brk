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
use brk_error::{Error, Result};
use quick_cache::sync::GuardResult;
use tracing::{error, info};

use crate::{AppState, HeaderMapExtended, ModifiedState, ResponseExtended};

pub async fn file_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    path: extract::Path<String>,
) -> Response {
    any_handler(headers, state, Some(path))
}

pub async fn index_handler(headers: HeaderMap, State(state): State<AppState>) -> Response {
    any_handler(headers, state, None)
}

fn any_handler(
    headers: HeaderMap,
    state: AppState,
    path: Option<extract::Path<String>>,
) -> Response {
    let files_path = state.path.as_ref().unwrap();

    if let Some(path) = path.as_ref() {
        // Sanitize path components to prevent traversal attacks
        let sanitized: String = path
            .0
            .split('/')
            .filter(|component| !component.is_empty() && *component != "." && *component != "..")
            .collect::<Vec<_>>()
            .join("/");

        let mut path = files_path.join(&sanitized);

        // Canonicalize and verify the path stays within the project root
        // (allows symlinks to modules/ which is outside the website directory)
        if let Ok(canonical) = path.canonicalize()
            && let Ok(canonical_base) = files_path.canonicalize()
        {
            // Allow paths within files_path OR within project root (2 levels up)
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

        // Strip hash from import-mapped URLs (e.g., foo.abc12345.js -> foo.js)
        if !path.exists()
            && let Some(unhashed) = strip_importmap_hash(&path)
            && unhashed.exists()
        {
            path = unhashed;
        }

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

        path_to_response(&headers, &state, &path)
    } else {
        path_to_response(&headers, &state, &files_path.join("index.html"))
    }
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

    let serialized_path = path.to_str().unwrap();

    let must_revalidate = path
        .extension()
        .is_some_and(|extension| extension == "html")
        || serialized_path.ends_with("service-worker.js");

    let guard_res = if !cfg!(debug_assertions) && !must_revalidate {
        Some(state.cache.get_value_or_guard(
            &path.to_str().unwrap().to_owned(),
            Some(Duration::from_millis(50)),
        ))
    } else {
        None
    };

    let mut response = if let Some(GuardResult::Value(v)) = guard_res {
        Response::new(Body::from(v))
    } else {
        let content = fs::read(path).unwrap_or_else(|error| {
            error!("{error}");
            let path = path.to_str().unwrap();
            info!("Can't read file {path}");
            panic!("")
        });

        if let Some(GuardResult::Guard(g)) = guard_res {
            g.insert(content.clone().into())
                .map_err(|_| Error::QuickCacheError)?;
        }

        Response::new(content.into())
    };

    let headers = response.headers_mut();
    headers.insert_cors();
    headers.insert_content_type(path);

    if cfg!(debug_assertions) || must_revalidate {
        headers.insert_cache_control_must_revalidate();
    } else {
        headers.insert_cache_control_immutable();
    }

    headers.insert_last_modified(date);

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
