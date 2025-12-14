use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::{Redirect, Response},
    routing::get,
};
use brk_types::{Transaction, TxOutspend, TxStatus, TxidPath, TxidVoutPath};

use crate::{
    VERSION,
    extended::{HeaderMapExtended, ResponseExtended, ResultExtended, TransformResponseExtended},
};

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
                    Path(txid): Path<TxidPath>,
                    State(state): State<AppState>
                | {
                    let etag = format!("{VERSION}-{}", state.get_height().await);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state.get_transaction(txid).await.to_json_response(&etag)
                },
                |op| op
                    .transactions_tag()
                    .summary("Transaction information")
                    .description(
                        "Retrieve complete transaction data by transaction ID (txid). Returns the full transaction details including inputs, outputs, and metadata. The transaction data is read directly from the blockchain data files.",
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
                    Path(txid): Path<TxidPath>,
                    State(state): State<AppState>
                | {
                    let etag = format!("{VERSION}-{}", state.get_height().await);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state.get_transaction_status(txid).await.to_json_response(&etag)
                },
                |op| op
                    .transactions_tag()
                    .summary("Transaction status")
                    .description(
                        "Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.",
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
                    Path(txid): Path<TxidPath>,
                    State(state): State<AppState>
                | {
                    let etag = format!("{VERSION}-{}", state.get_height().await);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state.get_transaction_hex(txid).await.to_text_response(&etag)
                },
                |op| op
                    .transactions_tag()
                    .summary("Transaction hex")
                    .description(
                        "Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.",
                    )
                    .ok_response::<String>()
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
                    Path(path): Path<TxidVoutPath>,
                    State(state): State<AppState>
                | {
                    let etag = format!("{VERSION}-{}", state.get_height().await);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    let txid = TxidPath { txid: path.txid };
                    state.get_tx_outspend(txid, path.vout).await.to_json_response(&etag)
                },
                |op| op
                    .transactions_tag()
                    .summary("Output spend status")
                    .description(
                        "Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details.",
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
                    Path(txid): Path<TxidPath>,
                    State(state): State<AppState>
                | {
                    let etag = format!("{VERSION}-{}", state.get_height().await);
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    state.get_tx_outspends(txid).await.to_json_response(&etag)
                },
                |op| op
                    .transactions_tag()
                    .summary("All output spend statuses")
                    .description(
                        "Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.",
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
