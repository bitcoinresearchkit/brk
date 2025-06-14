use axum::{Router, routing::get};

use super::AppState;

mod file;
mod website;

use file::{file_handler, index_handler};
pub use website::Website;

pub trait FilesRoutes {
    fn add_website_routes(self, website: Website) -> Self;
}

impl FilesRoutes for Router<AppState> {
    fn add_website_routes(self, website: Website) -> Self {
        if website.is_some() {
            self.route("/{*path}", get(file_handler))
                .route("/", get(index_handler))
        } else {
            self
        }
    }
}
