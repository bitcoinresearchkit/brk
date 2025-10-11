use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Response,
};
use brk_structs::{AddressInfo, AddressPath};

use crate::{
    VERSION,
    extended::{HeaderMapExtended, ResponseExtended, ResultExtended, TransformResponseExtended},
};

use super::AppState;

pub trait AddressesRoutes {
    fn add_addresses_routes(self) -> Self;
}

impl AddressesRoutes for ApiRouter<AppState> {
    fn add_addresses_routes(self) -> Self {
        self.api_route(
            "/api/chain/address/{address}",
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
                .tag("Chain")
                .summary("Address information")
                .description("Retrieve comprehensive information about a Bitcoin address including balance, transaction history, UTXOs, and estimated investment metrics. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR, etc.).")
                .with_ok_response::<AddressInfo, _>(|res| res)
                .with_not_modified()
                .with_bad_request()
                .with_not_found()
                .with_server_error()
            ),
        )
    }
}
