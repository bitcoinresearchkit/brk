use brk_rpc::Auth;
use brk_types::{FeeRate, Txid};

use super::*;

impl Mempool {
    /// Test-only constructor that wires a Client at the default URL without
    /// touching the network. `simple_http` only parses the URL on init.
    pub fn for_test() -> Self {
        let client = Client::new(Client::default_url(), Auth::None).unwrap();
        Self(Arc::new(Inner {
            client,
            state: RwLock::new(State::default()),
            rebuilder: Rebuilder::default(),
            started: AtomicBool::new(false),
            cycle: Mutex::new(()),
        }))
    }

    pub fn test_state_lock(&self) -> &RwLock<State> {
        &self.0.state
    }

    pub fn test_tick(&self, gbt_txids: &[Txid], min_fee: FeeRate) {
        self.0
            .rebuilder
            .tick(&self.0.state, gbt_txids, min_fee, true);
    }
}
