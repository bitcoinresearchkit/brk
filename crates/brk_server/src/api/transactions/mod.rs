use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::{Redirect, Response},
    routing::get,
};
use brk_types::{Transaction, TxStatus, TxidPath};

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
    }
}
