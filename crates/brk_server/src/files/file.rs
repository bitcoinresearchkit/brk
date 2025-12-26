use std::{fs, path::Path, time::Duration};

use axum::{
    body::Body,
    extract::{self, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use brk_error::{Error, Result};
use log::{error, info};
use quick_cache::sync::GuardResult;

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

        // Canonicalize and verify the path stays within the allowed directory
        if let Ok(canonical) = path.canonicalize()
            && let Ok(canonical_base) = files_path.canonicalize()
            && !canonical.starts_with(&canonical_base)
        {
            let mut response: Response<Body> =
                (StatusCode::FORBIDDEN, "Access denied".to_string()).into_response();
            response.headers_mut().insert_cors();
            return response;
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
    if modified == ModifiedState::NotModifiedSince {
        return Ok(Response::new_not_modified());
    }

    let serialized_path = path.to_str().unwrap();

    let must_revalidate = path
        .extension()
        .is_some_and(|extension| extension == "html")
        || serialized_path.ends_with("service-worker.js");

    let guard_res = if !must_revalidate {
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

    if must_revalidate {
        headers.insert_cache_control_must_revalidate();
    } else if path.extension().is_some_and(|extension| {
        extension == "jpg"
            || extension == "png"
            || extension == "woff2"
            || extension == "js"
            || extension == "map"
    }) {
        headers.insert_cache_control_immutable();
    }

    headers.insert_last_modified(date);

    Ok(response)
}
