use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::{Redirect, Response},
    routing::get,
};
use brk_structs::{AddressInfo, AddressPath};

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
                Path(address): Path<AddressPath>,
                State(state): State<AppState>
            | {
                let etag = format!("{VERSION}-{}", state.get_height());
                if headers.has_etag(&etag) {
                    return Response::new_not_modified();
                }
                match state.get_address_info(address).with_status() {
                    Ok(value) => Response::new_json(&value, &etag),
                    Err((status, message)) => Response::new_json_with(status, &message, &etag)
                }
            }, |op| op
                .addresses_tag()
                .summary("Address information")
                .description("Retrieve comprehensive information about a Bitcoin address including balance, transaction history, UTXOs, and estimated investment metrics. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR, etc.).")
                .ok_response::<AddressInfo>()
                .not_modified()
                .bad_request()
                .not_found()
                .server_error()
            ),
        )
    }
}
