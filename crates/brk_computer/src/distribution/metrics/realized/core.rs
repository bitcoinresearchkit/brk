use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, CentsSigned, Dollars, Height, Indexes, StoredF64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, AnyVec, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode, WritableVec,
};

use crate::{
    blocks,
    distribution::state::RealizedOps,
    internal::{
        ComputedFromHeight, FiatRollingDelta1m, LazyFromHeight, NegCentsUnsignedToDollars,
        RatioCents64, RollingWindow24h,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedMinimal;

#[derive(Deref, DerefMut, Traversable)]
pub struct RealizedCore<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub minimal: RealizedMinimal<M>,

    pub realized_cap_delta: FiatRollingDelta1m<Cents, CentsSigned, M>,

    pub neg_realized_loss: LazyFromHeight<Dollars, Cents>,
    pub net_realized_pnl: ComputedFromHeight<CentsSigned, M>,
    pub net_realized_pnl_sum: RollingWindow24h<CentsSigned, M>,

    pub value_created: ComputedFromHeight<Cents, M>,
    pub value_destroyed: ComputedFromHeight<Cents, M>,
    pub value_created_sum: RollingWindow24h<Cents, M>,
    pub value_destroyed_sum: RollingWindow24h<Cents, M>,
    pub sopr: RollingWindow24h<StoredF64, M>,
}

impl RealizedCore {
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

        let net_realized_pnl = cfg.import("net_realized_pnl", v1)?;
        let net_realized_pnl_sum = cfg.import("net_realized_pnl", v1)?;

        let value_created = cfg.import("value_created", v0)?;
        let value_destroyed = cfg.import("value_destroyed", v0)?;
        let value_created_sum = cfg.import("value_created", v1)?;
        let value_destroyed_sum = cfg.import("value_destroyed", v1)?;
        let sopr = cfg.import("sopr", v1)?;

        Ok(Self {
            minimal,
            realized_cap_delta: cfg.import("realized_cap_delta", v1)?,
            neg_realized_loss,
            net_realized_pnl,
            net_realized_pnl_sum,
            value_created,
            value_destroyed,
            value_created_sum,
            value_destroyed_sum,
            sopr,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.minimal
            .min_stateful_height_len()
            .min(self.value_created.height.len())
            .min(self.value_destroyed.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &impl RealizedOps) -> Result<()> {
        self.minimal.truncate_push(height, state)?;
        self.value_created
            .height
            .truncate_push(height, state.value_created())?;
        self.value_destroyed
            .height
            .truncate_push(height, state.value_destroyed())?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.minimal.collect_vecs_mut();
        vecs.push(&mut self.value_created.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.value_destroyed.height);
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

        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.minimal.compute_rest_part1(blocks, starting_indexes, exit)?;

        self.net_realized_pnl.height.compute_transform2(
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

        self.realized_cap_delta.compute(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.minimal.realized_cap_cents.height,
            exit,
        )?;

        self.net_realized_pnl_sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_24h_ago,
            &self.net_realized_pnl.height,
            exit,
        )?;

        self.value_created_sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_24h_ago,
            &self.value_created.height,
            exit,
        )?;
        self.value_destroyed_sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_24h_ago,
            &self.value_destroyed.height,
            exit,
        )?;

        self.sopr._24h.compute_binary::<Cents, Cents, RatioCents64>(
            starting_indexes.height,
            &self.value_created_sum._24h.height,
            &self.value_destroyed_sum._24h.height,
            exit,
        )?;

        Ok(())
    }
}
