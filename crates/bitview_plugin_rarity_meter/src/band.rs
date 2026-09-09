use bitview_traversable::Traversable;
use brk_types::{Cents, PartsPerMillion32};

use bitview_vecs::{LazyPerBlock, LazyRatioPerBlock, Price};

#[derive(Clone, Traversable)]
pub struct Band {
    #[traversable(flatten)]
    pub ratio: LazyRatioPerBlock<PartsPerMillion32>,
    pub price: Price<LazyPerBlock<Cents>>,
}
