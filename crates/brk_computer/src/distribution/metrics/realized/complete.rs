use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPointsSigned32, Bitcoin, Cents, CentsSigned, Dollars, Height, Indexes, StoredF64, Version,
};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, AnyVec, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode, WritableVec,
};

use crate::{
    blocks,
    distribution::state::RealizedState,
    internal::{
        CentsPlus, CentsUnsignedToDollars, ComputedFromHeight, LazyFromHeight, PercentFromHeight,
        RatioCents64, RatioCentsSignedCentsBps32, RatioCentsSignedDollarsBps32, RollingEmas1w1m,
        RollingEmas2w, RollingWindows, ValueFromHeightCumulative,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

use super::CoreRealized;

#[derive(Deref, DerefMut, Traversable)]
pub struct RealizedComplete<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: CoreRealized<M>,

    pub profit_value_created: ComputedFromHeight<Cents, M>,
    pub profit_value_destroyed: ComputedFromHeight<Cents, M>,
    pub loss_value_created: ComputedFromHeight<Cents, M>,
    pub loss_value_destroyed: ComputedFromHeight<Cents, M>,

    pub value_created: ComputedFromHeight<Cents, M>,
    pub value_destroyed: ComputedFromHeight<Cents, M>,

    pub capitulation_flow: LazyFromHeight<Dollars, Cents>,
    pub profit_flow: LazyFromHeight<Dollars, Cents>,

    pub value_created_sum: RollingWindows<Cents, M>,
    pub value_destroyed_sum: RollingWindows<Cents, M>,

    pub gross_pnl_sum: RollingWindows<Cents, M>,

    pub net_pnl_change_1m: ComputedFromHeight<CentsSigned, M>,
    pub net_pnl_change_1m_rel_to_realized_cap: PercentFromHeight<BasisPointsSigned32, M>,
    pub net_pnl_change_1m_rel_to_market_cap: PercentFromHeight<BasisPointsSigned32, M>,

    pub sopr: RollingWindows<StoredF64, M>,
    pub sopr_24h_ema: RollingEmas1w1m<StoredF64, M>,

    pub sent_in_profit: ValueFromHeightCumulative<M>,
    pub sent_in_profit_ema: RollingEmas2w<M>,
    pub sent_in_loss: ValueFromHeightCumulative<M>,
    pub sent_in_loss_ema: RollingEmas2w<M>,
}

impl RealizedComplete {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;

        let core = CoreRealized::forced_import(cfg)?;

        let profit_value_created = cfg.import_computed("profit_value_created", v0)?;
        let profit_value_destroyed = cfg.import_computed("profit_value_destroyed", v0)?;
        let loss_value_created = cfg.import_computed("loss_value_created", v0)?;
        let loss_value_destroyed = cfg.import_computed("loss_value_destroyed", v0)?;
        let value_created = cfg.import_computed("value_created", v0)?;
        let value_destroyed = cfg.import_computed("value_destroyed", v0)?;

        let capitulation_flow = LazyFromHeight::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("capitulation_flow"),
            cfg.version,
            loss_value_destroyed.height.read_only_boxed_clone(),
            &loss_value_destroyed,
        );
        let profit_flow = LazyFromHeight::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("profit_flow"),
            cfg.version,
            profit_value_destroyed.height.read_only_boxed_clone(),
            &profit_value_destroyed,
        );

        let value_created_sum = cfg.import_rolling("value_created", Version::ONE)?;
        let value_destroyed_sum = cfg.import_rolling("value_destroyed", Version::ONE)?;
        let gross_pnl_sum = cfg.import_rolling("gross_pnl_sum", Version::ONE)?;

        let sopr = cfg.import_rolling("sopr", Version::ONE)?;
        let sopr_24h_ema = cfg.import_emas_1w_1m("sopr_24h", Version::ONE)?;

        Ok(Self {
            core,
            profit_value_created,
            profit_value_destroyed,
            loss_value_created,
            loss_value_destroyed,
            value_created,
            value_destroyed,
            capitulation_flow,
            profit_flow,
            value_created_sum,
            value_destroyed_sum,
            gross_pnl_sum,
            sopr,
            sopr_24h_ema,
            net_pnl_change_1m: cfg.import_computed("net_pnl_change_1m", Version::new(3))?,
            net_pnl_change_1m_rel_to_realized_cap: cfg
                .import_percent_bps32("net_pnl_change_1m_rel_to_realized_cap", Version::new(4))?,
            net_pnl_change_1m_rel_to_market_cap: cfg
                .import_percent_bps32("net_pnl_change_1m_rel_to_market_cap", Version::new(4))?,
            sent_in_profit: cfg.import_value_cumulative("sent_in_profit", v0)?,
            sent_in_profit_ema: cfg.import_emas_2w("sent_in_profit", v0)?,
            sent_in_loss: cfg.import_value_cumulative("sent_in_loss", v0)?,
            sent_in_loss_ema: cfg.import_emas_2w("sent_in_loss", v0)?,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.core
            .min_stateful_height_len()
            .min(self.profit_value_created.height.len())
            .min(self.profit_value_destroyed.height.len())
            .min(self.loss_value_created.height.len())
            .min(self.loss_value_destroyed.height.len())
            .min(self.sent_in_profit.base.sats.height.len())
            .min(self.sent_in_loss.base.sats.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &RealizedState) -> Result<()> {
        self.core.truncate_push(height, state)?;
        self.profit_value_created
            .height
            .truncate_push(height, state.profit_value_created())?;
        self.profit_value_destroyed
            .height
            .truncate_push(height, state.profit_value_destroyed())?;
        self.loss_value_created
            .height
            .truncate_push(height, state.loss_value_created())?;
        self.loss_value_destroyed
            .height
            .truncate_push(height, state.loss_value_destroyed())?;
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
        let mut vecs = self.core.collect_vecs_mut();
        vecs.push(&mut self.profit_value_created.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.profit_value_destroyed.height);
        vecs.push(&mut self.loss_value_created.height);
        vecs.push(&mut self.loss_value_destroyed.height);
        vecs.push(&mut self.sent_in_profit.base.sats.height);
        vecs.push(&mut self.sent_in_loss.base.sats.height);
        vecs
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        let core_refs: Vec<&CoreRealized> = others.iter().map(|o| &o.core).collect();
        self.core
            .compute_from_stateful(starting_indexes, &core_refs, exit)?;

        sum_others!(self, starting_indexes, others, exit; profit_value_created.height);
        sum_others!(self, starting_indexes, others, exit; profit_value_destroyed.height);
        sum_others!(self, starting_indexes, others, exit; loss_value_created.height);
        sum_others!(self, starting_indexes, others, exit; loss_value_destroyed.height);
        sum_others!(self, starting_indexes, others, exit; sent_in_profit.base.sats.height);
        sum_others!(self, starting_indexes, others, exit; sent_in_loss.base.sats.height);

        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.core.compute_rest_part1(starting_indexes, exit)?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        height_to_supply: &impl ReadableVec<Height, Bitcoin>,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        self.core.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            height_to_supply,
            exit,
        )?;

        self.value_created
            .compute_binary::<Cents, Cents, CentsPlus>(
                starting_indexes.height,
                &self.profit_value_created.height,
                &self.loss_value_created.height,
                exit,
            )?;
        self.value_destroyed
            .compute_binary::<Cents, Cents, CentsPlus>(
                starting_indexes.height,
                &self.profit_value_destroyed.height,
                &self.loss_value_destroyed.height,
                exit,
            )?;

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
        self.gross_pnl_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.core.gross_pnl.cents.height,
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

        self.net_pnl_change_1m.height.compute_rolling_change(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.core.net_realized_pnl.cumulative.height,
            exit,
        )?;

        self.net_pnl_change_1m_rel_to_realized_cap
            .compute_binary::<CentsSigned, Cents, RatioCentsSignedCentsBps32>(
                starting_indexes.height,
                &self.net_pnl_change_1m.height,
                &self.core.realized_cap_cents.height,
                exit,
            )?;

        self.net_pnl_change_1m_rel_to_market_cap
            .compute_binary::<CentsSigned, Dollars, RatioCentsSignedDollarsBps32>(
                starting_indexes.height,
                &self.net_pnl_change_1m.height,
                height_to_market_cap,
                exit,
            )?;

        Ok(())
    }
}
