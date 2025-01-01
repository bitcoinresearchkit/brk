use super::{SliceExtended, Txindex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Txoutindex {
    pub txindex: Txindex,
    pub vout: u16,
}

const SHIFT: u64 = 16;
const AND: u64 = (1 << SHIFT) - 1;

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

impl From<u64> for Txoutindex {
    fn from(value: u64) -> Self {
        Self {
            txindex: (value >> SHIFT).into(),
            vout: (value & AND) as u16,
        }
    }
}
impl From<Txoutindex> for u64 {
    fn from(value: Txoutindex) -> Self {
        (u64::from(value.txindex) << SHIFT) + value.vout as u64
    }
}

impl From<Txoutindex> for fjall::Slice {
    fn from(value: Txoutindex) -> Self {
        u64::from(value).to_be_bytes().into()
    }
}
impl From<fjall::Slice> for Txoutindex {
    fn from(value: fjall::Slice) -> Self {
        fjall::Slice::read_u64(&value).into()
    }
}
