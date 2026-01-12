use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Redirect,
    routing::get,
};
use brk_types::{Hex, Transaction, TxOutspend, TxStatus, TxidParam, TxidVout};

use crate::{CacheStrategy, extended::TransformResponseExtended};

use super::AppState;

pub trait TxRoutes {
    fn add_tx_routes(self) -> Self;
}

impl TxRoutes for ApiRouter<AppState> {
    fn add_tx_routes(self) -> Self {
        self
            .route("/api/tx", get(Redirect::temporary("/api/transactions")))
            .route("/api/transactions", get(Redirect::temporary("/api#tag/transactions")))
            .api_route(
            "/api/tx/{txid}",
            get_with(
                async |
                    headers: HeaderMap,
                    Path(txid): Path<TxidParam>,
                    State(state): State<AppState>
                | {
                    state.cached_json(&headers, CacheStrategy::Height, move |q| q.transaction(txid)).await
                },
                |op| op
                    .id("get_tx")
                    .transactions_tag()
                    .summary("Transaction information")
                    .description(
                        "Retrieve complete transaction data by transaction ID (txid). Returns inputs, outputs, fee, size, and confirmation status.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction)*",
                    )
                    .ok_response::<Transaction>()
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
                    headers: HeaderMap,
                    Path(txid): Path<TxidParam>,
                    State(state): State<AppState>
                | {
                    state.cached_json(&headers, CacheStrategy::Height, move |q| q.transaction_status(txid)).await
                },
                |op| op
                    .id("get_tx_status")
                    .transactions_tag()
                    .summary("Transaction status")
                    .description(
                        "Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-status)*",
                    )
                    .ok_response::<TxStatus>()
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
                    headers: HeaderMap,
                    Path(txid): Path<TxidParam>,
                    State(state): State<AppState>
                | {
                    state.cached_text(&headers, CacheStrategy::Height, move |q| q.transaction_hex(txid)).await
                },
                |op| op
                    .id("get_tx_hex")
                    .transactions_tag()
                    .summary("Transaction hex")
                    .description(
                        "Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-hex)*",
                    )
                    .ok_response::<Hex>()
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
                    headers: HeaderMap,
                    Path(path): Path<TxidVout>,
                    State(state): State<AppState>
                | {
                    let txid = TxidParam { txid: path.txid };
                    state.cached_json(&headers, CacheStrategy::Height, move |q| q.outspend(txid, path.vout)).await
                },
                |op| op
                    .id("get_tx_outspend")
                    .transactions_tag()
                    .summary("Output spend status")
                    .description(
                        "Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspend)*",
                    )
                    .ok_response::<TxOutspend>()
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
                    headers: HeaderMap,
                    Path(txid): Path<TxidParam>,
                    State(state): State<AppState>
                | {
                    state.cached_json(&headers, CacheStrategy::Height, move |q| q.outspends(txid)).await
                },
                |op| op
                    .id("get_tx_outspends")
                    .transactions_tag()
                    .summary("All output spend statuses")
                    .description(
                        "Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.\n\n*[Mempool.space docs](https://mempool.space/docs/api/rest#get-transaction-outspends)*",
                    )
                    .ok_response::<Vec<TxOutspend>>()
                    .not_modified()
                    .bad_request()
                    .not_found()
                    .server_error(),
            ),
        )
    }
}
