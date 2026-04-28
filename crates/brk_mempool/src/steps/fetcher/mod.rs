mod fetched;

pub use fetched::Fetched;

use brk_error::Result;
use brk_rpc::{Client, RawTx};
use brk_types::{MempoolEntryInfo, Txid};
use rustc_hash::FxHashMap;

use crate::stores::{MempoolState, TxGraveyard, TxStore};

/// Cap before the batch RPC so we never hand bitcoind an unbounded batch.
const MAX_TX_FETCHES_PER_CYCLE: usize = 10_000;

/// Three batched round-trips per cycle regardless of mempool size:
/// `getrawmempool verbose`, then `getrawtransaction` for new txs, then
/// `getrawtransaction` for confirmed parents.
///
/// The third batch is best-effort. Without `-txindex` Core returns -5
/// for every confirmed parent. `brk_query` fills missing prevouts at
/// read time from the indexer, so this is purely a latency
/// optimization when `-txindex` is available.
pub struct Fetcher;

impl Fetcher {
    pub fn fetch(client: &Client, state: &MempoolState) -> Result<Fetched> {
        let entries_info = Self::list_pool(client)?;
        let new_raws = Self::fetch_new(client, state, &entries_info)?;
        let parent_raws = Self::fetch_parents(client, state, &new_raws)?;
        Ok(Fetched {
            entries_info,
            new_raws,
            parent_raws,
        })
    }

    fn list_pool(client: &Client) -> Result<Vec<MempoolEntryInfo>> {
        client.get_raw_mempool_verbose()
    }

    fn fetch_new(
        client: &Client,
        state: &MempoolState,
        entries_info: &[MempoolEntryInfo],
    ) -> Result<FxHashMap<Txid, RawTx>> {
        let new_txids = {
            let known = state.txs.read();
            let graveyard = state.graveyard.read();
            Self::new_txids(entries_info, &known, &graveyard)
        };
        client.get_raw_transactions(&new_txids)
    }

    fn fetch_parents(
        client: &Client,
        state: &MempoolState,
        new_raws: &FxHashMap<Txid, RawTx>,
    ) -> Result<FxHashMap<Txid, RawTx>> {
        let parent_txids = {
            let known = state.txs.read();
            Self::unique_confirmed_parents(new_raws, &known)
        };
        client.get_raw_transactions(&parent_txids)
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
            .map(|info| info.txid.clone())
            .collect()
    }

    fn unique_confirmed_parents(new_raws: &FxHashMap<Txid, RawTx>, known: &TxStore) -> Vec<Txid> {
        let mut v = new_raws
            .values()
            .flat_map(|raw| &raw.tx.input)
            .map(|txin| Txid::from(txin.previous_output.txid))
            .filter(|prev| !known.contains(prev) && !new_raws.contains_key(prev))
            .collect::<Vec<_>>();
        v.dedup();
        v
    }
}
