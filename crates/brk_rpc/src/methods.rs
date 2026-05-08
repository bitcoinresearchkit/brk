use std::{thread::sleep, time::Duration};

use bitcoin::{consensus::encode, hex::FromHex};
use brk_error::{Error, Result};
use brk_types::{
    Bitcoin, BlockHash, FeeRate, Height, MempoolEntryInfo, Sats, Timestamp, Txid, VSize, Vout,
    Weight,
};
use corepc_jsonrpc::error::Error as JsonRpcError;
use corepc_types::{
    v17::{
        GetBlockCount, GetBlockHash, GetBlockHeader, GetBlockHeaderVerbose, GetBlockVerboseOne,
        GetBlockVerboseZero, GetRawMempool, GetTxOut,
    },
    v24::GetMempoolInfo,
};
use rustc_hash::FxHashMap;
use serde::Deserialize;
use serde_json::Value;
use tracing::{debug, info};

/// Bitcoin Core's `-5` (`RPC_INVALID_ADDRESS_OR_KEY`) is the expected
/// response when querying a confirmed transaction without `-txindex`.
/// The mempool fetcher tolerates these per-item failures silently.
const RPC_NOT_FOUND: i32 = -5;

use crate::{BlockHeaderInfo, BlockInfo, BlockTemplateTx, Client, RawTx, TxOutInfo};

/// Per-batch request count for `get_block_hashes_range`. Sized so the
/// JSON request body stays well under a megabyte and bitcoind doesn't
/// spend too long on a single batch before yielding results.
const BATCH_CHUNK: usize = 2000;

/// Live mempool state fetched in one batched bitcoind round-trip:
/// `getrawmempool verbose` + `getblocktemplate` + `getmempoolinfo`.
/// `gbt` is validated to be a subset of `entries` before construction;
/// callers that want strict consistency should rely on this fact.
pub struct MempoolState {
    pub entries: Vec<MempoolEntryInfo>,
    pub gbt: Vec<BlockTemplateTx>,
    pub min_fee: FeeRate,
}

#[derive(Deserialize)]
struct VerboseEntryRaw {
    vsize: VSize,
    weight: Weight,
    time: Timestamp,
    #[serde(rename = "ancestorcount")]
    ancestor_count: u64,
    #[serde(rename = "ancestorsize")]
    ancestor_size: VSize,
    #[serde(rename = "descendantsize")]
    descendant_size: VSize,
    fees: VerboseFeesRaw,
    depends: Vec<String>,
    #[serde(rename = "chunkweight", default)]
    chunk_weight: Option<Weight>,
}

#[derive(Deserialize)]
struct VerboseFeesRaw {
    base: Bitcoin,
    ancestor: Bitcoin,
    descendant: Bitcoin,
    #[serde(default)]
    chunk: Option<Bitcoin>,
}

#[derive(Deserialize)]
struct GbtResponseRaw {
    transactions: Vec<GbtTxRaw>,
}

#[derive(Deserialize)]
struct GbtTxRaw {
    txid: bitcoin::Txid,
    fee: u64,
}

fn build_verbose(raw: FxHashMap<String, VerboseEntryRaw>) -> Result<Vec<MempoolEntryInfo>> {
    raw.into_iter()
        .map(|(txid_str, e)| {
            let depends = e
                .depends
                .iter()
                .map(|s| Client::parse_txid(s, "depends txid"))
                .collect::<Result<Vec<_>>>()?;
            Ok(MempoolEntryInfo {
                txid: Client::parse_txid(&txid_str, "mempool txid")?,
                vsize: e.vsize,
                weight: e.weight,
                fee: Sats::from(e.fees.base),
                first_seen: e.time,
                ancestor_count: e.ancestor_count,
                ancestor_size: e.ancestor_size,
                ancestor_fee: Sats::from(e.fees.ancestor),
                descendant_size: e.descendant_size,
                descendant_fee: Sats::from(e.fees.descendant),
                chunk_fee: e.fees.chunk.map(Sats::from),
                chunk_weight: e.chunk_weight,
                depends,
            })
        })
        .collect()
}

fn build_gbt(raw: GbtResponseRaw) -> Vec<BlockTemplateTx> {
    raw.transactions
        .into_iter()
        .map(|t| BlockTemplateTx {
            txid: Txid::from(t.txid),
            fee: Sats::from(t.fee),
        })
        .collect()
}

