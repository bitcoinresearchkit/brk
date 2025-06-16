use std::{fs, path::Path};

use axum::{
    body::Body,
    extract::{self, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use log::{error, info};

use crate::{
    AppState,
    traits::{HeaderMapExtended, ModifiedState, ResponseExtended},
};

pub async fn file_handler(
    headers: HeaderMap,
    State(app_state): State<AppState>,
    path: extract::Path<String>,
) -> Response {
    any_handler(headers, app_state, Some(path))
}

pub async fn index_handler(headers: HeaderMap, State(app_state): State<AppState>) -> Response {
    any_handler(headers, app_state, None)
}

fn any_handler(
    headers: HeaderMap,
    app_state: AppState,
    path: Option<extract::Path<String>>,
) -> Response {
    let dist_path = app_state.dist_path();

    if let Some(path) = path.as_ref() {
        let path = path.0.replace("..", "").replace("\\", "");

        let mut path = dist_path.join(&path);

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
                path = dist_path.join("index.html");
            }
        }

        path_to_response(&headers, &path)
    } else {
        path_to_response(&headers, &dist_path.join("index.html"))
    }
}

fn path_to_response(headers: &HeaderMap, path: &Path) -> Response {
    match path_to_response_(headers, path) {
        Ok(response) => response,
        Err(error) => {
            let mut response =
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response();

            response.headers_mut().insert_cors();

            response
        }
    }
}

fn path_to_response_(headers: &HeaderMap, path: &Path) -> color_eyre::Result<Response> {
    let (modified, date) = headers.check_if_modified_since(path)?;
    if modified == ModifiedState::NotModifiedSince {
        return Ok(Response::new_not_modified());
    }

    let content = fs::read(path).unwrap_or_else(|error| {
        error!("{error}");
        let path = path.to_str().unwrap();
        info!("Can't read file {path}");
        panic!("")
    });

    let mut response = Response::new(content.into());

    let headers = response.headers_mut();
    headers.insert_cors();
    headers.insert_content_type(path);

    let serialized_path = path.to_str().unwrap();

    if path
        .extension()
        .is_some_and(|extension| extension == "html")
        || serialized_path.ends_with("service-worker.js")
    {
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
