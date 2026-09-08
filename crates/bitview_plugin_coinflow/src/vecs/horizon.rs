use bitview_traversable::Traversable;
use brk_types::{BoundedRatio, StoredF64};

use bitview_vecs::LazyPerBlock;

#[derive(Clone, Traversable)]
pub struct HorizonVecs {
    #[traversable(wrap = "supply/in_loss", rename = "share")]
    pub supply_in_loss_share: LazyPerBlock<StoredF64, BoundedRatio>,
}
