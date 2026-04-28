use brk_types::TxidPrefix;

use super::{TxAddition, TxRemoval};

pub struct TxsPulled {
    pub added: Vec<TxAddition>,
    pub removed: Vec<(TxidPrefix, TxRemoval)>,
}
