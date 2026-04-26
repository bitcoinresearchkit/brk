use brk_types::TxidPrefix;
use rustc_hash::FxHashMap;

use super::{Addition, Removal};

/// Output of one pull cycle: the full diff, ready for the Applier.
pub struct Pulled {
    pub added: Vec<Addition>,
    pub removed: FxHashMap<TxidPrefix, Removal>,
}
