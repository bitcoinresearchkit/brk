use fjall::Slice;

use super::{Addressindex, Txoutindex};

pub struct Addresstxoutindex {
    addressindex: Addressindex,
    txoutindex: Txoutindex,
}

impl From<(Addressindex, Txoutindex)> for Addresstxoutindex {
    fn from(value: (Addressindex, Txoutindex)) -> Self {
        Self {
            addressindex: value.0,
            txoutindex: value.1,
        }
    }
}

impl From<Addresstxoutindex> for Slice {
    fn from(value: Addresstxoutindex) -> Self {
        let addressindex_slice = Self::from(value.addressindex);
        let txoutindex_slice = Self::from(value.txoutindex);
        Self::from([addressindex_slice, txoutindex_slice].concat())
    }
}
impl From<Slice> for Addresstxoutindex {
    fn from(value: Slice) -> Self {
        let addressindex = Addressindex::from(Slice::from(&value[..Addressindex::BYTES]));
        let txoutindex = Txoutindex::from(Slice::from(&value[Addressindex::BYTES..]));
        Self {
            addressindex,
            txoutindex,
        }
    }
}