fn build_min_fee(raw: GetMempoolInfo) -> FeeRate {
    FeeRate::from(raw.mempool_min_fee * 100_000.0)
}

impl Client {
    /// Returns the numbers of block in the longest chain.
    pub fn get_block_count(&self) -> Result<u64> {
        let r: GetBlockCount = self.0.call_with_retry("getblockcount", &[])?;
        Ok(r.0)
    }

    /// Returns the numbers of block in the longest chain.
    pub fn get_last_height(&self) -> Result<Height> {
        self.get_block_count().map(Height::from)
    }

    pub fn get_block<'a, H>(&self, hash: &'a H) -> Result<bitcoin::Block>
    where
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        let hash: &bitcoin::BlockHash = hash.into();
        let r: GetBlockVerboseZero = self
            .0
            .call_with_retry("getblock", &[serde_json::to_value(hash)?, Value::from(0u8)])?;
        r.block()
            .map_err(|e| Error::Parse(format!("decode getblock: {e}")))
    }

    pub fn get_block_info<'a, H>(&self, hash: &'a H) -> Result<BlockInfo>
    where
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        let hash: &bitcoin::BlockHash = hash.into();
        let r: GetBlockVerboseOne = self
            .0
            .call_with_retry("getblock", &[serde_json::to_value(hash)?, Value::from(1u8)])?;
        Ok(BlockInfo {
            height: r.height as usize,
            confirmations: r.confirmations,
        })
    }

    pub fn get_block_header<'a, H>(&self, hash: &'a H) -> Result<bitcoin::block::Header>
    where
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        let hash: &bitcoin::BlockHash = hash.into();
        let r: GetBlockHeader = self.0.call_with_retry(
            "getblockheader",
            &[serde_json::to_value(hash)?, Value::Bool(false)],
        )?;
        let bytes = Vec::from_hex(&r.0).map_err(|e| Error::Parse(format!("header hex: {e}")))?;
        bitcoin::consensus::deserialize::<bitcoin::block::Header>(&bytes).map_err(Error::from)
    }

    pub fn get_block_header_info<'a, H>(&self, hash: &'a H) -> Result<BlockHeaderInfo>
    where
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        let hash: &bitcoin::BlockHash = hash.into();
        let r: GetBlockHeaderVerbose = self
            .0
            .call_with_retry("getblockheader", &[serde_json::to_value(hash)?])?;
        let previous_block_hash = r
            .previous_block_hash
            .map(|s| Self::parse_block_hash(&s, "previousblockhash"))
            .transpose()?;
        Ok(BlockHeaderInfo {
            height: r.height as usize,
            confirmations: r.confirmations,
            previous_block_hash,
        })
    }

    pub fn get_block_hash<H>(&self, height: H) -> Result<BlockHash>
    where
        H: Into<u64> + Copy,
    {
        let height: u64 = height.into();
        let r: GetBlockHash = self
            .0
            .call_with_retry("getblockhash", &[serde_json::to_value(height)?])?;
        Ok(BlockHash::from(r.block_hash()?))
    }

    /// Get every canonical block hash for the inclusive height range
    /// `start..=end` in a single JSON-RPC batch request. Returns hashes
    /// in canonical order (`start`, `start+1`, …, `end`). Use this
    /// whenever resolving more than ~2 heights — one HTTP round-trip
    /// beats N sequential `get_block_hash` calls once the per-call
    /// overhead dominates.
    pub fn get_block_hashes_range<H1, H2>(&self, start: H1, end: H2) -> Result<Vec<BlockHash>>
    where
        H1: Into<u64>,
        H2: Into<u64>,
    {
        let start: u64 = start.into();
        let end: u64 = end.into();
        if end < start {
            return Ok(Vec::new());
        }
        let total = (end - start + 1) as usize;
        let mut hashes = Vec::with_capacity(total);

        let mut chunk_start = start;
        while chunk_start <= end {
            let chunk_end = (chunk_start + BATCH_CHUNK as u64 - 1).min(end);
            let args = (chunk_start..=chunk_end).map(|h| vec![Value::from(h)]);
            let chunk: Vec<String> = self.0.call_batch("getblockhash", args)?;
            for hex in chunk {
                hashes.push(Self::parse_block_hash(&hex, "getblockhash batch")?);
            }
            chunk_start = chunk_end + 1;
        }
        Ok(hashes)
    }

    pub fn get_tx_out(
        &self,
        txid: &Txid,
        vout: Vout,
        include_mempool: Option<bool>,
    ) -> Result<Option<TxOutInfo>> {
        let txid: &bitcoin::Txid = txid.into();
        let mut args: Vec<Value> = vec![
            serde_json::to_value(txid)?,
            serde_json::to_value(u32::from(vout))?,
        ];
        if let Some(mempool) = include_mempool {
            args.push(Value::Bool(mempool));
        }
        let r: Option<GetTxOut> = self.0.call_with_retry("gettxout", &args)?;
        match r {
            Some(r) => {
                let script_pub_key = bitcoin::ScriptBuf::from_hex(&r.script_pubkey.hex)
                    .map_err(|e| Error::Parse(format!("script hex: {e}")))?;
                Ok(Some(TxOutInfo {
                    coinbase: r.coinbase,
                    value: Sats::from(Bitcoin::from(r.value)),
                    script_pub_key,
                }))
            }
            None => Ok(None),
        }
    }

    pub fn get_raw_mempool(&self) -> Result<Vec<Txid>> {
        let r: GetRawMempool = self.0.call_with_retry("getrawmempool", &[])?;
        r.0.iter()
            .map(|s| Self::parse_txid(s, "mempool txid"))
            .collect()
    }

    pub fn get_raw_transaction<'a, T, H>(
        &self,
        txid: &'a T,
        block_hash: Option<&'a H>,
    ) -> Result<bitcoin::Transaction>
    where
        &'a T: Into<&'a bitcoin::Txid>,
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        let hex = self.get_raw_transaction_hex(txid, block_hash)?;
        let tx = encode::deserialize_hex::<bitcoin::Transaction>(&hex)?;
        Ok(tx)
    }

    pub fn get_raw_transaction_hex<'a, T, H>(
        &self,
        txid: &'a T,
        block_hash: Option<&'a H>,
    ) -> Result<String>
    where
        &'a T: Into<&'a bitcoin::Txid>,
        &'a H: Into<&'a bitcoin::BlockHash>,
    {
        let txid: &bitcoin::Txid = txid.into();
        let mut args: Vec<Value> = vec![serde_json::to_value(txid)?, Value::Bool(false)];
        if let Some(bh) = block_hash {
            let bh: &bitcoin::BlockHash = bh.into();
            args.push(serde_json::to_value(bh)?);
        }
        self.0.call_with_retry("getrawtransaction", &args)
    }

    pub fn get_mempool_raw_tx(&self, txid: &Txid) -> Result<RawTx> {
        let hex = self.get_raw_transaction_hex(txid, None as Option<&BlockHash>)?;
        let tx = encode::deserialize_hex::<bitcoin::Transaction>(&hex)?;
        Ok(RawTx {
            tx,
            hex: hex.into(),
        })
    }

    /// Batched `getrawtransaction` over a slice of txids. Returns a map keyed
    /// by txid containing the deserialized tx and its raw hex. Individual
    /// failures (e.g. a tx that evicted between the listing and this call)
    /// are logged and dropped so a single bad entry doesn't kill the batch.
    ///
    /// Chunked at `BATCH_CHUNK` requests per round-trip.
    pub fn get_raw_transactions(&self, txids: &[Txid]) -> Result<FxHashMap<Txid, RawTx>> {
        let mut out: FxHashMap<Txid, RawTx> =
            FxHashMap::with_capacity_and_hasher(txids.len(), Default::default());

        for chunk in txids.chunks(BATCH_CHUNK) {
            let args = chunk.iter().map(|t| {
                let bt: &bitcoin::Txid = t.into();
                vec![
                    serde_json::to_value(bt).unwrap_or(Value::Null),
                    Value::Bool(false),
                ]
            });
            let results: Vec<Result<String>> =
                self.0.call_batch_per_item("getrawtransaction", args)?;

            for (txid, res) in chunk.iter().zip(results) {
                match res.and_then(|hex| {
                    let tx = encode::deserialize_hex::<bitcoin::Transaction>(&hex)?;
                    Ok::<_, Error>(RawTx {
                        tx,
                        hex: hex.into(),
                    })
                }) {
                    Ok(raw) => {
                        out.insert(*txid, raw);
                    }
                    Err(Error::CorepcRPC(JsonRpcError::Rpc(rpc))) if rpc.code == RPC_NOT_FOUND => {}
                    Err(e) => {
                        debug!(txid = %txid, error = %e, "getrawtransaction batch: item failed")
                    }
                }
            }
        }

        Ok(out)
    }

    pub fn send_raw_transaction(&self, hex: &str) -> Result<Txid> {
        let txid: bitcoin::Txid = self
            .0
            .call_once("sendrawtransaction", &[Value::String(hex.to_string())])
            .map_err(|e| {
                // Bitcoin Core returns RPC error codes for client-side problems
                // (decode failed, verification failed, already in chain, etc.).
                // Surface these as 400 (Parse) so HTTP callers see a 4xx, matching
                // mempool.space's POST /api/tx behavior.
                if let Error::CorepcRPC(JsonRpcError::Rpc(rpc)) = &e
                    && matches!(rpc.code, -22 | -25 | -26 | -27)
                {
                    return Error::Parse(rpc.message.clone());
                }
                e
            })?;
        Ok(Txid::from(txid))
    }

    /// Verbose mempool listing + Core's projected next block + live
    /// `mempoolminfee`, fetched in a single bitcoind round-trip.
    /// Validates that every GBT txid is present in the verbose listing
    /// and returns `Ok(None)` on mismatch so the caller can skip the
    /// cycle (within-batch races inside bitcoind are rare; persistent
    /// drift is bug-shaped). Other failures bubble up as `Err`.
    pub fn fetch_mempool_state(&self) -> Result<Option<MempoolState>> {
        let requests: [(&str, Vec<Value>); 3] = [
            ("getrawmempool", vec![Value::Bool(true)]),
            (
                "getblocktemplate",
                vec![serde_json::json!({ "rules": ["segwit"] })],
            ),
            ("getmempoolinfo", vec![]),
        ];
        let mut out = self.0.call_mixed_batch(&requests)?.into_iter();
        let verbose_raw = out.next().ok_or(Error::Internal("missing verbose"))??;
        let gbt_raw = out.next().ok_or(Error::Internal("missing gbt"))??;
        let info_raw = out.next().ok_or(Error::Internal("missing mempoolinfo"))??;

        let verbose: FxHashMap<String, VerboseEntryRaw> = serde_json::from_str(verbose_raw.get())?;
        let entries = build_verbose(verbose)?;
        let gbt = build_gbt(serde_json::from_str(gbt_raw.get())?);
        let min_fee = build_min_fee(serde_json::from_str(info_raw.get())?);

        #[cfg(debug_assertions)]
        {
            let entry_set: rustc_hash::FxHashSet<Txid> = entries.iter().map(|e| e.txid).collect();
            let missing = gbt.iter().filter(|t| !entry_set.contains(&t.txid)).count();
            if missing > 0 {
                tracing::warn!(
                    missing,
                    gbt_total = gbt.len(),
                    "getblocktemplate has {missing} txids not in verbose mempool; skipping cycle"
                );
                return Ok(None);
            }
        }

        Ok(Some(MempoolState {
            entries,
            gbt,
            min_fee,
        }))
    }

    pub fn get_closest_valid_height(&self, hash: BlockHash) -> Result<(Height, BlockHash)> {
        debug!("Get closest valid height...");

        let mut current = hash;
        loop {
            let info = self.get_block_header_info(&current)?;
            if info.confirmations > 0 {
                return Ok((info.height.into(), current));
            }
            current = info.previous_block_hash.ok_or(Error::NotFound(
                "Reached genesis without finding main chain".into(),
            ))?;
        }
    }

    pub fn wait_for_synced_node(&self) -> Result<()> {
        #[derive(Deserialize)]
        struct SyncProgress {
            headers: u64,
            blocks: u64,
        }
        let is_synced = || -> Result<bool> {
            let p: SyncProgress = self.0.call_with_retry("getblockchaininfo", &[])?;
            Ok(p.headers == p.blocks)
        };

        if !is_synced()? {
            info!("Waiting for node to sync...");
            while !is_synced()? {
                sleep(Duration::from_secs(1))
            }
        }

        Ok(())
    }

    fn parse_txid(s: &str, label: &str) -> Result<Txid> {
        s.parse::<bitcoin::Txid>()
            .map(Txid::from)
            .map_err(|e| Error::Parse(format!("{label}: {e}")))
    }

    fn parse_block_hash(s: &str, label: &str) -> Result<BlockHash> {
        s.parse::<bitcoin::BlockHash>()
            .map(BlockHash::from)
            .map_err(|e| Error::Parse(format!("{label}: {e}")))
    }
}
