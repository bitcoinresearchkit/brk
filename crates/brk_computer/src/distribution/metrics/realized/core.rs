use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Cents, CentsSigned, Dollars, Height, Indexes, StoredF64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode,
};

use crate::{
    blocks,
    distribution::state::{CohortState, CostBasisOps, RealizedOps},
    internal::{
        ComputedPerBlock, FiatRollingDelta1m, LazyPerBlock,
        NegCentsUnsignedToDollars, PerBlockWithSum24h, RatioCents64,
        RollingWindow24hPerBlock,
    },
    prices,
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedMinimal;

#[derive(Traversable)]
pub struct RealizedSoprCore<M: StorageMode = Rw> {
    pub ratio: RollingWindow24hPerBlock<StoredF64, M>,
}

#[derive(Deref, DerefMut, Traversable)]
pub struct RealizedCore<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub minimal: RealizedMinimal<M>,

    #[traversable(wrap = "profit", rename = "cumulative")]
    pub profit_cumulative: ComputedPerBlock<Cents, M>,
    #[traversable(wrap = "loss", rename = "cumulative")]
    pub loss_cumulative: ComputedPerBlock<Cents, M>,

    #[traversable(wrap = "cap", rename = "delta")]
    pub cap_delta: FiatRollingDelta1m<Cents, CentsSigned, M>,

    #[traversable(wrap = "loss", rename = "negative")]
    pub neg_loss: LazyPerBlock<Dollars, Cents>,
    pub net_pnl: PerBlockWithSum24h<CentsSigned, M>,
    pub sopr: RealizedSoprCore<M>,
}

impl RealizedCore {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let v1 = Version::ONE;

        let minimal = RealizedMinimal::forced_import(cfg)?;

        let neg_realized_loss = LazyPerBlock::from_height_source::<NegCentsUnsignedToDollars>(
            &cfg.name("neg_realized_loss"),
            cfg.version + Version::ONE,
            minimal.loss.raw.cents.height.read_only_boxed_clone(),
            cfg.indexes,
        );

        Ok(Self {
            minimal,
            profit_cumulative: cfg.import("realized_profit_cumulative", v0)?,
            loss_cumulative: cfg.import("realized_loss_cumulative", v0)?,
            cap_delta: cfg.import("realized_cap_delta", v1)?,
            neg_loss: neg_realized_loss,
            net_pnl: cfg.import("net_realized_pnl", v1)?,
            sopr: RealizedSoprCore {
                ratio: cfg.import("sopr", v1)?,
            },
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.minimal.min_stateful_height_len()
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &CohortState<impl RealizedOps, impl CostBasisOps>) -> Result<()> {
        self.minimal.truncate_push(height, state)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.minimal.collect_vecs_mut()
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

        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.minimal
            .compute_rest_part1(blocks, starting_indexes, exit)?;

        self.profit_cumulative.height.compute_cumulative(
            starting_indexes.height,
            &self.minimal.profit.raw.cents.height,
            exit,
        )?;
        self.loss_cumulative.height.compute_cumulative(
            starting_indexes.height,
            &self.minimal.loss.raw.cents.height,
            exit,
        )?;

        self.net_pnl.raw.height.compute_transform2(
            starting_indexes.height,
            &self.minimal.profit.raw.cents.height,
            &self.minimal.loss.raw.cents.height,
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

        self.cap_delta.compute(
            starting_indexes.height,
            &blocks.lookback.height_1m_ago,
            &self.minimal.cap.cents.height,
            exit,
        )?;

        self.net_pnl.sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_24h_ago,
            &self.net_pnl.raw.height,
            exit,
        )?;

        self.sopr
            .ratio
            ._24h
            .compute_binary::<Cents, Cents, RatioCents64>(
                starting_indexes.height,
                &self.minimal.sopr.value_created.sum._24h.height,
                &self.minimal.sopr.value_destroyed.sum._24h.height,
                exit,
            )?;

        Ok(())
    }
}
