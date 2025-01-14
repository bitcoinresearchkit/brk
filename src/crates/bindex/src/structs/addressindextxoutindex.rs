use snkrj::{direct_repr, Storable, UnsizedStorable};

use super::{Addressindex, Txoutindex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Addressindextxoutindex {
    addressindex: Addressindex,
    txoutindex: Txoutindex,
}
direct_repr!(Addressindextxoutindex);

impl From<(Addressindex, Txoutindex)> for Addressindextxoutindex {
    fn from(value: (Addressindex, Txoutindex)) -> Self {
        Self {
            addressindex: value.0,
            txoutindex: value.1,
        }
    }
}
