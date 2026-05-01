use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, Uri},
};
use brk_types::{AddrStats, AddrValidation, Transaction, Utxo, Version};

use crate::{
    AppState, CacheStrategy,
    extended::TransformResponseExtended,
    params::{AddrAfterTxidParam, AddrParam, Empty, ValidateAddrParam},
};

pub trait AddrRoutes {
    fn add_addr_routes(self) -> Self;
}

impl AddrRoutes for ApiRouter<AppState> {
    fn add_addr_routes(self) -> Self {
        self.api_route(
            "/api/address/{address}",
            get_with(async |
                uri: Uri,
                headers: HeaderMap,
                Path(path): Path<AddrParam>,
                _: Empty,
                State(state): State<AppState>
            | {
                let strategy = state.addr_strategy(Version::ONE, &path.addr, false);
                state.respond_json(&headers, strategy, &uri, move |q| q.addr(path.addr)).await
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
                _: Empty,
                State(state): State<AppState>
            | {
                let strategy = state.addr_strategy(Version::ONE, &path.addr, false);
                state.respond_json(&headers, strategy, &uri, move |q| q.addr_txs(path.addr, 50, 25)).await
            }, |op| op
                .id("get_address_txs")
                .addrs_tag()
                .summary("Address transactions")
                .description("Get transaction history for an address, sorted with newest first. Returns up to 50 mempool transactions plus the first 25 confirmed transactions. To paginate further confirmed transactions, use `/address/{address}/txs/chain/{last_seen_txid}`.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions)*")
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
                _: Empty,
                State(state): State<AppState>
            | {
                let strategy = state.addr_strategy(Version::ONE, &path.addr, true);
                state.respond_json(&headers, strategy, &uri, move |q| q.addr_txs_chain(&path.addr, None, 25)).await
            }, |op| op
                .id("get_address_confirmed_txs")
                .addrs_tag()
                .summary("Address confirmed transactions")
                .description("Get the first 25 confirmed transactions for an address. For pagination, use the path-style form `/txs/chain/{last_seen_txid}`.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-chain)*")
                .json_response::<Vec<Transaction>>()
                .not_modified()
                .bad_request()
                .not_found()
                .server_error()
            ),
        )
        .api_route(
            "/api/address/{address}/txs/chain/{after_txid}",
            get_with(async |
                uri: Uri,
                headers: HeaderMap,
                Path(path): Path<AddrAfterTxidParam>,
                _: Empty,
                State(state): State<AppState>
            | {
                let strategy = state.addr_strategy(Version::ONE, &path.addr, true);
                state.respond_json(&headers, strategy, &uri, move |q| q.addr_txs_chain(&path.addr, Some(path.after_txid), 25)).await
            }, |op| op
                .id("get_address_confirmed_txs_after")
                .addrs_tag()
                .summary("Address confirmed transactions (paginated)")
                .description("Get the next 25 confirmed transactions strictly older than `after_txid` (Esplora-canonical pagination form, matches mempool.space).\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-chain)*")
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
                _: Empty,
                State(state): State<AppState>
            | {
                let hash = state.sync(|q| q.addr_mempool_hash(&path.addr)).unwrap_or(0);
                state.respond_json(&headers, CacheStrategy::MempoolHash(hash), &uri, move |q| q.addr_mempool_txs(&path.addr, 50)).await
            }, |op| op
                .id("get_address_mempool_txs")
                .addrs_tag()
                .summary("Address mempool transactions")
                .description("Get unconfirmed transactions for an address from the mempool, newest first (up to 50).\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-transactions-mempool)*")
                .json_response::<Vec<Transaction>>()
                .not_modified()
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
                _: Empty,
                State(state): State<AppState>
            | {
                let strategy = state.addr_strategy(Version::ONE, &path.addr, false);
                state.respond_json(&headers, strategy, &uri, move |q| q.addr_utxos(path.addr, 1000)).await
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
                _: Empty,
                State(state): State<AppState>
            | {
                state.respond_json(&headers, CacheStrategy::Deploy, &uri, move |_q| Ok(AddrValidation::from_addr(&path.addr))).await
            }, |op| op
                .id("validate_address")
                .addrs_tag()
                .summary("Validate address")
                .description("Validate a Bitcoin address and get information about its type and scriptPubKey. Returns `isvalid: false` with an error message for invalid addresses.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-address-validate)*")
                .json_response::<AddrValidation>()
                .not_modified()
                .server_error()
            ),
        )
    }
}
