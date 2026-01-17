use aide::axum::ApiRouter;
use axum::{response::Redirect, routing::get};

use super::AppState;

mod file;
mod website;

use file::{file_handler, index_handler};
pub use website::*;

pub trait FilesRoutes {
    fn add_files_routes(self, website: &Website) -> Self;
}

impl FilesRoutes for ApiRouter<AppState> {
    fn add_files_routes(self, website: &Website) -> Self {
        if website.is_enabled() {
            self.route("/{*path}", get(file_handler))
                .route("/", get(index_handler))
        } else {
            self.route("/", get(Redirect::temporary("/api")))
        }
    }
}
