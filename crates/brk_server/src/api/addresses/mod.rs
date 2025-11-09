use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::{Redirect, Response},
    routing::get,
};
use brk_types::{Address, AddressStats};

use crate::{
    VERSION,
    extended::{HeaderMapExtended, ResponseExtended, ResultExtended, TransformResponseExtended},
};

use super::AppState;

pub trait AddressRoutes {
    fn add_addresses_routes(self) -> Self;
}

impl AddressRoutes for ApiRouter<AppState> {
    fn add_addresses_routes(self) -> Self {
        self
            .route("/api/address", get(Redirect::temporary("/api/addresses")))
            .route("/api/addresses", get(Redirect::temporary("/api#tag/addresses")))
            .api_route(
            "/api/address/{address}",
            get_with(async |
                headers: HeaderMap,
                Path(address): Path<Address>,
                State(state): State<AppState>
            | {
                let etag = format!("{VERSION}-{}", state.get_height().await);
                if headers.has_etag(&etag) {
                    return Response::new_not_modified();
                }
                state.get_address(address).await.to_json_response(&etag)
            }, |op| op
                .addresses_tag()
                .summary("Address information")
                .description("Retrieve comprehensive information about a Bitcoin address including balance, transaction history, UTXOs, and estimated investment metrics. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR, etc.).")
                .ok_response::<AddressStats>()
                .not_modified()
                .bad_request()
                .not_found()
                .server_error()
            ),
        )
    }
}
