use fjall::Slice;

use super::{Addressindex, Txoutindex};

#[derive(Debug)]
pub struct Addressindextxoutindex {
    addressindex: Addressindex,
    txoutindex: Txoutindex,
}

impl From<(Addressindex, Txoutindex)> for Addressindextxoutindex {
    fn from(value: (Addressindex, Txoutindex)) -> Self {
        Self {
            addressindex: value.0,
            txoutindex: value.1,
        }
    }
}

impl From<Addressindextxoutindex> for Slice {
    fn from(value: Addressindextxoutindex) -> Self {
        let addressindex_slice = Self::from(value.addressindex);
        let txindexvout_slice = Self::from(value.txoutindex);
        Self::from([addressindex_slice, txindexvout_slice].concat())
    }
}
impl TryFrom<Slice> for Addressindextxoutindex {
    type Error = color_eyre::Report;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        let addressindex = Addressindex::try_from(&value[..Addressindex::BYTES])?;
        let txindexvout = Txoutindex::try_from(&value[Addressindex::BYTES..])?;

        Ok(Self {
            addressindex,
            txoutindex: txindexvout,
        })
    }
}
