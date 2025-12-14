use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::State,
    http::HeaderMap,
    response::{Redirect, Response},
    routing::get,
};
use brk_types::DifficultyAdjustment;

use crate::{
    VERSION,
    extended::{HeaderMapExtended, ResponseExtended, ResultExtended, TransformResponseExtended},
};

use super::AppState;

pub trait MiningRoutes {
    fn add_mining_routes(self) -> Self;
}

impl MiningRoutes for ApiRouter<AppState> {
    fn add_mining_routes(self) -> Self {
        self.route(
            "/api/v1/mining",
            get(Redirect::temporary("/api#tag/mining")),
        )
        .api_route(
            "/api/v1/difficulty-adjustment",
            get_with(
                async |headers: HeaderMap, State(state): State<AppState>| {
                    let etag = format!("{VERSION}-{}", state.get_height().await);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state
                        .get_difficulty_adjustment()
                        .await
                        .to_json_response(&etag)
                },
                |op| {
                    op.mining_tag()
                        .summary("Difficulty adjustment")
                        .description("Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.")
                        .ok_response::<DifficultyAdjustment>()
                        .not_modified()
                        .server_error()
                },
            ),
        )
    }
}
