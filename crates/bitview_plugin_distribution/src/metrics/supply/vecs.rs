use bitview_cohort::{
    AgeRange, AgeRangeId, CohortContext, Filter, UTXOAndAddrGroups, UTXOGroupsWithoutAmount,
    UTXORows,
};
use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_transforms::{HalveCents, HalveDollars, HalveSats, HalveSatsToBitcoin, SatsToCents};
use bitview_traversable::Traversable;
use bitview_vecs::{
    CachedWindowStartVec, ColumnarValuePerBlockCumulativeRolling, LazyPercentPerBlock,
    LazyRollingDeltasAmountFromHeight, LazyValuePerBlock, LazyValuePerBlockCumulativeRolling,
};
use brk_error::Result;
use brk_types::{
    Cents, Height, PartsPerMillion32, PartsPerMillionSigned64, Sats, SatsSigned, Version,
};
use vecdb::{
    AnyStoredVec, BinaryTransform, CacheBudget, CachedBoxedVec, Database, Rw, StorageMode,
};

use super::{SupplyBase, SupplyByCohort, SupplySources, SupplyTotal};
use crate::state::UnrealizedState;

const MATURED_VERSION: Version = Version::new(5);

#[derive(Traversable)]
pub struct SupplyVecs<M: StorageMode = Rw> {
    /// Supply: amount of bitcoin held in unspent transaction outputs.
    pub total: SupplyTotal<M>,
    /// Amount of unspent bitcoin that ages out of an exact UTXO age range
    /// during the represented block interval.
    pub matured: ColumnarValuePerBlockCumulativeRolling<
        AgeRangeId,
        AgeRange<LazyValuePerBlockCumulativeRolling>,
        M,
    >,
    /// One half of a UTXO cohort's unspent supply.
    pub half: UTXOGroupsWithoutAmount<LazyValuePerBlock>,
    /// Unspent supply in profit: UTXO cohort outputs whose creation price is
    /// less than or equal to the represented block's spot price.
    pub in_profit: SupplyByCohort<M>,
    /// Unspent supply in loss: UTXO cohort outputs whose creation price is
    /// greater than the represented block's spot price.
    pub in_loss: SupplyByCohort<M>,
    /// Change in a UTXO or address-balance cohort's unspent supply over a trailing window, with
    /// the relative change measured against the window's starting value.
    pub delta: UTXOAndAddrGroups<
        LazyRollingDeltasAmountFromHeight<Sats, SatsSigned, PartsPerMillionSigned64>,
    >,
    /// Share of all unspent supply held by a UTXO or address-balance cohort.
    pub dominance: UTXOAndAddrGroups<LazyPercentPerBlock<PartsPerMillion32>>,
}

impl SupplyVecs {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        cached_starts: &Windows<&CachedWindowStartVec>,
        spot_price: &CachedBoxedVec<Height, Cents>,
    ) -> Result<Self> {
        let total = SupplyTotal::forced_import(cache, db, version, mappings, spot_price)?;
        let all_supply = total.all_supply();
        let in_profit = SupplyByCohort::forced_import(
            cache,
            db,
            "supply_in_profit",
            version,
            mappings,
            spot_price,
        )?;
        let in_loss = SupplyByCohort::forced_import(
            cache,
            db,
            "supply_in_loss",
            version,
            mappings,
            spot_price,
        )?;
        let utxo = total.cohorts.utxo.map_named(|filter, cohort_name, total| {
            let full_name = CohortContext::Utxo.full_name(filter, cohort_name);
            if matches!(filter, Filter::All) {
                SupplyBase::from_all_total(
                    &full_name,
                    version,
                    total.clone(),
                    mappings,
                    cached_starts,
                )
            } else {
                SupplyBase::from_total(
                    &full_name,
                    version,
                    total.clone(),
                    all_supply,
                    mappings,
                    cached_starts,
                )
            }
        });
        let addr_balance = total
            .cohorts
            .addr_balance
            .series
            .map_named(|filter, name, total| {
                let full_name = CohortContext::Addr.full_name(filter, name);
                SupplyBase::from_total(
                    &full_name,
                    version + Version::ONE,
                    total.clone(),
                    all_supply,
                    mappings,
                    cached_starts,
                )
            });
        let bases = UTXOAndAddrGroups { utxo, addr_balance };
        let delta = bases.map_named(|_, _, _, base| base.delta.clone());
        let dominance = bases.map_named(|_, _, _, base| base.dominance.clone());
        let half = in_profit.cohorts.map_named(|filter, cohort_name, _| {
            let full_name = CohortContext::Utxo.full_name(filter, cohort_name);
            LazyValuePerBlock::from_spot_block_source::<
                HalveSats,
                HalveSatsToBitcoin,
                HalveCents,
                HalveDollars,
            >(
                &SupplyBase::metric_name(&full_name, "supply_half"),
                total.get(filter).expect("supported half-supply view"),
                version,
            )
        });
        let matured_version = version + MATURED_VERSION;
        let matured = ColumnarValuePerBlockCumulativeRolling::forced_import(
            cache,
            db,
            &format!(
                "{}_age_range_matured_supply_cumulative",
                CohortContext::Utxo.prefix()
            ),
            matured_version,
            |sats, cents| {
                AgeRangeId::series(CohortContext::Utxo, |column, name| {
                    let name = format!("{name}_matured_supply");
                    let (sats, cents) =
                        ColumnarValuePerBlockCumulativeRolling::<AgeRangeId, ()>::sources_from(
                            sats,
                            cents,
                            &format!("{name}_cumulative"),
                            matured_version,
                            [column],
                        );
                    LazyValuePerBlockCumulativeRolling::from_cumulative_sources(
                        &name,
                        matured_version,
                        &sats,
                        &cents,
                        mappings,
                        cached_starts,
                    )
                })
            },
        )?;

        Ok(Self {
            total,
            matured,
            half,
            in_profit,
            in_loss,
            delta,
            dominance,
        })
    }

    pub fn sources(&self, filter: &Filter) -> Option<SupplySources> {
        Some(SupplySources {
            total: self.total.get(filter)?.clone(),
            in_profit: self.in_profit.get(filter)?.clone(),
        })
    }

    pub fn min_resume_len(&self) -> usize {
        self.total
            .min_len()
            .min(self.matured.len())
            .min(self.in_profit.min_len())
            .min(self.in_loss.min_len())
    }

    #[inline(always)]
    pub fn push_maturation(&mut self, matured: &AgeRange<Sats>, price: Cents) {
        let cents = AgeRange::from_fn(|column| SatsToCents::apply(*column.select(matured), price));
        self.matured.push_block(matured.clone(), cents);
    }

    #[inline(always)]
    pub fn push(&mut self, total: UTXORows<Sats>, profitability: &UTXORows<UnrealizedState>) {
        let in_profit = profitability.map(|state| state.supply_in_profit);
        let in_loss = profitability.map(|state| state.supply_in_loss);

        self.total.push(total);
        self.in_profit.push(in_profit);
        self.in_loss.push(in_loss);
    }

    pub fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.total.collect_vecs_mut();
        vecs.extend(self.matured.collect_vecs_mut());
        vecs.extend(self.in_profit.collect_vecs_mut());
        vecs.extend(self.in_loss.collect_vecs_mut());
        vecs
    }
}
