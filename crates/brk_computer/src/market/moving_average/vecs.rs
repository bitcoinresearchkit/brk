use brk_traversable::Traversable;
use brk_types::Cents;
use vecdb::{Rw, StorageMode};

use crate::internal::{LazyPerBlock, Price, PriceWithRatioPerBlock};

#[derive(Traversable)]
pub struct SmaVecs<M: StorageMode = Rw> {
    pub _1w: PriceWithRatioPerBlock<M>,
    pub _8d: PriceWithRatioPerBlock<M>,
    pub _13d: PriceWithRatioPerBlock<M>,
    pub _21d: PriceWithRatioPerBlock<M>,
    pub _1m: PriceWithRatioPerBlock<M>,
    pub _34d: PriceWithRatioPerBlock<M>,
    pub _55d: PriceWithRatioPerBlock<M>,
    pub _89d: PriceWithRatioPerBlock<M>,
    pub _111d: PriceWithRatioPerBlock<M>,
    pub _144d: PriceWithRatioPerBlock<M>,
    pub _200d: PriceWithRatioPerBlock<M>,
    pub _350d: PriceWithRatioPerBlock<M>,
    pub _1y: PriceWithRatioPerBlock<M>,
    pub _2y: PriceWithRatioPerBlock<M>,
    pub _200w: PriceWithRatioPerBlock<M>,
    pub _4y: PriceWithRatioPerBlock<M>,
    #[traversable(wrap = "200d", rename = "x2_4")]
    pub _200d_x2_4: Price<LazyPerBlock<Cents, Cents>>,
    #[traversable(wrap = "200d", rename = "x0_8")]
    pub _200d_x0_8: Price<LazyPerBlock<Cents, Cents>>,
    #[traversable(wrap = "350d", rename = "x2")]
    pub _350d_x2: Price<LazyPerBlock<Cents, Cents>>,
}

#[derive(Traversable)]
pub struct EmaVecs<M: StorageMode = Rw> {
    pub _1w: PriceWithRatioPerBlock<M>,
    pub _8d: PriceWithRatioPerBlock<M>,
    pub _12d: PriceWithRatioPerBlock<M>,
    pub _13d: PriceWithRatioPerBlock<M>,
    pub _21d: PriceWithRatioPerBlock<M>,
    pub _26d: PriceWithRatioPerBlock<M>,
    pub _1m: PriceWithRatioPerBlock<M>,
    pub _34d: PriceWithRatioPerBlock<M>,
    pub _55d: PriceWithRatioPerBlock<M>,
    pub _89d: PriceWithRatioPerBlock<M>,
    pub _144d: PriceWithRatioPerBlock<M>,
    pub _200d: PriceWithRatioPerBlock<M>,
    pub _1y: PriceWithRatioPerBlock<M>,
    pub _2y: PriceWithRatioPerBlock<M>,
    pub _200w: PriceWithRatioPerBlock<M>,
    pub _4y: PriceWithRatioPerBlock<M>,
}

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub sma: SmaVecs<M>,
    pub ema: EmaVecs<M>,
}
