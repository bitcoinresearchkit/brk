use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats, StoredF32, StoredF64, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::internal::{
    ComputedFromHeightLast,
    NegPercentageDollarsF32, PercentageDollarsF32, PercentageSatsF64,
};

use crate::distribution::metrics::{ImportConfig, RealizedBase, UnrealizedBase};

/// Base relative metrics (always computed when relative is enabled).
/// All fields are non-Optional - market_cap and realized_cap are always
/// available when relative metrics are enabled.
#[derive(Traversable)]
pub struct RelativeBase<M: StorageMode = Rw> {
    // === Supply in Profit/Loss Relative to Own Supply ===
    pub supply_in_profit_rel_to_own_supply: ComputedFromHeightLast<StoredF64, M>,
    pub supply_in_loss_rel_to_own_supply: ComputedFromHeightLast<StoredF64, M>,

    // === Unrealized vs Market Cap ===
    pub unrealized_profit_rel_to_market_cap: ComputedFromHeightLast<StoredF32, M>,
    pub unrealized_loss_rel_to_market_cap: ComputedFromHeightLast<StoredF32, M>,
    pub neg_unrealized_loss_rel_to_market_cap: ComputedFromHeightLast<StoredF32, M>,
    pub net_unrealized_pnl_rel_to_market_cap: ComputedFromHeightLast<StoredF32, M>,
    pub nupl: ComputedFromHeightLast<StoredF32, M>,

    // === Invested Capital in Profit/Loss as % of Realized Cap ===
    pub invested_capital_in_profit_pct: ComputedFromHeightLast<StoredF32, M>,
    pub invested_capital_in_loss_pct: ComputedFromHeightLast<StoredF32, M>,
}

impl RelativeBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let v2 = Version::new(2);

        Ok(Self {
            supply_in_profit_rel_to_own_supply: ComputedFromHeightLast::forced_import(
                cfg.db, &cfg.name("supply_in_profit_rel_to_own_supply"), cfg.version + v1, cfg.indexes,
            )?,
            supply_in_loss_rel_to_own_supply: ComputedFromHeightLast::forced_import(
                cfg.db, &cfg.name("supply_in_loss_rel_to_own_supply"), cfg.version + v1, cfg.indexes,
            )?,
            unrealized_profit_rel_to_market_cap: ComputedFromHeightLast::forced_import(
                cfg.db, &cfg.name("unrealized_profit_rel_to_market_cap"), cfg.version + v2, cfg.indexes,
            )?,
            unrealized_loss_rel_to_market_cap: ComputedFromHeightLast::forced_import(
                cfg.db, &cfg.name("unrealized_loss_rel_to_market_cap"), cfg.version + v2, cfg.indexes,
            )?,
            neg_unrealized_loss_rel_to_market_cap: ComputedFromHeightLast::forced_import(
                cfg.db, &cfg.name("neg_unrealized_loss_rel_to_market_cap"), cfg.version + v2, cfg.indexes,
            )?,
            net_unrealized_pnl_rel_to_market_cap: ComputedFromHeightLast::forced_import(
                cfg.db, &cfg.name("net_unrealized_pnl_rel_to_market_cap"), cfg.version + v2, cfg.indexes,
            )?,
            nupl: ComputedFromHeightLast::forced_import(
                cfg.db, &cfg.name("nupl"), cfg.version + v2, cfg.indexes,
            )?,
            invested_capital_in_profit_pct: ComputedFromHeightLast::forced_import(
                cfg.db, &cfg.name("invested_capital_in_profit_pct"), cfg.version, cfg.indexes,
            )?,
            invested_capital_in_loss_pct: ComputedFromHeightLast::forced_import(
                cfg.db, &cfg.name("invested_capital_in_loss_pct"), cfg.version, cfg.indexes,
            )?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        unrealized: &UnrealizedBase,
        realized: &RealizedBase,
        supply_total_sats: &impl ReadableVec<Height, Sats>,
        market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit_rel_to_own_supply
            .compute_binary::<Sats, Sats, PercentageSatsF64>(
                max_from, &unrealized.supply_in_profit.sats.height, supply_total_sats, exit,
            )?;
        self.supply_in_loss_rel_to_own_supply
            .compute_binary::<Sats, Sats, PercentageSatsF64>(
                max_from, &unrealized.supply_in_loss.sats.height, supply_total_sats, exit,
            )?;
        self.unrealized_profit_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                max_from, &unrealized.unrealized_profit.height, market_cap, exit,
            )?;
        self.unrealized_loss_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                max_from, &unrealized.unrealized_loss.height, market_cap, exit,
            )?;
        self.neg_unrealized_loss_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, NegPercentageDollarsF32>(
                max_from, &unrealized.unrealized_loss.height, market_cap, exit,
            )?;
        self.net_unrealized_pnl_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                max_from, &unrealized.net_unrealized_pnl.height, market_cap, exit,
            )?;
        self.nupl
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                max_from, &unrealized.net_unrealized_pnl.height, market_cap, exit,
            )?;
        self.invested_capital_in_profit_pct
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                max_from, &unrealized.invested_capital_in_profit.height, &realized.realized_cap.height, exit,
            )?;
        self.invested_capital_in_loss_pct
            .compute_binary::<Dollars, Dollars, PercentageDollarsF32>(
                max_from, &unrealized.invested_capital_in_loss.height, &realized.realized_cap.height, exit,
            )?;
        Ok(())
    }
}
