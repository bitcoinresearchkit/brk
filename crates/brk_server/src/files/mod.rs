use std::path::PathBuf;

use aide::axum::ApiRouter;
use axum::routing::get;

use super::AppState;

mod file;

use file::{file_handler, index_handler};

pub trait FilesRoutes {
    fn add_files_routes(self, path: Option<&PathBuf>) -> Self;
}

impl FilesRoutes for ApiRouter<AppState> {
    fn add_files_routes(self, path: Option<&PathBuf>) -> Self {
        if path.is_some() {
            self.route("/{*path}", get(file_handler))
                .route("/", get(index_handler))
        } else {
            self
        }
    }
}
