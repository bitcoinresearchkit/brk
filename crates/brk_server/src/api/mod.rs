use axum::{Router, routing::get};

use super::AppState;

mod explorer;
mod query;

pub use query::DTS;

pub trait ApiRoutes {
    fn add_api_routes(self) -> Self;
}

impl ApiRoutes for Router<AppState> {
    fn add_api_routes(self) -> Self {
        self.route("/api/query", get(query::handler))
    }
}
