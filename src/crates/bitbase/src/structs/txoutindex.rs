use fjall::Slice;

use super::{SliceExtended, Txindex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Txoutindex {
    pub txindex: Txindex,
    pub vout: u16,
}

impl Txoutindex {
    pub const BYTES: usize = size_of::<Self>();
}

impl From<Txindex> for Txoutindex {
    fn from(value: Txindex) -> Self {
        Self {
            txindex: value,
            vout: 0,
        }
    }
}

impl From<(Txindex, u16)> for Txoutindex {
    fn from(value: (Txindex, u16)) -> Self {
        Self {
            txindex: value.0,
            vout: value.1,
        }
    }
}

impl From<Txoutindex> for Slice {
    fn from(value: Txoutindex) -> Self {
        let txindex_slice = Self::from(value.txindex);
        let vout_slice = Self::from(value.vout.to_be_bytes());
        Self::from([txindex_slice, vout_slice].concat())
    }
}
impl From<Slice> for Txoutindex {
    fn from(value: Slice) -> Self {
        let txindex = Txindex::from(Slice::from(&value[..Txindex::BYTES]));
        let vout = Slice::from(&value[Txindex::BYTES..]).read_u16();
        Self { txindex, vout }
    }
}
