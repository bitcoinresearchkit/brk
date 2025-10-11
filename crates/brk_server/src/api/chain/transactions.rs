use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Response,
};
use brk_structs::{TransactionInfo, TxidPath};

use crate::{
    VERSION,
    extended::{HeaderMapExtended, ResponseExtended, ResultExtended, TransformResponseExtended},
};

use super::AppState;

pub trait TransactionsRoutes {
    fn add_transactions_routes(self) -> Self;
}

impl TransactionsRoutes for ApiRouter<AppState> {
    fn add_transactions_routes(self) -> Self {
        self.api_route(
            "/api/chain/tx/{txid}",
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
                    .tag("Chain")
                    .summary("Transaction information")
                    .description(
                        "Retrieve complete transaction data by transaction ID (txid). Returns the full transaction details including inputs, outputs, and metadata. The transaction data is read directly from the blockchain data files.",
                    )
                    .with_ok_response::<TransactionInfo, _>(|res| res)
                    .with_not_modified()
                    .with_bad_request()
                    .with_not_found()
                    .with_server_error(),
            ),
        )
    }
}
