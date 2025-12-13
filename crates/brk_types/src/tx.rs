use crate::{FeeRate, RawLockTime, Sats, TxIn, TxIndex, TxOut, TxStatus, TxVersion, Txid, VSize, Weight};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::CheckedSub;

#[derive(Debug, Clone, Serialize, JsonSchema)]
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
    pub fn fee(tx: &Transaction) -> Option<Sats> {
        let in_ = tx
            .input
            .iter()
            .map(|txin| txin.prevout.as_ref().map(|txout| txout.value))
            .sum::<Option<Sats>>()?;
        let out = tx.output.iter().map(|txout| txout.value).sum::<Sats>();
        Some(in_.checked_sub(out).unwrap())
    }

    pub fn compute_fee(&mut self) {
        self.fee = Self::fee(self).unwrap_or_default();
    }

    /// Virtual size in vbytes (weight / 4, rounded up)
    #[inline]
    pub fn vsize(&self) -> VSize {
        VSize::from(self.weight)
    }

    /// Fee rate in sat/vB
    #[inline]
    pub fn fee_rate(&self) -> FeeRate {
        FeeRate::from((self.fee, self.vsize()))
    }
}
