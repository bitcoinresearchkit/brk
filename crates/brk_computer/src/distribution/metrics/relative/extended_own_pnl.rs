use brk_traversable::Traversable;
use brk_types::{Dollars, StoredF32, Version};

use crate::internal::{
    LazyBinaryFromHeightLast, NegPercentageDollarsF32, PercentageDollarsF32,
};

use crate::distribution::metrics::{ImportConfig, UnrealizedBase};

/// Extended relative metrics for own total unrealized PnL (extended only).
#[derive(Clone, Traversable)]
pub struct RelativeExtendedOwnPnl {
    pub unrealized_profit_rel_to_own_total_unrealized_pnl:
        LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub unrealized_loss_rel_to_own_total_unrealized_pnl:
        LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub neg_unrealized_loss_rel_to_own_total_unrealized_pnl:
        LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
    pub net_unrealized_pnl_rel_to_own_total_unrealized_pnl:
        LazyBinaryFromHeightLast<StoredF32, Dollars, Dollars>,
}

impl RelativeExtendedOwnPnl {
    pub(crate) fn forced_import(
        cfg: &ImportConfig,
        unrealized: &UnrealizedBase,
    ) -> Self {
        let v1 = Version::ONE;
        let v2 = Version::new(2);

        Self {
            unrealized_profit_rel_to_own_total_unrealized_pnl:
                LazyBinaryFromHeightLast::from_block_last_and_binary_block::<PercentageDollarsF32, _, _>(
                    &cfg.name("unrealized_profit_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    &unrealized.unrealized_profit,
                    &unrealized.total_unrealized_pnl,
                ),
            unrealized_loss_rel_to_own_total_unrealized_pnl:
                LazyBinaryFromHeightLast::from_block_last_and_binary_block::<PercentageDollarsF32, _, _>(
                    &cfg.name("unrealized_loss_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    &unrealized.unrealized_loss,
                    &unrealized.total_unrealized_pnl,
                ),
            neg_unrealized_loss_rel_to_own_total_unrealized_pnl:
                LazyBinaryFromHeightLast::from_block_last_and_binary_block::<NegPercentageDollarsF32, _, _>(
                    &cfg.name("neg_unrealized_loss_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v1,
                    &unrealized.unrealized_loss,
                    &unrealized.total_unrealized_pnl,
                ),
            net_unrealized_pnl_rel_to_own_total_unrealized_pnl:
                LazyBinaryFromHeightLast::from_both_binary_block::<PercentageDollarsF32, _, _, _, _>(
                    &cfg.name("net_unrealized_pnl_rel_to_own_total_unrealized_pnl"),
                    cfg.version + v2,
                    &unrealized.net_unrealized_pnl,
                    &unrealized.total_unrealized_pnl,
                ),
        }
    }
}
