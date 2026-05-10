mod fetched;

pub use fetched::Fetched;

use brk_error::Result;
use brk_rpc::{Client, MempoolState};
use brk_types::Txid;
use parking_lot::RwLock;

use crate::{State, stores::TxStore};

/// Cap before the batch RPC so we never hand bitcoind an unbounded batch.
const MAX_TX_FETCHES_PER_CYCLE: usize = 10_000;

/// Three batched round-trips per cycle, scaling with churn rather than
/// mempool size: `getblocktemplate` + `getrawmempool false` +
/// `getmempoolinfo` in one mixed batch; then `getmempoolentry` and
/// `getrawtransaction` per *new* txid only.
///
/// `getblocktemplate` is validated to be a subset of the txid listing
/// inside the RPC layer; mismatches return `Ok(None)` so the cycle is
/// skipped without polluting downstream state.
///
/// Confirmed prevouts are resolved post-apply by the caller-supplied
/// resolver passed to `Mempool::update_with`, so the in-crate path no
/// longer issues a fourth batch for parents.
pub struct Fetcher;

impl Fetcher {
    pub fn fetch(client: &Client, lock: &RwLock<State>) -> Result<Option<Fetched>> {
        let Some(MempoolState {
            live_txids,
            gbt,
            min_fee,
        }) = client.fetch_mempool_state()?
        else {
            return Ok(None);
        };
        let new_txids = {
            let state = lock.read();
            Self::new_txids(&live_txids, &state.txs)
        };
        let new_entries = client.fetch_mempool_entries(&new_txids)?;
        let new_txs = client.get_raw_transactions(&new_txids)?;
        Ok(Some(Fetched {
            live_txids,
            new_entries,
            new_txs,
            gbt,
            min_fee,
        }))
    }

    /// Live txids the local store hasn't seen yet. Graveyard txs are
    /// included so a re-broadcast (post-reorg or a peer republishing)
    /// flows through `Preparer::classify_addition` and lands as
    /// [`crate::TxAddition::Revived`] instead of sitting orphaned for
    /// the full graveyard retention.
    fn new_txids(live_txids: &[Txid], known: &TxStore) -> Vec<Txid> {
        live_txids
            .iter()
            .filter(|txid| !known.contains(txid))
            .take(MAX_TX_FETCHES_PER_CYCLE)
            .copied()
            .collect()
    }
}
