use std::path::Path;

use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, Response, StatusCode},
};

use crate::{HeaderMapExtended, Result, Website};

pub async fn file_handler(
    State(website): State<Website>,
    headers: HeaderMap,
    path: axum::extract::Path<String>,
) -> Result<Response<Body>> {
    serve(&website, &path.0, &headers)
}

pub async fn index_handler(
    State(website): State<Website>,
    headers: HeaderMap,
) -> Result<Response<Body>> {
    serve(&website, "", &headers)
}

fn serve(website: &Website, path: &str, request_headers: &HeaderMap) -> Result<Response<Body>> {
    let path = sanitize(path);

    let is_html = path.is_empty()
        || Path::new(&path).extension().is_none()
        || path.ends_with(".html");

    // Etag 304 check (release mode, HTML only)
    if is_html {
        if let Some(etag) = website.index_etag() {
            if request_headers.has_etag(etag) {
                let mut response = Response::builder()
                    .status(StatusCode::NOT_MODIFIED)
                    .body(Body::empty())
                    .unwrap();
                let headers = response.headers_mut();
                headers.insert_etag(etag);
                headers.insert_cache_control_must_revalidate();
                return Ok(response);
            }
        }
    }

    let content = website.get_file(&path)?;
    let mut response = Response::new(Body::from(content));
    let headers = response.headers_mut();

    if is_html {
        headers.insert_content_type_text_html();
        if let Some(etag) = website.index_etag() {
            headers.insert_etag(etag);
        }
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
