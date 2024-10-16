use axum::{extract::State, http::HeaderMap, response::Response};
use reqwest::header::HOST;

use crate::AppState;

use super::response::{generic_to_reponse, update_reponse_headers};

pub async fn fallback(headers: HeaderMap, State(app_state): State<AppState>) -> Response {
    update_reponse_headers(
        generic_to_reponse(
            app_state
                .routes
                .to_full_paths(headers[HOST].to_str().unwrap().to_string()),
            None,
        ),
        60,
        None,
    )
}
