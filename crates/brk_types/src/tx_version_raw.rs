use derive_more::Deref;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Raw transaction version (i32) from Bitcoin protocol.
/// Unlike TxVersion (u8, indexed), this preserves non-standard values
/// used in coinbase txs for miner signaling/branding.
#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TxVersionRaw(i32);

impl From<bitcoin::transaction::Version> for TxVersionRaw {
    #[inline]
    fn from(value: bitcoin::transaction::Version) -> Self {
        Self(value.0)
    }
}
