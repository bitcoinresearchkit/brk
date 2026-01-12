use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::Redirect,
    routing::get,
};
use brk_types::{
    AddressParam, AddressStats, AddressTxidsParam, AddressValidation, Txid, Utxo,
    ValidateAddressParam,
};

use crate::{CacheStrategy, extended::TransformResponseExtended};

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
                Path(path): Path<AddressParam>,
                State(state): State<AppState>
            | {
                state.cached_json(&headers, CacheStrategy::Height, move |q| q.address(path.address)).await
            }, |op| op
                .id("get_address")
                .addresses_tag()
                .summary("Address information")
                .description("Retrieve address information including balance and transaction counts. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR).\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address)*")
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
                Path(path): Path<AddressParam>,
                Query(params): Query<AddressTxidsParam>,
                State(state): State<AppState>
            | {
                state.cached_json(&headers, CacheStrategy::Height, move |q| q.address_txids(path.address, params.after_txid, params.limit)).await
            }, |op| op
                .id("get_address_txs")
                .addresses_tag()
                .summary("Address transaction IDs")
                .description("Get transaction IDs for an address, newest first. Use after_txid for pagination.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions)*")
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
                Path(path): Path<AddressParam>,
                State(state): State<AppState>
            | {
                state.cached_json(&headers, CacheStrategy::Height, move |q| q.address_utxos(path.address)).await
            }, |op| op
                .id("get_address_utxos")
                .addresses_tag()
                .summary("Address UTXOs")
                .description("Get unspent transaction outputs (UTXOs) for an address. Returns txid, vout, value, and confirmation status for each UTXO.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-utxo)*")
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
                Path(path): Path<AddressParam>,
                State(state): State<AppState>
            | {
                // Mempool txs for an address - use MaxAge since it's volatile
                state.cached_json(&headers, CacheStrategy::MaxAge(5), move |q| q.address_mempool_txids(path.address)).await
            }, |op| op
                .id("get_address_mempool_txs")
                .addresses_tag()
                .summary("Address mempool transactions")
                .description("Get unconfirmed transaction IDs for an address from the mempool (up to 50).\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-mempool)*")
                .ok_response::<Vec<Txid>>()
                .bad_request()
                .not_found()
                .server_error()
            ),
        )
        .api_route(
            "/api/address/{address}/txs/chain",
            get_with(async |
                headers: HeaderMap,
                Path(path): Path<AddressParam>,
                Query(params): Query<AddressTxidsParam>,
                State(state): State<AppState>
            | {
                state.cached_json(&headers, CacheStrategy::Height, move |q| q.address_txids(path.address, params.after_txid, 25)).await
            }, |op| op
                .id("get_address_confirmed_txs")
                .addresses_tag()
                .summary("Address confirmed transactions")
                .description("Get confirmed transaction IDs for an address, 25 per page. Use ?after_txid=<txid> for pagination.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-chain)*")
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
                Path(path): Path<ValidateAddressParam>,
                State(state): State<AppState>
            | {
                state.cached_json(&headers, CacheStrategy::Static, move |_q| Ok(AddressValidation::from_address(&path.address))).await
            }, |op| op
                .id("validate_address")
                .addresses_tag()
                .summary("Validate address")
                .description("Validate a Bitcoin address and get information about its type and scriptPubKey.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-validate)*")
                .ok_response::<AddressValidation>()
                .not_modified()
            ),
        )
    }
}
