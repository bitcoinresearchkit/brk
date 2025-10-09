use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use brk_structs::{AddressInfo, AddressPath};

use crate::extended::{ResponseExtended, ResultExtended, TransformResponseExtended};

use super::AppState;

pub trait AddressesRoutes {
    fn add_addresses_routes(self) -> Self;
}

impl AddressesRoutes for ApiRouter<AppState> {
    fn add_addresses_routes(self) -> Self {
        self.api_route(
            "/api/chain/address/{address}",
            get_with(async |Path(address): Path<AddressPath>,
                   State(app_state): State<AppState>|
                   -> Result<Response, (StatusCode, Json<String>)> {
                let address_info = app_state.interface.get_address_info(address).to_server_result()?;

                let bytes = sonic_rs::to_vec(&address_info).unwrap();

                Ok(Response::new_json_from_bytes(bytes))
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
