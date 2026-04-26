mod fetched;

pub use fetched::Fetched;

use brk_error::Result;
use brk_rpc::{Client, RawTx};
use brk_types::{MempoolEntryInfo, Txid};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::stores::{TxGraveyard, TxStore};

/// Cap on how many new txs we fetch per cycle (applied before the batch RPC
/// so we never hand bitcoind an unbounded batch).
const MAX_TX_FETCHES_PER_CYCLE: usize = 10_000;

/// Talks to Bitcoin Core. Three batched round-trips regardless of
/// mempool size:
/// 1. `getrawmempool verbose` - authoritative listing
/// 2. `getrawtransaction` batch - every new tx (txids not in
///    `known` / `graveyard`, capped at `MAX_TX_FETCHES_PER_CYCLE`)
/// 3. `getrawtransaction` batch - unique confirmed parents of those
///    new txs that aren't resolvable from `known` or step 2.
///
/// Step 3 is best-effort: without `-txindex`, Core returns -5 for every
/// confirmed parent and the batch yields an empty map. `brk_query`
/// fills missing prevouts at read time from the indexer, so this is
/// purely a latency optimization when `-txindex` is available.
pub struct Fetcher;

impl Fetcher {
    pub fn fetch(client: &Client, known: &TxStore, graveyard: &TxGraveyard) -> Result<Fetched> {
        let entries_info = client.get_raw_mempool_verbose()?;

        let new_txids = Self::new_txids(&entries_info, known, graveyard);
        let new_raws = client.get_raw_transactions(&new_txids)?;

        let parent_txids = Self::unique_confirmed_parents(&new_raws, known);
        let parent_raws = client.get_raw_transactions(&parent_txids)?;

        Ok(Fetched {
            entries_info,
            new_raws,
            parent_raws,
        })
    }

    /// Txids in the listing that we don't already have cached (live or
    /// buried) and therefore need to fetch raw bytes for. Order-preserving
    /// so the batch matches the listing order for debuggability.
    fn new_txids(
        entries_info: &[MempoolEntryInfo],
        known: &TxStore,
        graveyard: &TxGraveyard,
    ) -> Vec<Txid> {
        entries_info
            .iter()
            .filter(|info| !known.contains(&info.txid) && !graveyard.contains(&info.txid))
            .take(MAX_TX_FETCHES_PER_CYCLE)
            .map(|info| info.txid.clone())
            .collect()
    }

    /// Parent txids referenced by `new_raws` inputs that aren't already
    /// resolvable: not in the mempool store, not in `new_raws` itself.
    fn unique_confirmed_parents(
        new_raws: &FxHashMap<Txid, RawTx>,
        known: &TxStore,
    ) -> Vec<Txid> {
        let mut set: FxHashSet<Txid> = FxHashSet::default();
        for raw in new_raws.values() {
            for txin in &raw.tx.input {
                let prev: Txid = txin.previous_output.txid.into();
                if !known.contains_key(&prev) && !new_raws.contains_key(&prev) {
                    set.insert(prev);
                }
            }
        }
        set.into_iter().collect()
    }
}
