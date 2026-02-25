use brk_traversable::Traversable;
use brk_types::{Dollars, Sats, StoredF32, Version};

use crate::internal::{
    LazyBinaryComputedFromHeightLast, LazyBinaryFromHeightLast,
    NegPercentageDollarsF32, PercentageDollarsF32,
};

use crate::distribution::metrics::{ImportConfig, UnrealizedBase};

/// Extended relative metrics for own market cap (extended && rel_to_all).
#[derive(Clone, Traversable)]
pub struct RelativeExtendedOwnMarketCap {
    pub unrealized_profit_rel_to_own_market_cap:
        LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub unrealized_loss_rel_to_own_market_cap:
        LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub neg_unrealized_loss_rel_to_own_market_cap:
        LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub net_unrealized_pnl_rel_to_own_market_cap:
        LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
}

impl RelativeExtendedOwnMarketCap {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
        unrealized: &UnrealizedBase,
        own_market_cap: &LazyBinaryComputedFromHeightLast<Dollars, Sats, Dollars>,
    ) -> Self {
        let v2 = Version::new(2);

        Self {
            unrealized_profit_rel_to_own_market_cap:
                LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                    PercentageDollarsF32, _, _,
                >(
                    &cfg.name("unrealized_profit_rel_to_own_market_cap"),
                    cfg.version + v2,
                    &unrealized.unrealized_profit,
                    own_market_cap,
                ),
            unrealized_loss_rel_to_own_market_cap:
                LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                    PercentageDollarsF32, _, _,
                >(
                    &cfg.name("unrealized_loss_rel_to_own_market_cap"),
                    cfg.version + v2,
                    &unrealized.unrealized_loss,
                    own_market_cap,
                ),
            neg_unrealized_loss_rel_to_own_market_cap:
                LazyBinaryFromHeightLast::from_block_last_and_lazy_binary_computed_block_last::<
                    NegPercentageDollarsF32, _, _,
                >(
                    &cfg.name("neg_unrealized_loss_rel_to_own_market_cap"),
                    cfg.version + v2,
                    &unrealized.unrealized_loss,
                    own_market_cap,
                ),
            net_unrealized_pnl_rel_to_own_market_cap:
                LazyBinaryFromHeightLast::from_binary_block_and_lazy_binary_block_last::<
                    PercentageDollarsF32, _, _, _, _,
                >(
                    &cfg.name("net_unrealized_pnl_rel_to_own_market_cap"),
                    cfg.version + v2,
                    &unrealized.net_unrealized_pnl,
                    own_market_cap,
                ),
        }
    }
}
