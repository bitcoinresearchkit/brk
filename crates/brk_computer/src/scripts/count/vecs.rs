use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedPerBlockCumulativeSum;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub p2a: ComputedPerBlockCumulativeSum<StoredU64, M>,
    pub p2ms: ComputedPerBlockCumulativeSum<StoredU64, M>,
    pub p2pk33: ComputedPerBlockCumulativeSum<StoredU64, M>,
    pub p2pk65: ComputedPerBlockCumulativeSum<StoredU64, M>,
    pub p2pkh: ComputedPerBlockCumulativeSum<StoredU64, M>,
    pub p2sh: ComputedPerBlockCumulativeSum<StoredU64, M>,
    pub p2tr: ComputedPerBlockCumulativeSum<StoredU64, M>,
    pub p2wpkh: ComputedPerBlockCumulativeSum<StoredU64, M>,
    pub p2wsh: ComputedPerBlockCumulativeSum<StoredU64, M>,
    pub opreturn: ComputedPerBlockCumulativeSum<StoredU64, M>,
    pub emptyoutput: ComputedPerBlockCumulativeSum<StoredU64, M>,
    pub unknownoutput: ComputedPerBlockCumulativeSum<StoredU64, M>,

    pub segwit: ComputedPerBlockCumulativeSum<StoredU64, M>,
}
