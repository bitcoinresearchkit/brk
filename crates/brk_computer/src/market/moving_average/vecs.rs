use brk_traversable::Traversable;
use brk_types::Cents;
use vecdb::{Rw, StorageMode};

use crate::internal::{PriceWithRatioPerBlock, LazyPerBlock, Price};
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_sma_1w: PriceWithRatioPerBlock<M>,
    pub price_sma_8d: PriceWithRatioPerBlock<M>,
    pub price_sma_13d: PriceWithRatioPerBlock<M>,
    pub price_sma_21d: PriceWithRatioPerBlock<M>,
    pub price_sma_1m: PriceWithRatioPerBlock<M>,
    pub price_sma_34d: PriceWithRatioPerBlock<M>,
    pub price_sma_55d: PriceWithRatioPerBlock<M>,
    pub price_sma_89d: PriceWithRatioPerBlock<M>,
    pub price_sma_111d: PriceWithRatioPerBlock<M>,
    pub price_sma_144d: PriceWithRatioPerBlock<M>,
    pub price_sma_200d: PriceWithRatioPerBlock<M>,
    pub price_sma_350d: PriceWithRatioPerBlock<M>,
    pub price_sma_1y: PriceWithRatioPerBlock<M>,
    pub price_sma_2y: PriceWithRatioPerBlock<M>,
    pub price_sma_200w: PriceWithRatioPerBlock<M>,
    pub price_sma_4y: PriceWithRatioPerBlock<M>,

    pub price_ema_1w: PriceWithRatioPerBlock<M>,
    pub price_ema_8d: PriceWithRatioPerBlock<M>,
    pub price_ema_12d: PriceWithRatioPerBlock<M>,
    pub price_ema_13d: PriceWithRatioPerBlock<M>,
    pub price_ema_21d: PriceWithRatioPerBlock<M>,
    pub price_ema_26d: PriceWithRatioPerBlock<M>,
    pub price_ema_1m: PriceWithRatioPerBlock<M>,
    pub price_ema_34d: PriceWithRatioPerBlock<M>,
    pub price_ema_55d: PriceWithRatioPerBlock<M>,
    pub price_ema_89d: PriceWithRatioPerBlock<M>,
    pub price_ema_144d: PriceWithRatioPerBlock<M>,
    pub price_ema_200d: PriceWithRatioPerBlock<M>,
    pub price_ema_1y: PriceWithRatioPerBlock<M>,
    pub price_ema_2y: PriceWithRatioPerBlock<M>,
    pub price_ema_200w: PriceWithRatioPerBlock<M>,
    pub price_ema_4y: PriceWithRatioPerBlock<M>,

    pub price_sma_200d_x2_4: Price<LazyPerBlock<Cents, Cents>>,
    pub price_sma_200d_x0_8: Price<LazyPerBlock<Cents, Cents>>,
    pub price_sma_350d_x2: Price<LazyPerBlock<Cents, Cents>>,
}
