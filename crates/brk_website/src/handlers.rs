use std::path::Path;

use axum::{body::Body, extract::State, http::Response};

use crate::{HeaderMapExtended, Result, Website};

pub async fn file_handler(
    State(website): State<Website>,
    path: axum::extract::Path<String>,
) -> Result<Response<Body>> {
    serve(&website, &path.0)
}

pub async fn index_handler(State(website): State<Website>) -> Result<Response<Body>> {
    serve(&website, "")
}

fn serve(website: &Website, path: &str) -> Result<Response<Body>> {
    let path = sanitize(path);
    let content = website.get_file(&path)?;

    let mut response = Response::new(Body::from(content));
    let headers = response.headers_mut();

    // Empty path or no extension = index.html (SPA fallback)
    let is_html = path.is_empty()
        || Path::new(&path).extension().is_none()
        || path.ends_with(".html");

    if is_html {
        headers.insert_content_type_text_html();
    } else {
        headers.insert_content_type(Path::new(&path));
    }

    if cfg!(debug_assertions) || is_html {
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
