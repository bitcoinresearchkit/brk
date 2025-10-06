use aide::axum::ApiRouter;
use axum::{response::Html, routing::get};

use crate::api::{chain::ApiExplorerRoutes, metrics::ApiMetricsRoutes};

use super::AppState;

mod chain;
mod metrics;

pub trait ApiRoutes {
    fn add_api_routes(self) -> Self;
}

impl ApiRoutes for ApiRouter<AppState> {
    fn add_api_routes(self) -> Self {
        self.add_api_explorer_routes()
            .add_api_metrics_routes()
            .route("/api", get(Html::from(include_str!("./scalar.html"))))
    }
}
