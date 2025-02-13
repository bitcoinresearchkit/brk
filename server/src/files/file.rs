use std::{
    fs::{self},
    path::{Path, PathBuf},
    time::Instant,
};

use axum::{
    body::Body,
    extract,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use logger::{error, info};
use reqwest::StatusCode;

use crate::{
    log_result,
    traits::{HeaderMapExtended, ModifiedState, ResponseExtended},
};

use super::minify::minify_js;

const WEBSITE_DEV_PATH: &str = "../website/";

pub async fn file_handler(headers: HeaderMap, path: extract::Path<String>) -> Response {
    any_handler(headers, Some(path))
}

pub async fn index_handler(headers: HeaderMap) -> Response {
    any_handler(headers, None)
}

fn any_handler(headers: HeaderMap, path: Option<extract::Path<String>>) -> Response {
    let instant = Instant::now();

    let response = if let Some(path) = path.as_ref() {
        let path = path.0.replace("..", "").replace("\\", "");

        let mut path = str_to_path(&path);

        if !path.exists() {
            if path.extension().is_some() {
                let mut response: Response<Body> =
                    (StatusCode::INTERNAL_SERVER_ERROR, "File doesn't exist".to_string()).into_response();

                response.headers_mut().insert_cors();

                return response;
            } else {
                path = str_to_path("index.html");
            }
        }

        path_to_response(&headers, &path)
    } else {
        path_to_response(&headers, &str_to_path("index.html"))
    };

    log_result(
        response.status(),
        &format!("/{}", path.map_or("".to_owned(), |p| p.0)),
        instant,
    );

    response
}

fn path_to_response(headers: &HeaderMap, path: &Path) -> Response {
    match path_to_response_(headers, path) {
        Ok(response) => response,
        Err(error) => {
            let mut response = (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response();

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

    let mut response;

    let is_localhost = headers.check_if_host_is_localhost();

    if !is_localhost
        && path.extension().unwrap_or_else(|| {
            dbg!(path);
            panic!();
        }) == "js"
    {
        let content = minify_js(path);

        response = Response::new(content.into());
    } else {
        let content = fs::read(path).unwrap_or_else(|error| {
            error!("{error}");
            let path = path.to_str().unwrap();
            info!("Can't read file {path}");
            panic!("")
        });

        response = Response::new(content.into());
    }

    let headers = response.headers_mut();
    headers.insert_cors();
    headers.insert_content_type(path);

    if !is_localhost {
        let serialized_path = path.to_str().unwrap();

        if serialized_path.contains("fonts/")
            || serialized_path.contains("assets/")
            || serialized_path.contains("packages/")
            || path.extension().is_some_and(|extension| {
                extension == "pdf" || extension == "jpg" || extension == "png" || extension == "woff2"
            })
        {
            headers.insert_cache_control_immutable();
        }
    }

    headers.insert_last_modified(date);

    Ok(response)
}

fn str_to_path(path: &str) -> PathBuf {
    PathBuf::from(&format!("{WEBSITE_DEV_PATH}{path}"))
}
