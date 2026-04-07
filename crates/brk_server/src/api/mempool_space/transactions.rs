use aide::axum::{
    ApiRouter,
    routing::{get_with, post_with},
};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, Uri},
};
use brk_types::{CpfpInfo, MerkleProof, Transaction, TxOutspend, TxStatus, Txid, Version};

use crate::{
    AppState, CacheStrategy,
    cache::CacheParams,
    extended::{ResponseExtended, TransformResponseExtended},
    params::{TxidParam, TxidVout, TxidsParam},
};

pub trait TxRoutes {
    fn add_tx_routes(self) -> Self;
}

impl TxRoutes for ApiRouter<AppState> {
    fn add_tx_routes(self) -> Self {
        self
            .api_route(
            "/api/v1/cpfp/{txid}",
            get_with(
                async |uri: Uri, headers: HeaderMap, Path(param): Path<TxidParam>, State(state): State<AppState>| {
                    state.cached_json(&headers, state.tx_cache(Version::ONE, &param.txid), &uri, move |q| q.cpfp(&param.txid)).await
                },
                |op| op
                    .id("get_cpfp")
                    .transactions_tag()
                    .summary("CPFP info")
                    .description("Returns ancestors and descendants for a CPFP (Child Pays For Parent) transaction, including the effective fee rate of the package.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-children-pay-for-parent)*")
                    .json_response::<CpfpInfo>()
                    .not_modified()
                    .bad_request()
                    .not_found()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/tx/{txid}",
            get_with(
                async |
                    uri: Uri,
                    headers: HeaderMap,
                    Path(param): Path<TxidParam>,
                    State(state): State<AppState>
                | {
                    state.cached_json(&headers, state.tx_cache(Version::ONE, &param.txid), &uri, move |q| q.transaction(&param.txid)).await
                },
                |op| op
                    .id("get_tx")
                    .transactions_tag()
                    .summary("Transaction information")
                    .description(
                        "Retrieve complete transaction data by transaction ID (txid). Returns inputs, outputs, fee, size, and confirmation status.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction)*",
                    )
                    .json_response::<Transaction>()
                    .not_modified()
                    .bad_request()
                    .not_found()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/tx/{txid}/hex",
            get_with(
                async |
                    uri: Uri,
                    headers: HeaderMap,
                    Path(param): Path<TxidParam>,
                    State(state): State<AppState>
                | {
                    state.cached_text(&headers, state.tx_cache(Version::ONE, &param.txid), &uri, move |q| q.transaction_hex(&param.txid)).await
                },
                |op| op
                    .id("get_tx_hex")
                    .transactions_tag()
                    .summary("Transaction hex")
                    .description(
                        "Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-hex)*",
                    )
                    .text_response()
                    .not_modified()
                    .bad_request()
                    .not_found()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/tx/{txid}/merkleblock-proof",
            get_with(
                async |uri: Uri, headers: HeaderMap, Path(param): Path<TxidParam>, State(state): State<AppState>| {
                    state.cached_text(&headers, state.tx_cache(Version::ONE, &param.txid), &uri, move |q| q.merkleblock_proof(&param.txid)).await
                },
                |op| op
                    .id("get_tx_merkleblock_proof")
                    .transactions_tag()
                    .summary("Transaction merkleblock proof")
                    .description("Get the merkleblock proof for a transaction (BIP37 format, hex encoded).\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-merkleblock-proof)*")
                    .text_response()
                    .not_modified()
                    .bad_request()
                    .not_found()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/tx/{txid}/merkle-proof",
            get_with(
                async |uri: Uri, headers: HeaderMap, Path(param): Path<TxidParam>, State(state): State<AppState>| {
                    state.cached_json(&headers, state.tx_cache(Version::ONE, &param.txid), &uri, move |q| q.merkle_proof(&param.txid)).await
                },
                |op| op
                    .id("get_tx_merkle_proof")
                    .transactions_tag()
                    .summary("Transaction merkle proof")
                    .description("Get the merkle inclusion proof for a transaction.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-merkle-proof)*")
                    .json_response::<MerkleProof>()
                    .not_modified()
                    .bad_request()
                    .not_found()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/tx/{txid}/outspend/{vout}",
            get_with(
                async |
                    uri: Uri,
                    headers: HeaderMap,
                    Path(path): Path<TxidVout>,
                    State(state): State<AppState>
                | {
                    let v = Version::ONE;
                    let immutable = CacheParams::immutable(v);
                    if immutable.matches_etag(&headers) {
                        return ResponseExtended::new_not_modified_with(&immutable);
                    }
                    let outspend = state.run(move |q| q.outspend(&path.txid, path.vout)).await;
                    let height = state.sync(|q| q.height());
                    let is_deep = outspend.as_ref().is_ok_and(|o| o.is_deeply_spent(height));
                    let strategy = if is_deep { CacheStrategy::Immutable(v) } else { CacheStrategy::Tip };
                    state.cached_json(&headers, strategy, &uri, move |_| outspend).await
                },
                |op| op
                    .id("get_tx_outspend")
                    .transactions_tag()
                    .summary("Output spend status")
                    .description(
                        "Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspend)*",
                    )
                    .json_response::<TxOutspend>()
                    .not_modified()
                    .bad_request()
                    .not_found()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/tx/{txid}/outspends",
            get_with(
                async |
                    uri: Uri,
                    headers: HeaderMap,
                    Path(param): Path<TxidParam>,
                    State(state): State<AppState>
                | {
                    let v = Version::ONE;
                    let immutable = CacheParams::immutable(v);
                    if immutable.matches_etag(&headers) {
                        return ResponseExtended::new_not_modified_with(&immutable);
                    }
                    let outspends = state.run(move |q| q.outspends(&param.txid)).await;
                    let height = state.sync(|q| q.height());
                    let all_deep = outspends.as_ref().is_ok_and(|os| os.iter().all(|o| o.is_deeply_spent(height)));
                    let strategy = if all_deep { CacheStrategy::Immutable(v) } else { CacheStrategy::Tip };
                    state.cached_json(&headers, strategy, &uri, move |_| outspends).await
                },
                |op| op
                    .id("get_tx_outspends")
                    .transactions_tag()
                    .summary("All output spend statuses")
                    .description(
                        "Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspends)*",
                    )
                    .json_response::<Vec<TxOutspend>>()
                    .not_modified()
                    .bad_request()
                    .not_found()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/tx/{txid}/raw",
            get_with(
                async |uri: Uri, headers: HeaderMap, Path(param): Path<TxidParam>, State(state): State<AppState>| {
                    state.cached_bytes(&headers, state.tx_cache(Version::ONE, &param.txid), &uri, move |q| q.transaction_raw(&param.txid)).await
                },
                |op| op
                    .id("get_tx_raw")
                    .transactions_tag()
                    .summary("Transaction raw")
                    .description("Returns a transaction as binary data.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-raw)*")
                    .binary_response()
                    .not_modified()
                    .bad_request()
                    .not_found()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/tx/{txid}/status",
            get_with(
                async |
                    uri: Uri,
                    headers: HeaderMap,
                    Path(param): Path<TxidParam>,
                    State(state): State<AppState>
                | {
                    state.cached_json(&headers, state.tx_cache(Version::ONE, &param.txid), &uri, move |q| q.transaction_status(&param.txid)).await
                },
                |op| op
                    .id("get_tx_status")
                    .transactions_tag()
                    .summary("Transaction status")
                    .description(
                        "Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-status)*",
                    )
                    .json_response::<TxStatus>()
                    .not_modified()
                    .bad_request()
                    .not_found()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/v1/transaction-times",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    let params = TxidsParam::from_query(uri.query().unwrap_or(""));
                    state.cached_json(&headers, state.mempool_cache(), &uri, move |q| q.transaction_times(&params.txids)).await
                },
                |op| op
                    .id("get_transaction_times")
                    .transactions_tag()
                    .summary("Transaction first-seen times")
                    .description("Returns timestamps when transactions were first seen in the mempool. Returns 0 for mined or unknown transactions.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-times)*")
                    .json_response::<Vec<u64>>()
                    .not_modified()
                    .server_error(),
            ),
        )
        .api_route(
            "/api/tx",
            post_with(
                async |State(state): State<AppState>, body: String| {
                    let hex = body.trim().to_string();
                    state.run(move |q| q.broadcast_transaction(&hex))
                        .await
                        .map(|txid| txid.to_string())
                        .map_err(crate::Error::from)
                },
                |op| {
                    op.id("post_tx")
                        .transactions_tag()
                        .summary("Broadcast transaction")
                        .description("Broadcast a raw transaction to the network. The transaction should be provided as hex in the request body. The txid will be returned on success.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#post-transaction)*")
                        .json_response::<Txid>()
                        .bad_request()
                        .server_error()
                },
            ),
        )
    }
}
