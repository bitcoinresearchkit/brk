use axum::{Router, routing::get};

use super::AppState;

mod file;
mod frontend;
mod minify;

use file::{file_handler, index_handler};
pub use frontend::Frontend;

pub trait FilesRoutes {
    fn add_website_routes(self, frontend: Frontend) -> Self;
}

impl FilesRoutes for Router<AppState> {
    fn add_website_routes(self, frontend: Frontend) -> Self {
        if frontend.is_some() {
            self.route("/{*path}", get(file_handler))
                .route("/", get(index_handler))
        } else {
            self
        }
    }
}
