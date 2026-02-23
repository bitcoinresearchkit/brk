use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use vecdb::{Formattable, Pco};

use crate::{TxIndex, Vout};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, JsonSchema, Hash, Pco)]
pub struct OutPoint(u64);

impl OutPoint {
    pub const COINBASE: Self = Self(u64::MAX);

    pub fn new(txindex: TxIndex, vout: Vout) -> Self {
        let txindex_bits = u64::from(txindex) << 32;
        let vout_bits = u64::from(vout);
        Self(txindex_bits | vout_bits)
    }

    #[inline(always)]
    pub fn txindex(self) -> TxIndex {
        TxIndex::from((self.0 >> 32) as u32)
    }

    #[inline(always)]
    pub fn vout(self) -> Vout {
        if self.is_coinbase() {
            return Vout::MAX;
        }
        Vout::from(self.0 as u32)
    }

    #[inline(always)]
    pub fn is_coinbase(self) -> bool {
        self == Self::COINBASE
    }

    #[inline(always)]
    pub fn is_not_coinbase(self) -> bool {
        self != Self::COINBASE
    }
}

impl std::fmt::Display for OutPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "txindex: {}, vout: {}", self.txindex(), self.vout())
    }
}

impl Formattable for OutPoint {
    fn fmt_csv(&self, f: &mut String) -> std::fmt::Result {
        use std::fmt::Write;
        let start = f.len();
        write!(f, "{}", self)?;
        if f.as_bytes()[start..].contains(&b',') {
            f.insert(start, '"');
            f.push('"');
        }
        Ok(())
    }
}

impl Serialize for OutPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("OutPoint", 2)?;
        state.serialize_field("txindex", &self.txindex())?;
        state.serialize_field("vout", &self.vout())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for OutPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            txindex: TxIndex,
            vout: Vout,
        }
        let h = Helper::deserialize(deserializer)?;
        Ok(Self::new(h.txindex, h.vout))
    }
}
