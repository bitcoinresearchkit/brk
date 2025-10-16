use allocative::Allocative;
use schemars::JsonSchema;
use serde::Serialize;
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
    Allocative,
    JsonSchema,
)]
pub struct OutPoint {
    txindex: TxIndex,
    vout: Vout,
    _padding: u16,
}

impl OutPoint {
    pub const COINBASE: Self = Self {
        txindex: TxIndex::COINBASE,
        vout: Vout::MAX,
        _padding: 0,
    };

    pub fn new(txindex: TxIndex, vout: Vout) -> Self {
        Self {
            txindex,
            vout,
            _padding: 0,
        }
    }

    pub fn txindex(&self) -> TxIndex {
        self.txindex
    }

    pub fn vout(&self) -> Vout {
        self.vout
    }

    pub fn is_coinbase(self) -> bool {
        self == Self::COINBASE
    }

    pub fn to_be_bytes(&self) -> [u8; 6] {
        let txindex = self.txindex.to_be_bytes();
        let vout = self.vout.to_be_bytes();
        [
            txindex[0], txindex[1], txindex[2], txindex[3], vout[0], vout[1],
        ]
    }
}

impl From<&[u8]> for OutPoint {
    fn from(value: &[u8]) -> Self {
        let txindex = TxIndex::from(&value[4..8]);
        let vout = Vout::from(&value[8..10]);
        Self::new(txindex, vout)
    }
}

impl std::fmt::Display for OutPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "txindex: {}, vout: {}", self.txindex, self.vout)
    }
}
