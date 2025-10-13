use crate::{RawLockTime, Sats, TxIndex, TxInput, TxOutput, TxStatus, TxVersion, Txid};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize, JsonSchema)]
/// Transaction information compatible with mempool.space API format
pub struct Tx {
    #[schemars(example = "9a0b3b8305bb30cacf9e8443a90d53a76379fb3305047fdeaa4e4b0934a2a1ba")]
    pub txid: Txid,

    #[schemars(example = TxIndex::new(0))]
    pub index: TxIndex,

    #[schemars(example = 2)]
    pub version: TxVersion,

    #[schemars(example = 0)]
    pub locktime: RawLockTime,

    /// Transaction size in bytes
    #[schemars(example = 222)]
    pub size: u32,

    /// Transaction weight (for SegWit transactions)
    #[schemars(example = 558)]
    pub weight: u32,

    /// Number of signature operations
    #[schemars(example = 1)]
    pub sigops: u32,

    /// Transaction fee in satoshis
    #[schemars(example = Sats::new(31))]
    pub fee: Sats,

    /// Transaction inputs
    pub vin: Vec<TxInput>,

    /// Transaction outputs
    pub vout: Vec<TxOutput>,

    pub status: TxStatus,
}
