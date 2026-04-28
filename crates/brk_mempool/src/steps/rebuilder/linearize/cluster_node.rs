use brk_types::{Sats, VSize};
use smallvec::SmallVec;

use crate::stores::TxIndex;

use super::LocalIdx;

pub(crate) struct ClusterNode {
    pub(crate) tx_index: TxIndex,
    pub(crate) fee: Sats,
    pub(crate) vsize: VSize,
    pub(crate) parents: SmallVec<[LocalIdx; 2]>,
    pub(crate) children: SmallVec<[LocalIdx; 2]>,
}
