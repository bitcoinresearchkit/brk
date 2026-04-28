use brk_types::{FeeRate, Sats, VSize};
use smallvec::SmallVec;

use super::LocalIdx;

pub(crate) struct Chunk {
    pub(crate) nodes: SmallVec<[LocalIdx; 4]>,
    pub(crate) fee: Sats,
    pub(crate) vsize: VSize,
}

impl Chunk {
    pub(super) fn from_mask(mask: u128, fee: Sats, vsize: VSize) -> Self {
        let mut nodes: SmallVec<[LocalIdx; 4]> = SmallVec::new();
        let mut bits = mask;
        while bits != 0 {
            nodes.push(bits.trailing_zeros() as LocalIdx);
            bits &= bits - 1;
        }
        Self { nodes, fee, vsize }
    }

    pub(crate) fn fee_rate(&self) -> FeeRate {
        FeeRate::from((self.fee, self.vsize))
    }
}
