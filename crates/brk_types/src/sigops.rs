use bitcoin::{Amount, OutPoint, ScriptBuf, TxOut};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::{Formattable, Pco};

use crate::{OutputType, VSize};

/// BIP-141 sigop cost. The block-level budget is 80,000, so a `u32`
/// fits a single tx's count with room to spare.
///
/// Witness sigops count as 1; legacy and P2SH-redeem sigops count as 4.
/// Five vbytes per sigop is the policy adjustment Core applies in
/// `nSigOpCost` to discourage sigop-heavy txs (`max(weight/4, sigops*5)`).
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Pco,
    JsonSchema,
)]
#[serde(transparent)]
pub struct SigOps(u32);

impl SigOps {
    pub const ZERO: Self = Self(0);

    /// Vbytes per sigop under BIP-141 policy. Core's `nSigOpCost`
    /// adjustment factor: `adjusted_vsize = max(vsize, sigops * 5)`.
    pub const VBYTES_PER_SIGOP: u64 = 5;

    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// BIP-141 vbyte equivalent of this sigop count.
    #[inline]
    pub fn vsize_cost(self) -> VSize {
        VSize::new(u64::from(self.0) * Self::VBYTES_PER_SIGOP)
    }

    /// Policy-adjusted vsize: `max(vsize, sigops * 5)`. The denominator
    /// Core uses when ranking sigop-heavy txs at fixed fee.
    #[inline]
    pub fn adjust_vsize(self, vsize: VSize) -> VSize {
        vsize.max(self.vsize_cost())
    }

    /// BIP-141 sigop cost using only each input's prevout `OutputType` as
    /// hint. Avoids reading the real `script_pubkey`: bitcoin-rs's sigop
    /// walk only inspects script *structure* (`is_p2sh` / `is_p2wpkh` /
    /// `is_p2wsh`), so a canonical empty-hash script of the matching shape
    /// produces the same count as the real one. Other output types
    /// contribute nothing on the input side, so we return `None` for them.
    pub fn of_bitcoin_tx_with_kinds<F>(tx: &bitcoin::Transaction, mut kind_at: F) -> Self
    where
        F: FnMut(&OutPoint) -> Option<OutputType>,
    {
        Self::from(tx.total_sigop_cost(|outpoint| {
            let script_pubkey = match kind_at(outpoint)? {
                OutputType::P2SH => synthetic_p2sh_spk(),
                OutputType::P2WPKH => synthetic_p2wpkh_spk(),
                OutputType::P2WSH => synthetic_p2wsh_spk(),
                _ => return None,
            };
            Some(TxOut {
                value: Amount::ZERO,
                script_pubkey,
            })
        }))
    }
}

fn synthetic_p2sh_spk() -> ScriptBuf {
    // OP_HASH160 PUSH20 <20 zero bytes> OP_EQUAL
    let mut bytes = Vec::with_capacity(23);
    bytes.push(0xa9);
    bytes.push(0x14);
    bytes.extend_from_slice(&[0u8; 20]);
    bytes.push(0x87);
    ScriptBuf::from_bytes(bytes)
}

fn synthetic_p2wpkh_spk() -> ScriptBuf {
    // OP_0 PUSH20 <20 zero bytes>
    let mut bytes = Vec::with_capacity(22);
    bytes.push(0x00);
    bytes.push(0x14);
    bytes.extend_from_slice(&[0u8; 20]);
    ScriptBuf::from_bytes(bytes)
}

fn synthetic_p2wsh_spk() -> ScriptBuf {
    // OP_0 PUSH32 <32 zero bytes>
    let mut bytes = Vec::with_capacity(34);
    bytes.push(0x00);
    bytes.push(0x20);
    bytes.extend_from_slice(&[0u8; 32]);
    ScriptBuf::from_bytes(bytes)
}

impl From<u32> for SigOps {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<usize> for SigOps {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<SigOps> for u32 {
    #[inline]
    fn from(value: SigOps) -> Self {
        value.0
    }
}

impl Formattable for SigOps {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        let mut b = itoa::Buffer::new();
        buf.extend_from_slice(b.format(self.0).as_bytes());
    }
}
