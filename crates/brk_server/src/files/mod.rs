use axum::{routing::get, Router};

use super::AppState;

mod file;
mod minify;

use file::{file_handler, index_handler};

pub trait FilesRoutes {
    fn add_website_routes(self) -> Self;
}

impl FilesRoutes for Router<AppState> {
    fn add_website_routes(self) -> Self {
        self.route("/{*path}", get(file_handler)).route("/", get(index_handler))
    }
}
