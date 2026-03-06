use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints32, BasisPointsSigned32, Bitcoin, Cents, CentsSigned, Dollars, Height, Indexes,
    StoredF64, Version,
};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, AnyVec, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode, WritableVec,
};

use crate::{
    blocks,
    distribution::state::RealizedOps,
    internal::{
        ComputedFromHeight, ComputedFromHeightCumulative,
        ComputedFromHeightRatioPercentiles, FiatFromHeight,
        LazyFromHeight, NegCentsUnsignedToDollars, PercentFromHeight, RatioCents64,
        RatioCentsBp32, RatioCentsSignedCentsBps32, RollingEmas1w1m, RollingEmas2w,
        RollingWindows, ValueFromHeightCumulative,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedMinimal;

#[derive(Deref, DerefMut, Traversable)]
pub struct RealizedBase<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub minimal: RealizedMinimal<M>,

    pub realized_cap_change_1m: ComputedFromHeight<CentsSigned, M>,

    pub neg_realized_loss: LazyFromHeight<Dollars, Cents>,
    pub net_realized_pnl: ComputedFromHeightCumulative<CentsSigned, M>,
    pub net_realized_pnl_ema_1w: ComputedFromHeight<CentsSigned, M>,
    pub gross_pnl: FiatFromHeight<Cents, M>,

    pub realized_profit_ema_1w: ComputedFromHeight<Cents, M>,
    pub realized_loss_ema_1w: ComputedFromHeight<Cents, M>,

    pub realized_profit_rel_to_realized_cap: PercentFromHeight<BasisPoints32, M>,
    pub realized_loss_rel_to_realized_cap: PercentFromHeight<BasisPoints32, M>,
    pub net_realized_pnl_rel_to_realized_cap: PercentFromHeight<BasisPointsSigned32, M>,

    pub value_created: ComputedFromHeight<Cents, M>,
    pub value_destroyed: ComputedFromHeight<Cents, M>,
    pub value_created_sum: RollingWindows<Cents, M>,
    pub value_destroyed_sum: RollingWindows<Cents, M>,
    pub sopr: RollingWindows<StoredF64, M>,
    pub sopr_24h_ema: RollingEmas1w1m<StoredF64, M>,

    pub realized_price_ratio_percentiles: ComputedFromHeightRatioPercentiles<M>,

    pub sent_in_profit: ValueFromHeightCumulative<M>,
    pub sent_in_loss: ValueFromHeightCumulative<M>,
    pub sent_in_profit_ema: RollingEmas2w<M>,
    pub sent_in_loss_ema: RollingEmas2w<M>,
}

impl RealizedBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let v1 = Version::ONE;

        let minimal = RealizedMinimal::forced_import(cfg)?;

        let neg_realized_loss = LazyFromHeight::from_height_source::<NegCentsUnsignedToDollars>(
            &cfg.name("neg_realized_loss"),
            cfg.version + Version::ONE,
            minimal.realized_loss.height.read_only_boxed_clone(),
            cfg.indexes,
        );

        let realized_profit_ema_1w = cfg.import_computed("realized_profit_ema_1w", v0)?;
        let realized_loss_ema_1w = cfg.import_computed("realized_loss_ema_1w", v0)?;

        let net_realized_pnl = cfg.import_cumulative("net_realized_pnl", v0)?;
        let net_realized_pnl_ema_1w = cfg.import_computed("net_realized_pnl_ema_1w", v0)?;
        let gross_pnl = cfg.import_fiat("realized_gross_pnl", v0)?;

        let realized_profit_rel_to_realized_cap =
            cfg.import_percent_bp32("realized_profit_rel_to_realized_cap", Version::new(2))?;
        let realized_loss_rel_to_realized_cap =
            cfg.import_percent_bp32("realized_loss_rel_to_realized_cap", Version::new(2))?;
        let net_realized_pnl_rel_to_realized_cap =
            cfg.import_percent_bps32("net_realized_pnl_rel_to_realized_cap", Version::new(2))?;

        let value_created = cfg.import_computed("value_created", v0)?;
        let value_destroyed = cfg.import_computed("value_destroyed", v0)?;
        let value_created_sum = cfg.import_rolling("value_created", v1)?;
        let value_destroyed_sum = cfg.import_rolling("value_destroyed", v1)?;
        let sopr = cfg.import_rolling("sopr", v1)?;
        let sopr_24h_ema = cfg.import_emas_1w_1m("sopr_24h", v1)?;

        let realized_price_ratio_percentiles =
            ComputedFromHeightRatioPercentiles::forced_import(
                cfg.db,
                &cfg.name("realized_price"),
                cfg.version + v1,
                cfg.indexes,
            )?;

        Ok(Self {
            minimal,
            realized_cap_change_1m: cfg.import_computed("realized_cap_change_1m", v0)?,
            neg_realized_loss,
            net_realized_pnl,
            net_realized_pnl_ema_1w,
            gross_pnl,
            realized_profit_ema_1w,
            realized_loss_ema_1w,
            realized_profit_rel_to_realized_cap,
            realized_loss_rel_to_realized_cap,
            net_realized_pnl_rel_to_realized_cap,
            value_created,
            value_destroyed,
            value_created_sum,
            value_destroyed_sum,
            sopr,
            sopr_24h_ema,
            realized_price_ratio_percentiles,
            sent_in_profit: cfg.import_value_cumulative("sent_in_profit", v0)?,
            sent_in_loss: cfg.import_value_cumulative("sent_in_loss", v0)?,
            sent_in_profit_ema: cfg.import_emas_2w("sent_in_profit", v0)?,
            sent_in_loss_ema: cfg.import_emas_2w("sent_in_loss", v0)?,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.minimal
            .min_stateful_height_len()
            .min(self.value_created.height.len())
            .min(self.value_destroyed.height.len())
            .min(self.sent_in_profit.base.sats.height.len())
            .min(self.sent_in_loss.base.sats.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &impl RealizedOps) -> Result<()> {
        self.minimal.truncate_push(height, state)?;
        self.value_created
            .height
            .truncate_push(height, state.value_created())?;
        self.value_destroyed
            .height
            .truncate_push(height, state.value_destroyed())?;
        self.sent_in_profit
            .base
            .sats
            .height
            .truncate_push(height, state.sent_in_profit())?;
        self.sent_in_loss
            .base
            .sats
            .height
            .truncate_push(height, state.sent_in_loss())?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.minimal.collect_vecs_mut();
        vecs.push(&mut self.value_created.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.value_destroyed.height);
        vecs.push(&mut self.sent_in_profit.base.sats.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.sent_in_loss.base.sats.height);
        vecs
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        let minimal_refs: Vec<&RealizedMinimal> = others.iter().map(|o| &o.minimal).collect();
        self.minimal
            .compute_from_stateful(starting_indexes, &minimal_refs, exit)?;

        sum_others!(self, starting_indexes, others, exit; value_created.height);
        sum_others!(self, starting_indexes, others, exit; value_destroyed.height);
        sum_others!(self, starting_indexes, others, exit; sent_in_profit.base.sats.height);
        sum_others!(self, starting_indexes, others, exit; sent_in_loss.base.sats.height);

        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.minimal.compute_rest_part1(starting_indexes, exit)?;

        self.net_realized_pnl
            .compute(starting_indexes.height, exit, |vec| {
                vec.compute_transform2(
                    starting_indexes.height,
                    &self.minimal.realized_profit.height,
                    &self.minimal.realized_loss.height,
                    |(i, profit, loss, ..)| {
                        (
                            i,
                            CentsSigned::new(profit.inner() as i64 - loss.inner() as i64),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.gross_pnl.cents.height.compute_add(
            starting_indexes.height,
            &self.minimal.realized_profit.height,
            &self.minimal.realized_loss.height,
            exit,
        )?;

        Ok(())
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        height_to_supply: &impl ReadableVec<Height, Bitcoin>,
        exit: &Exit,
    ) -> Result<()> {
        self.minimal
            .compute_rest_part2(prices, starting_indexes, height_to_supply, exit)?;

        self.realized_cap_change_1m.height.compute_rolling_change(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.minimal.realized_cap_cents.height,
            exit,
        )?;

        self.realized_profit_ema_1w.height.compute_rolling_ema(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.minimal.realized_profit.height,
            exit,
        )?;
        self.realized_loss_ema_1w.height.compute_rolling_ema(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.minimal.realized_loss.height,
            exit,
        )?;
        self.net_realized_pnl_ema_1w.height.compute_rolling_ema(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.net_realized_pnl.height,
            exit,
        )?;

        self.realized_profit_rel_to_realized_cap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.minimal.realized_profit.height,
                &self.minimal.realized_cap_cents.height,
                exit,
            )?;
        self.realized_loss_rel_to_realized_cap
            .compute_binary::<Cents, Cents, RatioCentsBp32>(
                starting_indexes.height,
                &self.minimal.realized_loss.height,
                &self.minimal.realized_cap_cents.height,
                exit,
            )?;
        self.net_realized_pnl_rel_to_realized_cap
            .compute_binary::<CentsSigned, Cents, RatioCentsSignedCentsBps32>(
                starting_indexes.height,
                &self.net_realized_pnl.height,
                &self.minimal.realized_cap_cents.height,
                exit,
            )?;

        // SOPR: rolling sums of stateful value_created/destroyed, then ratio, then EMAs
        let window_starts = blocks.count.window_starts();
        self.value_created_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.value_created.height,
            exit,
        )?;
        self.value_destroyed_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.value_destroyed.height,
            exit,
        )?;

        for ((sopr, vc), vd) in self
            .sopr
            .as_mut_array()
            .into_iter()
            .zip(self.value_created_sum.as_array())
            .zip(self.value_destroyed_sum.as_array())
        {
            sopr.compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &vc.height,
                &vd.height,
                exit,
            )?;
        }

        self.sopr_24h_ema.compute_from_24h(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &blocks.count.height_1m_ago,
            &self.sopr._24h.height,
            exit,
        )?;

        // Realized price ratio percentiles
        self.realized_price_ratio_percentiles.compute(
            blocks,
            starting_indexes,
            exit,
            &self.minimal.realized_price_ratio.ratio.height,
            &self.minimal.realized_price.cents.height,
        )?;

        // Sent in profit/loss EMAs
        self.sent_in_profit_ema.compute(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.sent_in_profit.base.sats.height,
            &self.sent_in_profit.base.cents.height,
            exit,
        )?;
        self.sent_in_loss_ema.compute(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.sent_in_loss.base.sats.height,
            &self.sent_in_loss.base.cents.height,
            exit,
        )?;

        Ok(())
    }
}
