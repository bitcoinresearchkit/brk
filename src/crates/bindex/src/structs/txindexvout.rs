use fjall::Slice;

use super::{SliceExtended, Txindex};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Txindexvout {
    pub txindex: Txindex,
    pub vout: u32,
}

const BYTES: usize = size_of::<Txindexvout>();

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

impl From<Txindexvout> for Slice {
    fn from(value: Txindexvout) -> Self {
        let txindex_slice = Self::from(value.txindex);
        let vout_slice = Self::from(value.vout.to_be_bytes());
        Self::from([txindex_slice, vout_slice].concat())
    }
}
impl TryFrom<Slice> for Txindexvout {
    type Error = color_eyre::Report;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        Self::try_from(&value[..])
    }
}
impl TryFrom<&[u8]> for Txindexvout {
    type Error = color_eyre::Report;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let txindex = Txindex::try_from(&value[..BYTES])?;
        let vout = (&value[BYTES..]).read_be_u32()?;
        Ok(Self { txindex, vout })
    }
}
