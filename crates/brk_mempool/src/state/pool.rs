use std::sync::Arc;

use brk_error::{Error, Result};
use brk_types::{BlockHash, MempoolInfo, Timestamp, Txid};

use crate::{
    Snapshot,
    stores::{AddrTracker, OutpointSpends, ReadOnlyTxStore, TxGraveyard, TxStore},
};

use super::State;

/// One complete membership observation and its matching graph.
pub(crate) struct Pool {
    pub tip: BlockHash,
    pub info: MempoolInfo,
    pub txs: Arc<ReadOnlyTxStore>,
    pub addrs: Arc<AddrTracker>,
    pub outpoint_spends: Arc<OutpointSpends>,
    pub graveyard: Arc<TxGraveyard>,
    pub graph: Arc<Snapshot>,
    inputs_complete: bool,
}

impl Pool {
    pub fn new(
        state: &State,
        tip: BlockHash,
        graph: Arc<Snapshot>,
        previous: Option<&Self>,
    ) -> Self {
        assert_eq!(graph.content_revision(), state.txs.content_revision());
        // A new projection or chain tip does not change membership containers.
        let (txs, addrs, outpoint_spends) = match previous
            .filter(|pool| pool.txs.content_revision() == state.txs.content_revision())
        {
            Some(pool) => (
                pool.txs.clone(),
                pool.addrs.clone(),
                pool.outpoint_spends.clone(),
            ),
            None => (
                Arc::new(state.txs.read_only_clone()),
                Arc::new(state.addrs.clone()),
                Arc::new(state.outpoint_spends.clone()),
            ),
        };
        let graveyard = previous
            .filter(|pool| pool.graveyard.revision() == state.graveyard.revision())
            .map_or_else(
                || Arc::new(state.graveyard.clone()),
                |pool| pool.graveyard.clone(),
            );
        Self {
            tip,
            info: state.info.clone(),
            txs,
            addrs,
            outpoint_spends,
            graveyard,
            graph,
            inputs_complete: state.txs.unresolved().is_empty(),
        }
    }

    pub fn matches(&self, state: &State, tip: &BlockHash, graph: &Arc<Snapshot>) -> bool {
        self.tip == *tip
            && self.txs.content_revision() == state.txs.content_revision()
            && self.graveyard.revision() == state.graveyard.revision()
            && Arc::ptr_eq(&self.graph, graph)
    }

    pub fn ensure_at(&self, tip: &BlockHash) -> Result<()> {
        if self.tip != *tip {
            return Err(Error::StateUpdating);
        }
        Ok(())
    }

    pub fn ensure_resolved_at(&self, tip: &BlockHash) -> Result<()> {
        self.ensure_at(tip)?;
        if !self.inputs_complete {
            return Err(Error::StateUpdating);
        }
        Ok(())
    }

    pub fn first_seen(&self, txid: &Txid) -> Option<Timestamp> {
        self.txs
            .entry(txid)
            .map(|entry| entry.first_seen)
            .or_else(|| {
                self.graveyard
                    .get_vanished(txid)
                    .map(|tomb| tomb.entry.first_seen)
            })
    }

    /// Restart private work from a known complete publication after a panic.
    pub fn restore(&self) -> State {
        State {
            info: self.info.clone(),
            txs: TxStore::from_read_only(self.txs.as_ref().clone()),
            addrs: self.addrs.as_ref().clone(),
            outpoint_spends: self.outpoint_spends.as_ref().clone(),
            graveyard: self.graveyard.as_ref().clone(),
        }
    }
}
