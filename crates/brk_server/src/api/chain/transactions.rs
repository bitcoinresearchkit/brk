use aide::axum::{ApiRouter, routing::get_with};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use brk_structs::{TransactionInfo, TxidPath};

use crate::extended::{ResponseExtended, ResultExtended, TransformResponseExtended};

use super::AppState;

pub trait TransactionsRoutes {
    fn add_transactions_routes(self) -> Self;
}

impl TransactionsRoutes for ApiRouter<AppState> {
    fn add_transactions_routes(self) -> Self {
        self.api_route(
            "/api/chain/tx/{txid}",
            get_with(
                async |Path(txid): Path<TxidPath>,
                       State(app_state): State<AppState>|
                       -> Result<Response, (StatusCode, Json<String>)> {
                    let tx_info = app_state.interface.get_transaction_info(txid).to_server_result()?;

                    let bytes = sonic_rs::to_vec(&tx_info).unwrap();

                    Ok(Response::new_json_from_bytes(bytes))
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
