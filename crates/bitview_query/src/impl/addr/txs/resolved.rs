use std::sync::Arc;

use bitview_plugin::PluginReadGuard;
use brk_types::{BlockHash, Transaction};

use super::ResolvedAddrChainTxs;

/// One address page with frozen mempool bodies and a guarded confirmed selection.
pub struct ResolvedAddrTxs {
    guard: PluginReadGuard,
    mempool: Vec<Arc<Transaction>>,
    chain: Option<ResolvedAddrChainTxs>,
}

impl ResolvedAddrTxs {
    /// Latest relevant block for the captured confirmed page.
    pub fn chain_anchor(&self) -> Option<BlockHash> {
        self.chain
            .as_ref()
            .map(ResolvedAddrChainTxs::activity_anchor)
    }

    pub fn mempool_transactions(&self) -> &[Arc<Transaction>] {
        &self.mempool
    }
}
pub trait RImplAddrTxsResolvedResolvedAddrTxsInternal: Sized {
    fn new(
        guard: PluginReadGuard,
        mempool: Vec<Arc<Transaction>>,
        chain: Option<ResolvedAddrChainTxs>,
    ) -> Self;
    fn into_parts(
        self,
    ) -> (
        PluginReadGuard,
        Vec<Arc<Transaction>>,
        Option<ResolvedAddrChainTxs>,
    );
}
impl RImplAddrTxsResolvedResolvedAddrTxsInternal for ResolvedAddrTxs {
    fn new(
        guard: PluginReadGuard,
        mempool: Vec<Arc<Transaction>>,
        chain: Option<ResolvedAddrChainTxs>,
    ) -> Self {
        Self {
            guard,
            mempool,
            chain,
        }
    }
    fn into_parts(
        self,
    ) -> (
        PluginReadGuard,
        Vec<Arc<Transaction>>,
        Option<ResolvedAddrChainTxs>,
    ) {
        (self.guard, self.mempool, self.chain)
    }
}
