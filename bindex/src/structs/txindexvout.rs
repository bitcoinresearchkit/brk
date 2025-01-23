use snkrj::{direct_repr, Storable, UnsizedStorable};

use super::Txindex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Txindexvout {
    pub txindex: Txindex,
    pub vout: u32,
}
direct_repr!(Txindexvout);

impl From<Txindex> for Txindexvout {
    fn from(value: Txindex) -> Self {
        Self {
            txindex: value,
            vout: 0,
        }
    }
}

impl From<(Txindex, u32)> for Txindexvout {
    fn from(value: (Txindex, u32)) -> Self {
        Self {
            txindex: value.0,
            vout: value.1,
        }
    }
}
