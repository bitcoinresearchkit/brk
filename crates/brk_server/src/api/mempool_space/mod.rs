use aide::axum::ApiRouter;

use crate::AppState;

mod addrs;
mod blocks;
mod fees;
mod general;
mod mempool;
mod mining;
mod transactions;

use addrs::AddrRoutes;
use blocks::BlockRoutes;
use fees::FeesRoutes;
use general::GeneralRoutes;
use mempool::MempoolRoutes;
use mining::MiningRoutes;
use transactions::TxRoutes;

pub trait MempoolSpaceRoutes {
    fn add_mempool_space_routes(self) -> Self;
}

impl MempoolSpaceRoutes for ApiRouter<AppState> {
    fn add_mempool_space_routes(self) -> Self {
        self.add_general_routes()
            .add_addr_routes()
            .add_block_routes()
            .add_mining_routes()
            .add_fees_routes()
            .add_mempool_routes()
            .add_tx_routes()
    }
}
