use crate::{RawLockTime, Sats, TxIn, TxIndex, TxOut, TxStatus, TxVersion, Txid, Weight};
use bitcoincore_rpc::Client;
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::CheckedSub;

#[derive(Debug, Serialize, JsonSchema)]
/// Transaction information compatible with mempool.space API format
pub struct Transaction {
    #[schemars(example = TxIndex::new(0))]
    pub index: Option<TxIndex>,

    #[schemars(example = "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b")]
    pub txid: Txid,

    #[schemars(example = 2)]
    pub version: TxVersion,

    #[schemars(example = 0)]
    #[serde(rename = "locktime")]
    pub lock_time: RawLockTime,

    /// Transaction size in bytes
    #[schemars(example = 222)]
    #[serde(rename = "size")]
    pub total_size: usize,

    /// Transaction weight
    #[schemars(example = 558)]
    pub weight: Weight,

    /// Number of signature operations
    #[schemars(example = 1)]
    #[serde(rename = "sigops")]
    pub total_sigop_cost: usize,

    /// Transaction fee in satoshis
    #[schemars(example = Sats::new(31))]
    pub fee: Sats,

    /// Transaction inputs
    #[serde(rename = "vin")]
    pub input: Vec<TxIn>,

    /// Transaction outputs
    #[serde(rename = "vout")]
    pub output: Vec<TxOut>,

    pub status: TxStatus,
}

impl Transaction {
    pub fn fee(&self) -> Option<Sats> {
        let in_ = self
            .input
            .iter()
            .map(|txin| txin.prevout.as_ref().map(|txout| txout.value))
            .sum::<Option<Sats>>()?;
        let out = self.output.iter().map(|txout| txout.value).sum::<Sats>();
        Some(in_.checked_sub(out).unwrap())
    }
}

impl Transaction {
    pub fn from_mempool(tx: bitcoin::Transaction, rpc: &Client) -> Self {
        let mut this = Self {
            index: None,
            txid: tx.compute_txid().into(),
            version: tx.version.into(),
            total_sigop_cost: tx.total_sigop_cost(|_| None),
            weight: tx.weight().into(),
            lock_time: tx.lock_time.into(),
            total_size: tx.total_size(),
            fee: Sats::default(),
            input: tx
                .input
                .into_iter()
                .map(|txin| TxIn::from((txin, rpc)))
                .collect(),
            output: tx.output.into_iter().map(TxOut::from).collect(),
            status: TxStatus::UNCOMFIRMED,
        };
        this.fee = this.fee().unwrap_or_default();
        this
    }
}
