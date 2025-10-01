use axum::{Router, response::Redirect, routing::get};

use crate::api::{explorer::ApiExplorerRoutes, metrics::ApiMetricsRoutes};

use super::AppState;

mod explorer;
mod metrics;

pub trait ApiRoutes {
    fn add_api_routes(self) -> Self;
}

impl ApiRoutes for Router<AppState> {
    fn add_api_routes(self) -> Self {
        self.add_api_explorer_routes()
            .add_api_metrics_routes()
            .route(
                "/api",
                get(|| async {
                    Redirect::temporary(
                        "https://github.com/bitcoinresearchkit/brk/tree/main/crates/brk_server#api",
                    )
                }),
            )
    }
}
