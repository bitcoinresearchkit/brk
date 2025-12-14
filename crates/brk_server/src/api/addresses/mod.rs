use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{Redirect, Response},
    routing::get,
};
use brk_query::validate_address;
use brk_types::{Address, AddressStats, AddressTxidsParam, AddressValidation, Txid, Utxo};

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
        .api_route(
            "/api/address/{address}/txs",
            get_with(async |
                headers: HeaderMap,
                Path(address): Path<Address>,
                Query(params): Query<AddressTxidsParam>,
                State(state): State<AppState>
            | {
                let etag = format!("{VERSION}-{}", state.get_height().await);
                if headers.has_etag(&etag) {
                    return Response::new_not_modified();
                }
                state.get_address_txids(address, params.after_txid, params.limit).await.to_json_response(&etag)
            }, |op| op
                .addresses_tag()
                .summary("Address transaction IDs")
                .description("Get transaction IDs for an address, newest first. Use after_txid for pagination.")
                .ok_response::<Vec<Txid>>()
                .not_modified()
                .bad_request()
                .not_found()
                .server_error()
            ),
        )
        .api_route(
            "/api/address/{address}/utxo",
            get_with(async |
                headers: HeaderMap,
                Path(address): Path<Address>,
                State(state): State<AppState>
            | {
                let etag = format!("{VERSION}-{}", state.get_height().await);
                if headers.has_etag(&etag) {
                    return Response::new_not_modified();
                }
                state.get_address_utxos(address).await.to_json_response(&etag)
            }, |op| op
                .addresses_tag()
                .summary("Address UTXOs")
                .description("Get unspent transaction outputs for an address.")
                .ok_response::<Vec<Utxo>>()
                .not_modified()
                .bad_request()
                .not_found()
                .server_error()
            ),
        )
        .api_route(
            "/api/address/{address}/txs/mempool",
            get_with(async |
                headers: HeaderMap,
                Path(address): Path<Address>,
                State(state): State<AppState>
            | {
                let etag = format!("{VERSION}-{}", state.get_height().await);
                if headers.has_etag(&etag) {
                    return Response::new_not_modified();
                }
                state.get_address_mempool_txids(address).await.to_json_response(&etag)
            }, |op| op
                .addresses_tag()
                .summary("Address mempool transactions")
                .description("Get unconfirmed transaction IDs for an address from the mempool (up to 50).")
                .ok_response::<Vec<Txid>>()
                .not_modified()
                .bad_request()
                .not_found()
                .server_error()
            ),
        )
        .api_route(
            "/api/address/{address}/txs/chain/{after_txid}",
            get_with(async |
                headers: HeaderMap,
                Path((address, after_txid)): Path<(Address, Option<Txid>)>,
                State(state): State<AppState>
            | {
                let etag = format!("{VERSION}-{}", state.get_height().await);
                if headers.has_etag(&etag) {
                    return Response::new_not_modified();
                }
                state.get_address_txids(address, after_txid, 25).await.to_json_response(&etag)
            }, |op| op
                .addresses_tag()
                .summary("Address confirmed transactions")
                .description("Get confirmed transaction IDs for an address, 25 per page. Use after_txid for pagination.")
                .ok_response::<Vec<Txid>>()
                .not_modified()
                .bad_request()
                .not_found()
                .server_error()
            ),
        )
        .api_route(
            "/api/v1/validate-address/{address}",
            get_with(async |
                headers: HeaderMap,
                Path(address): Path<String>,
                State(state): State<AppState>
            | {
                let etag = VERSION;
                if headers.has_etag(etag) {
                    return Response::new_not_modified();
                }
                Response::new_json(validate_address(&address), etag)
            }, |op| op
                .addresses_tag()
                .summary("Validate address")
                .description("Validate a Bitcoin address and get information about its type and scriptPubKey.")
                .ok_response::<AddressValidation>()
                .not_modified()
            ),
        )
    }
}
