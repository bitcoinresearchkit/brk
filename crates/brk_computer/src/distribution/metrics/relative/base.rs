use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPointsSigned16, Dollars, Height, Sats, StoredF32, Version};
use vecdb::{Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::internal::{
    Bps16ToFloat, LazyFromHeight, NegRatioDollarsBps16, PercentFromHeight, RatioDollarsBp16,
    RatioDollarsBps16, RatioSatsBp16,
};

use crate::distribution::metrics::{ImportConfig, RealizedBase, UnrealizedBase};

#[derive(Traversable)]
pub struct RelativeBase<M: StorageMode = Rw> {
    pub supply_in_profit_rel_to_own_supply: PercentFromHeight<BasisPoints16, M>,
    pub supply_in_loss_rel_to_own_supply: PercentFromHeight<BasisPoints16, M>,

    pub unrealized_profit_rel_to_market_cap: PercentFromHeight<BasisPoints16, M>,
    pub unrealized_loss_rel_to_market_cap: PercentFromHeight<BasisPoints16, M>,
    pub neg_unrealized_loss_rel_to_market_cap: PercentFromHeight<BasisPointsSigned16, M>,
    pub net_unrealized_pnl_rel_to_market_cap: PercentFromHeight<BasisPointsSigned16, M>,
    pub nupl: LazyFromHeight<StoredF32, BasisPointsSigned16>,

    pub invested_capital_in_profit_rel_to_realized_cap: PercentFromHeight<BasisPoints16, M>,
    pub invested_capital_in_loss_rel_to_realized_cap: PercentFromHeight<BasisPoints16, M>,
}

impl RelativeBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let v2 = Version::new(2);

        let net_unrealized_pnl_rel_to_market_cap =
            cfg.import_percent_bps16("net_unrealized_pnl_rel_to_market_cap", v2)?;

        let nupl = LazyFromHeight::from_computed::<Bps16ToFloat>(
            &cfg.name("nupl"),
            cfg.version + v2,
            net_unrealized_pnl_rel_to_market_cap
                .bps
                .height
                .read_only_boxed_clone(),
            &net_unrealized_pnl_rel_to_market_cap.bps,
        );

        Ok(Self {
            supply_in_profit_rel_to_own_supply: cfg
                .import_percent_bp16("supply_in_profit_rel_to_own_supply", v1)?,
            supply_in_loss_rel_to_own_supply: cfg
                .import_percent_bp16("supply_in_loss_rel_to_own_supply", v1)?,
            unrealized_profit_rel_to_market_cap: cfg
                .import_percent_bp16("unrealized_profit_rel_to_market_cap", v2)?,
            unrealized_loss_rel_to_market_cap: cfg
                .import_percent_bp16("unrealized_loss_rel_to_market_cap", v2)?,
            neg_unrealized_loss_rel_to_market_cap: cfg
                .import_percent_bps16("neg_unrealized_loss_rel_to_market_cap", v2)?,
            net_unrealized_pnl_rel_to_market_cap,
            nupl,
            invested_capital_in_profit_rel_to_realized_cap: cfg.import_percent_bp16(
                "invested_capital_in_profit_rel_to_realized_cap",
                Version::ZERO,
            )?,
            invested_capital_in_loss_rel_to_realized_cap: cfg.import_percent_bp16(
                "invested_capital_in_loss_rel_to_realized_cap",
                Version::ZERO,
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
                max_from,
                &unrealized.supply_in_profit.sats.height,
                supply_total_sats,
                exit,
            )?;
        self.supply_in_loss_rel_to_own_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &unrealized.supply_in_loss.sats.height,
                supply_total_sats,
                exit,
            )?;
        self.unrealized_profit_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.unrealized_profit.usd.height,
                market_cap,
                exit,
            )?;
        self.unrealized_loss_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.unrealized_loss.usd.height,
                market_cap,
                exit,
            )?;
        self.neg_unrealized_loss_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, NegRatioDollarsBps16>(
                max_from,
                &unrealized.unrealized_loss.usd.height,
                market_cap,
                exit,
            )?;
        self.net_unrealized_pnl_rel_to_market_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBps16>(
                max_from,
                &unrealized.net_unrealized_pnl.usd.height,
                market_cap,
                exit,
            )?;
        self.invested_capital_in_profit_rel_to_realized_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.invested_capital_in_profit.usd.height,
                &realized.realized_cap.height,
                exit,
            )?;
        self.invested_capital_in_loss_rel_to_realized_cap
            .compute_binary::<Dollars, Dollars, RatioDollarsBp16>(
                max_from,
                &unrealized.invested_capital_in_loss.usd.height,
                &realized.realized_cap.height,
                exit,
            )?;
        Ok(())
    }
}
