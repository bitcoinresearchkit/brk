use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::{Redirect, Response},
    routing::get,
};
use brk_structs::{TransactionInfo, TxidPath};

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
                    let etag = format!("{VERSION}-{}", state.get_height());
                    if headers.has_etag(&etag) {
                        return Response::new_not_modified();
                    }
                    match state.get_transaction_info(txid).with_status() {
                        Ok(value) => Response::new_json(&value, &etag),
                        Err((status, message)) => Response::new_json_with(status, &message, &etag)
                    }
                },
                |op| op
                    .transactions_tag()
                    .summary("Transaction information")
                    .description(
                        "Retrieve complete transaction data by transaction ID (txid). Returns the full transaction details including inputs, outputs, and metadata. The transaction data is read directly from the blockchain data files.",
                    )
                    .ok_response::<TransactionInfo>()
                    .not_modified()
                    .bad_request()
                    .not_found()
                    .server_error(),
            ),
        )
    }
}
