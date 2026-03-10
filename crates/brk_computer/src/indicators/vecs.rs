use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPoints32};
use vecdb::{Database, Rw, StorageMode};

use crate::internal::{PercentPerBlock, RatioPerBlock};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub(crate) db: Database,
    pub puell_multiple: RatioPerBlock<BasisPoints32, M>,
    pub nvt: RatioPerBlock<BasisPoints32, M>,
    pub gini: PercentPerBlock<BasisPoints16, M>,
    pub rhodl_ratio: RatioPerBlock<BasisPoints32, M>,
}
