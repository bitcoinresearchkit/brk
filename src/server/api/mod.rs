use axum::{routing::get, Router};
use handlers::{dataset_handler, fallback};

use super::AppState;

mod handlers;
pub mod structs;

pub const API_URL_PREFIX: &str = "/api";

pub trait ApiRoutes {
    fn add_api_routes(self) -> Self;
}

impl ApiRoutes for Router<AppState> {
    fn add_api_routes(self) -> Self {
        self.route(&format!("{API_URL_PREFIX}/*path"), get(dataset_handler))
            .route(&format!("{API_URL_PREFIX}/"), get(fallback))
            .route(API_URL_PREFIX, get(fallback))
    }
}
