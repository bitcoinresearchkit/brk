use std::path::Path;

use axum::{body::Body, extract::State, response::Response};

use crate::{AppState, HeaderMapExtended, Result};

pub async fn file_handler(
    State(state): State<AppState>,
    path: axum::extract::Path<String>,
) -> Result<Response> {
    serve(&state, &path.0)
}

pub async fn index_handler(State(state): State<AppState>) -> Result<Response> {
    serve(&state, "")
}

fn serve(state: &AppState, path: &str) -> Result<Response> {
    let path = sanitize(path);
    let content = state.website.get_file(&path)?;

    let mut response = Response::new(Body::from(content));
    let headers = response.headers_mut();

    // Empty path or no extension = index.html (SPA fallback)
    if path.is_empty() || Path::new(&path).extension().is_none() {
        headers.insert_content_type_text_html();
    } else {
        headers.insert_content_type(Path::new(&path));
    }

    if cfg!(debug_assertions) {
        headers.insert_cache_control_must_revalidate();
    } else {
        headers.insert_cache_control_immutable();
    }

    Ok(response)
}

/// Sanitize path to prevent directory traversal attacks
fn sanitize(path: &str) -> String {
    path.split('/')
        .filter(|s| !s.is_empty() && *s != "." && *s != "..")
        .collect::<Vec<_>>()
        .join("/")
}
