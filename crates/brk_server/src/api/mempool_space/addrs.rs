use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Uri},
    response::Redirect,
    routing::get,
};
use brk_types::{AddrStats, AddrValidation, Transaction, Txid, Utxo, Version};

use crate::{
    AppState, CacheStrategy,
    extended::TransformResponseExtended,
    params::{AddrParam, AddrTxidsParam, ValidateAddrParam},
};

pub trait AddrRoutes {
    fn add_addr_routes(self) -> Self;
}

impl AddrRoutes for ApiRouter<AppState> {
    fn add_addr_routes(self) -> Self {
        self
            .route("/api/address", get(Redirect::temporary("/api/addresses")))
            .route("/api/addresses", get(Redirect::temporary("/api#tag/addresses")))
            .api_route(
            "/api/address/{address}",
            get_with(async |
                uri: Uri,
                headers: HeaderMap,
                Path(path): Path<AddrParam>,
                State(state): State<AppState>
            | {
                let strategy = state.addr_cache(Version::ONE, &path.addr);
                state.cached_json(&headers, strategy, &uri, move |q| q.addr(path.addr)).await
            }, |op| op
                .id("get_address")
                .addrs_tag()
                .summary("Address information")
                .description("Retrieve address information including balance and transaction counts. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR).\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address)*")
                .json_response::<AddrStats>()
                .not_modified()
                .bad_request()
                .not_found()
                .server_error()
            ),
        )
        .api_route(
            "/api/address/{address}/txs",
            get_with(async |
                uri: Uri,
                headers: HeaderMap,
                Path(path): Path<AddrParam>,
                Query(params): Query<AddrTxidsParam>,
                State(state): State<AppState>
            | {
                let strategy = state.addr_cache(Version::ONE, &path.addr);
                state.cached_json(&headers, strategy, &uri, move |q| q.addr_txs(path.addr, params.after_txid, 50)).await
            }, |op| op
                .id("get_address_txs")
                .addrs_tag()
                .summary("Address transactions")
                .description("Get transaction history for an address, sorted with newest first. Returns up to 50 mempool transactions plus the first 25 confirmed transactions. Use ?after_txid=<txid> for pagination.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions)*")
                .json_response::<Vec<Transaction>>()
                .not_modified()
                .bad_request()
                .not_found()
                .server_error()
            ),
        )
        .api_route(
            "/api/address/{address}/txs/chain",
            get_with(async |
                uri: Uri,
                headers: HeaderMap,
                Path(path): Path<AddrParam>,
                Query(params): Query<AddrTxidsParam>,
                State(state): State<AppState>
            | {
                let strategy = state.addr_cache(Version::ONE, &path.addr);
                state.cached_json(&headers, strategy, &uri, move |q| q.addr_txs(path.addr, params.after_txid, 25)).await
            }, |op| op
                .id("get_address_confirmed_txs")
                .addrs_tag()
                .summary("Address confirmed transactions")
                .description("Get confirmed transactions for an address, 25 per page. Use ?after_txid=<txid> for pagination.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-chain)*")
                .json_response::<Vec<Transaction>>()
                .not_modified()
                .bad_request()
                .not_found()
                .server_error()
            ),
        )
        .api_route(
            "/api/address/{address}/txs/mempool",
            get_with(async |
                uri: Uri,
                headers: HeaderMap,
                Path(path): Path<AddrParam>,
                State(state): State<AppState>
            | {
                let hash = state.sync(|q| q.addr_mempool_hash(&path.addr));
                state.cached_json(&headers, CacheStrategy::MempoolHash(hash), &uri, move |q| q.addr_mempool_txids(path.addr)).await
            }, |op| op
                .id("get_address_mempool_txs")
                .addrs_tag()
                .summary("Address mempool transactions")
                .description("Get unconfirmed transaction IDs for an address from the mempool (up to 50).\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-mempool)*")
                .json_response::<Vec<Txid>>()
                .bad_request()
                .not_found()
                .server_error()
            ),
        )
        .api_route(
            "/api/address/{address}/utxo",
            get_with(async |
                uri: Uri,
                headers: HeaderMap,
                Path(path): Path<AddrParam>,
                State(state): State<AppState>
            | {
                let strategy = state.addr_cache(Version::ONE, &path.addr);
                state.cached_json(&headers, strategy, &uri, move |q| q.addr_utxos(path.addr)).await
            }, |op| op
                .id("get_address_utxos")
                .addrs_tag()
                .summary("Address UTXOs")
                .description("Get unspent transaction outputs (UTXOs) for an address. Returns txid, vout, value, and confirmation status for each UTXO.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-utxo)*")
                .json_response::<Vec<Utxo>>()
                .not_modified()
                .bad_request()
                .not_found()
                .server_error()
            ),
        )
        .api_route(
            "/api/v1/validate-address/{address}",
            get_with(async |
                uri: Uri,
                headers: HeaderMap,
                Path(path): Path<ValidateAddrParam>,
                State(state): State<AppState>
            | {
                state.cached_json(&headers, CacheStrategy::Static, &uri, move |_q| Ok(AddrValidation::from_addr(&path.addr))).await
            }, |op| op
                .id("validate_address")
                .addrs_tag()
                .summary("Validate address")
                .description("Validate a Bitcoin address and get information about its type and scriptPubKey.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-validate)*")
                .json_response::<AddrValidation>()
                .not_modified()
            ),
        )
    }
}
