use aide::axum::ApiRouter;
use axum::{response::Redirect, routing::get};

use crate::api::chain::{addresses::AddressesRoutes, transactions::TransactionsRoutes};

use super::AppState;

mod addresses;
mod transactions;

pub trait ChainRoutes {
    fn add_chain_routes(self) -> Self;
}

impl ChainRoutes for ApiRouter<AppState> {
    fn add_chain_routes(self) -> Self {
        self.route("/api/chain", get(Redirect::temporary("/api#tag/chain")))
            .add_addresses_routes()
            .add_transactions_routes()
    }
}
