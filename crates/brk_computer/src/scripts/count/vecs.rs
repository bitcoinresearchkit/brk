use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::PerBlockCumulativeRolling;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub p2a: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    pub p2ms: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    pub p2pk33: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    pub p2pk65: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    pub p2pkh: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    pub p2sh: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    pub p2tr: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    pub p2wpkh: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    pub p2wsh: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    pub op_return: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    pub empty_output: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    pub unknown_output: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
}
