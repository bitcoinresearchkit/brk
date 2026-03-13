use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Bitcoin, Cents, CentsSigned, Dollars, Height, Indexes, StoredF64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{
    AnyStoredVec, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode,
};

use crate::{
    distribution::state::{CohortState, CostBasisOps, RealizedOps},
    internal::{
        FiatPerBlockCumulativeWithSumsAndDeltas, LazyPerBlock, NegCentsUnsignedToDollars,
        RatioCents64, RollingWindow24hPerBlock,
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

    #[traversable(wrap = "loss", rename = "negative")]
    pub neg_loss: LazyPerBlock<Dollars, Cents>,
    pub net_pnl: FiatPerBlockCumulativeWithSumsAndDeltas<CentsSigned, CentsSigned, BasisPointsSigned32, M>,
    pub sopr: RealizedSoprCore<M>,
}

impl RealizedCore {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        let minimal = RealizedMinimal::forced_import(cfg)?;

        let neg_realized_loss = LazyPerBlock::from_height_source::<NegCentsUnsignedToDollars>(
            &cfg.name("neg_realized_loss"),
            cfg.version + Version::ONE,
            minimal.loss.raw.cents.height.read_only_boxed_clone(),
            cfg.indexes,
        );

        let net_pnl = FiatPerBlockCumulativeWithSumsAndDeltas::forced_import(
            cfg.db,
            &cfg.name("net_realized_pnl"),
            cfg.version + v1,
            Version::new(4),
            cfg.indexes,
            cfg.cached_starts,
        )?;

        Ok(Self {
            minimal,
            neg_loss: neg_realized_loss,
            net_pnl,
            sopr: RealizedSoprCore {
                ratio: cfg.import("sopr", v1)?,
            },
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.minimal.min_stateful_len()
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
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.minimal
            .compute_rest_part1(starting_indexes, exit)?;

        self.net_pnl.raw.cents.height.compute_transform2(
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
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        height_to_supply: &impl ReadableVec<Height, Bitcoin>,
        exit: &Exit,
    ) -> Result<()> {
        self.minimal
            .compute_rest_part2(prices, starting_indexes, height_to_supply, exit)?;

        self.net_pnl
            .compute_rest(starting_indexes.height, exit)?;

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
