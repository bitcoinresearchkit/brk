use brk_traversable::Traversable;
use brk_types::Cents;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightPriceWithRatio, LazyFromHeight, Price};
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price_sma_1w: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_8d: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_13d: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_21d: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_1m: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_34d: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_55d: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_89d: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_111d: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_144d: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_200d: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_350d: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_1y: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_2y: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_200w: ComputedFromHeightPriceWithRatio<M>,
    pub price_sma_4y: ComputedFromHeightPriceWithRatio<M>,

    pub price_ema_1w: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_8d: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_12d: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_13d: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_21d: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_26d: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_1m: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_34d: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_55d: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_89d: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_144d: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_200d: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_1y: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_2y: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_200w: ComputedFromHeightPriceWithRatio<M>,
    pub price_ema_4y: ComputedFromHeightPriceWithRatio<M>,

    pub price_sma_200d_x2_4: Price<LazyFromHeight<Cents, Cents>>,
    pub price_sma_200d_x0_8: Price<LazyFromHeight<Cents, Cents>>,
    pub price_sma_350d_x2: Price<LazyFromHeight<Cents, Cents>>,
}
