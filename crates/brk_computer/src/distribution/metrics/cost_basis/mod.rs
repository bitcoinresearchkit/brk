use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Cents, Height, Indexes, Sats, Version};
use brk_types::CentsSquaredSats;
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, Rw, StorageMode, WritableVec};

use crate::internal::{FiatPerBlock, PerBlock, PercentPerBlock, PercentilesVecs, Price, PERCENTILES_LEN};

use super::ImportConfig;

#[derive(Traversable)]
pub struct CostBasisSide<M: StorageMode = Rw> {
    pub per_coin: FiatPerBlock<Cents, M>,
    pub per_dollar: FiatPerBlock<Cents, M>,
}

/// Cost basis metrics: min/max + profit/loss splits + percentiles + supply density.
/// Used by all/sth/lth cohorts only.
#[derive(Traversable)]
pub struct CostBasis<M: StorageMode = Rw> {
    pub in_profit: CostBasisSide<M>,
    pub in_loss: CostBasisSide<M>,
    pub min: Price<PerBlock<Cents, M>>,
    pub max: Price<PerBlock<Cents, M>>,
    pub percentiles: PercentilesVecs<M>,
    pub invested_capital: PercentilesVecs<M>,
    pub supply_density: PercentPerBlock<BasisPoints16, M>,
}

impl CostBasis {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            in_profit: CostBasisSide {
                per_coin: cfg.import("cost_basis_in_profit_per_coin", Version::ZERO)?,
                per_dollar: cfg.import("cost_basis_in_profit_per_dollar", Version::ZERO)?,
            },
            in_loss: CostBasisSide {
                per_coin: cfg.import("cost_basis_in_loss_per_coin", Version::ZERO)?,
                per_dollar: cfg.import("cost_basis_in_loss_per_dollar", Version::ZERO)?,
            },
            min: cfg.import("cost_basis_min", Version::ZERO)?,
            max: cfg.import("cost_basis_max", Version::ZERO)?,
            percentiles: PercentilesVecs::forced_import(
                cfg.db,
                &cfg.name("cost_basis"),
                cfg.version,
                cfg.indexes,
            )?,
            invested_capital: PercentilesVecs::forced_import(
                cfg.db,
                &cfg.name("invested_capital"),
                cfg.version,
                cfg.indexes,
            )?,
            supply_density: PercentPerBlock::forced_import(
                cfg.db,
                &cfg.name("supply_density"),
                cfg.version,
                cfg.indexes,
            )?,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.min
            .cents
            .height
            .len()
            .min(self.max.cents.height.len())
            .min(self.supply_density.bps.height.len())
    }

    #[inline(always)]
    pub(crate) fn push_minmax(&mut self, min_price: Cents, max_price: Cents) {
        self.min.cents.height.push(min_price);
        self.max.cents.height.push(max_price);
    }

    #[inline(always)]
    pub(crate) fn push_percentiles(
        &mut self,
        sat_prices: &[Cents; PERCENTILES_LEN],
        usd_prices: &[Cents; PERCENTILES_LEN],
    ) {
        self.percentiles.push(sat_prices);
        self.invested_capital.push(usd_prices);
    }

    #[inline(always)]
    pub(crate) fn push_density(&mut self, density_bps: BasisPoints16) {
        self.supply_density.bps.height.push(density_bps);
    }

    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.percentiles
            .validate_computed_version_or_reset(base_version)?;
        self.invested_capital
            .validate_computed_version_or_reset(base_version)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = vec![
            &mut self.in_profit.per_coin.cents.height,
            &mut self.in_profit.per_dollar.cents.height,
            &mut self.in_loss.per_coin.cents.height,
            &mut self.in_loss.per_dollar.cents.height,
            &mut self.min.cents.height,
            &mut self.max.cents.height,
            &mut self.supply_density.bps.height,
        ];
        vecs.extend(
            self.percentiles
                .vecs
                .iter_mut()
                .map(|v| &mut v.cents.height as &mut dyn AnyStoredVec),
        );
        vecs.extend(
            self.invested_capital
                .vecs
                .iter_mut()
                .map(|v| &mut v.cents.height as &mut dyn AnyStoredVec),
        );
        vecs
    }

    pub(crate) fn compute_prices(
        &mut self,
        starting_indexes: &Indexes,
        invested_cap_in_profit: &impl ReadableVec<Height, Cents>,
        invested_cap_in_loss: &impl ReadableVec<Height, Cents>,
        supply_in_profit_sats: &impl ReadableVec<Height, Sats>,
        supply_in_loss_sats: &impl ReadableVec<Height, Sats>,
        investor_cap_in_profit_raw: &impl ReadableVec<Height, CentsSquaredSats>,
        investor_cap_in_loss_raw: &impl ReadableVec<Height, CentsSquaredSats>,
        exit: &Exit,
    ) -> Result<()> {
        self.in_profit.per_coin.cents.height.compute_transform2(
            starting_indexes.height,
            invested_cap_in_profit,
            supply_in_profit_sats,
            |(h, invested_cents, supply_sats, ..)| {
                let supply = supply_sats.as_u128();
                if supply == 0 { return (h, Cents::ZERO); }
                (h, Cents::new((invested_cents.as_u128() * Sats::ONE_BTC_U128 / supply) as u64))
            },
            exit,
        )?;
        self.in_loss.per_coin.cents.height.compute_transform2(
            starting_indexes.height,
            invested_cap_in_loss,
            supply_in_loss_sats,
            |(h, invested_cents, supply_sats, ..)| {
                let supply = supply_sats.as_u128();
                if supply == 0 { return (h, Cents::ZERO); }
                (h, Cents::new((invested_cents.as_u128() * Sats::ONE_BTC_U128 / supply) as u64))
            },
            exit,
        )?;
        self.in_profit.per_dollar.cents.height.compute_transform2(
            starting_indexes.height,
            investor_cap_in_profit_raw,
            invested_cap_in_profit,
            |(h, investor_cap, invested_cents, ..)| {
                let invested_raw = invested_cents.as_u128() * Sats::ONE_BTC_U128;
                if invested_raw == 0 { return (h, Cents::ZERO); }
                (h, Cents::new((investor_cap.inner() / invested_raw) as u64))
            },
            exit,
        )?;
        self.in_loss.per_dollar.cents.height.compute_transform2(
            starting_indexes.height,
            investor_cap_in_loss_raw,
            invested_cap_in_loss,
            |(h, investor_cap, invested_cents, ..)| {
                let invested_raw = invested_cents.as_u128() * Sats::ONE_BTC_U128;
                if invested_raw == 0 { return (h, Cents::ZERO); }
                (h, Cents::new((investor_cap.inner() / invested_raw) as u64))
            },
            exit,
        )?;
        Ok(())
    }
}
