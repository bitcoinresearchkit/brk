use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{Compressable, Formattable};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{TxIndex, Vout};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Default,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
    JsonSchema,
    Hash,
    Compressable,
)]
pub struct OutPoint(u64);

impl OutPoint {
    pub const COINBASE: Self = Self(u64::MAX);

    pub fn new(txindex: TxIndex, vout: Vout) -> Self {
        let txindex_bits = u64::from(txindex) << 32;
        let vout_bits = u64::from(vout);
        Self(txindex_bits | vout_bits)
    }

    pub fn txindex(self) -> TxIndex {
        TxIndex::from((self.0 >> 32) as u32)
    }

    pub fn vout(self) -> Vout {
        Vout::from(self.0 as u32)
    }

    pub fn is_coinbase(self) -> bool {
        self == Self::COINBASE
    }
}

impl std::fmt::Display for OutPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "txindex: {}, vout: {}", self.txindex(), self.vout())
    }
}

impl Formattable for OutPoint {
    #[inline(always)]
    fn may_need_escaping() -> bool {
        true
    }
}
