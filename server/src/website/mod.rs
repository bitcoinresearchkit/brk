use axum::{routing::get, Router};

mod handlers;

use handlers::{file_handler, index_handler};

use super::AppState;

pub trait WebsiteRoutes {
    fn add_website_routes(self) -> Self;
}

impl WebsiteRoutes for Router<AppState> {
    fn add_website_routes(self) -> Self {
        self.route("/{*path}", get(file_handler)).route("/", get(index_handler))
    }
}
