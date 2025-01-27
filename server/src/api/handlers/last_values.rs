use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use serde_json::Value;

use crate::{io::Json, server::AppState};

pub async fn last_values_handler(State(app_state): State<AppState>) -> Response {
    let values = Json::import::<Value>(&app_state.config.path_datasets_last_values()).unwrap();
    let values = axum::Json(values);
    values.into_response()
}
