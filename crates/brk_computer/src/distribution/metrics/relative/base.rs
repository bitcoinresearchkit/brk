use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPointsSigned16, Dollars, Height, Sats, StoredF32, Version};
use vecdb::{Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::internal::{
    Bps16ToFloat, LazyFromHeight,
    NegRatioDollarsBps16, PercentFromHeight, RatioDollarsBp16, RatioDollarsBps16, RatioSatsBp16,
};

use crate::distribution::metrics::{ImportConfig, RealizedBase, UnrealizedBase};

/// Base relative metrics (always computed when relative is enabled).
/// All fields are non-Optional - market_cap and realized_cap are always
/// available when relative metrics are enabled.
#[derive(Traversable)]
pub struct RelativeBase<M: StorageMode = Rw> {
    // === Supply in Profit/Loss Relative to Own Supply ===
    pub supply_in_profit_rel_to_own_supply: PercentFromHeight<BasisPoints16, M>,
    pub supply_in_loss_rel_to_own_supply: PercentFromHeight<BasisPoints16, M>,

    // === Unrealized vs Market Cap ===
    pub unrealized_profit_rel_to_market_cap: PercentFromHeight<BasisPoints16, M>,
    pub unrealized_loss_rel_to_market_cap: PercentFromHeight<BasisPoints16, M>,
    pub neg_unrealized_loss_rel_to_market_cap: PercentFromHeight<BasisPointsSigned16, M>,
    pub net_unrealized_pnl_rel_to_market_cap: PercentFromHeight<BasisPointsSigned16, M>,
    pub nupl: LazyFromHeight<StoredF32, BasisPointsSigned16>,

    // === Invested Capital in Profit/Loss as % of Realized Cap ===
    pub invested_capital_in_profit_rel_to_realized_cap: PercentFromHeight<BasisPoints16, M>,
    pub invested_capital_in_loss_rel_to_realized_cap: PercentFromHeight<BasisPoints16, M>,
}

impl RelativeBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let v2 = Version::new(2);

        let net_unrealized_pnl_rel_to_market_cap =
            PercentFromHeight::forced_import_bps16(
                cfg.db, &cfg.name("net_unrealized_pnl_rel_to_market_cap"), cfg.version + v2, cfg.indexes,
            )?;

        let nupl = LazyFromHeight::from_computed::<Bps16ToFloat>(
            &cfg.name("nupl"),
            cfg.version + v2,
            net_unrealized_pnl_rel_to_market_cap.bps.height.read_only_boxed_clone(),
            &net_unrealized_pnl_rel_to_market_cap.bps,
        );

        Ok(Self {
            supply_in_profit_rel_to_own_supply:
                PercentFromHeight::forced_import_bp16(
                    cfg.db, &cfg.name("supply_in_profit_rel_to_own_supply"), cfg.version + v1, cfg.indexes,
                )?,
            supply_in_loss_rel_to_own_supply:
                PercentFromHeight::forced_import_bp16(
                    cfg.db, &cfg.name("supply_in_loss_rel_to_own_supply"), cfg.version + v1, cfg.indexes,
                )?,
            unrealized_profit_rel_to_market_cap:
                PercentFromHeight::forced_import_bp16(
                    cfg.db, &cfg.name("unrealized_profit_rel_to_market_cap"), cfg.version + v2, cfg.indexes,
                )?,
            unrealized_loss_rel_to_market_cap:
                PercentFromHeight::forced_import_bp16(
                    cfg.db, &cfg.name("unrealized_loss_rel_to_market_cap"), cfg.version + v2, cfg.indexes,
                )?,
            neg_unrealized_loss_rel_to_market_cap:
                PercentFromHeight::forced_import_bps16(
                    cfg.db, &cfg.name("neg_unrealized_loss_rel_to_market_cap"), cfg.version + v2, cfg.indexes,
                )?,
            net_unrealized_pnl_rel_to_market_cap,
            nupl,
            invested_capital_in_profit_rel_to_realized_cap:
                PercentFromHeight::forced_import_bp16(
                    cfg.db, &cfg.name("invested_capital_in_profit_rel_to_realized_cap"), cfg.version, cfg.indexes,
                )?,
            invested_capital_in_loss_rel_to_realized_cap:
                PercentFromHeight::forced_import_bp16(
                    cfg.db, &cfg.name("invested_capital_in_loss_rel_to_realized_cap"), cfg.version, cfg.indexes,
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
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from, &unrealized.supply_in_profit.sats.height, supply_total_sats, exit,
            )?;
        self.supply_in_loss_rel_to_own_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from, &unrealized.supply_in_loss.sats.height, supply_total_sats, exit,
            )?;
        self.unrealized_profit_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from, &unrealized.unrealized_profit.usd.height, market_cap, exit,
            )?;
        self.unrealized_loss_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from, &unrealized.unrealized_loss.usd.height, market_cap, exit,
            )?;
        self.neg_unrealized_loss_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, NegRatioDollarsBps16>(
                max_from, &unrealized.unrealized_loss.usd.height, market_cap, exit,
            )?;
        self.net_unrealized_pnl_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBps16>(
                max_from, &unrealized.net_unrealized_pnl.usd.height, market_cap, exit,
            )?;
        self.invested_capital_in_profit_rel_to_realized_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from, &unrealized.invested_capital_in_profit.usd.height, &realized.realized_cap.height, exit,
            )?;
        self.invested_capital_in_loss_rel_to_realized_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from, &unrealized.invested_capital_in_loss.usd.height, &realized.realized_cap.height, exit,
            )?;
        Ok(())
    }
}
