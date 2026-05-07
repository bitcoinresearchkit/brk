mod fetched;

pub use fetched::Fetched;

use brk_error::Result;
use brk_rpc::{Client, MempoolState};
use brk_types::{MempoolEntryInfo, Txid};
use parking_lot::RwLock;

use crate::{
    MempoolInner,
    stores::{TxGraveyard, TxStore},
};

/// Cap before the batch RPC so we never hand bitcoind an unbounded batch.
const MAX_TX_FETCHES_PER_CYCLE: usize = 10_000;

/// Two batched round-trips per cycle regardless of mempool size:
/// `getrawmempool verbose` + `getblocktemplate` + `getmempoolinfo` in
/// one mixed batch, then `getrawtransaction` for new txs.
///
/// `getblocktemplate` is validated to be a subset of the verbose
/// listing inside the RPC layer; mismatches return `Ok(None)` so the
/// cycle is skipped without polluting downstream state.
///
/// Confirmed prevouts are resolved post-apply by the caller-supplied
/// resolver passed to `Mempool::update_with`, so the in-crate path no
/// longer issues a third batch for parents.
pub struct Fetcher;

impl Fetcher {
    pub fn fetch(client: &Client, lock: &RwLock<MempoolInner>) -> Result<Option<Fetched>> {
        let Some(MempoolState {
            entries,
            gbt,
            min_fee,
        }) = client.fetch_mempool_state()?
        else {
            return Ok(None);
        };
        let new_txids = {
            let inner = lock.read();
            Self::new_txids(&entries, &inner.txs, &inner.graveyard)
        };
        let new_raws = client.get_raw_transactions(&new_txids)?;
        Ok(Some(Fetched {
            entries_info: entries,
            new_raws,
            gbt,
            min_fee,
        }))
    }

    fn new_txids(
        entries_info: &[MempoolEntryInfo],
        known: &TxStore,
        graveyard: &TxGraveyard,
    ) -> Vec<Txid> {
        entries_info
            .iter()
            .filter(|info| !known.contains(&info.txid) && !graveyard.contains(&info.txid))
            .take(MAX_TX_FETCHES_PER_CYCLE)
            .map(|info| info.txid)
            .collect()
    }
}
