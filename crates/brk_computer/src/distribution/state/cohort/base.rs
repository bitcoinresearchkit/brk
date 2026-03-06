use std::{collections::BTreeMap, path::Path};

use brk_error::Result;
use brk_types::{Age, Cents, CentsCompact, CentsSats, CentsSquaredSats, CostBasisSnapshot, Height, Sats, SupplyState};

use super::super::cost_basis::{CostBasisData, Percentiles, RealizedOps, UnrealizedState};

pub struct SendPrecomputed {
    pub sats: Sats,
    pub prev_price: Cents,
    pub age: Age,
    pub current_ps: CentsSats,
    pub prev_ps: CentsSats,
    pub ath_ps: CentsSats,
    pub prev_investor_cap: CentsSquaredSats,
}

impl SendPrecomputed {
    /// Pre-compute values for send_utxo when the same supply/prices are shared
    /// across multiple cohorts (age_range, epoch, class).
    pub(crate) fn new(
        supply: &SupplyState,
        current_price: Cents,
        prev_price: Cents,
        ath: Cents,
        age: Age,
    ) -> Option<Self> {
        if supply.utxo_count == 0 || supply.value == Sats::ZERO {
            return None;
        }
        let sats = supply.value;
        let current_ps = CentsSats::from_price_sats(current_price, sats);
        let prev_ps = CentsSats::from_price_sats(prev_price, sats);
        let ath_ps = if ath == current_price {
            current_ps
        } else {
            CentsSats::from_price_sats(ath, sats)
        };
        let prev_investor_cap = prev_ps.to_investor_cap(prev_price);
        Some(Self {
            sats,
            prev_price,
            age,
            current_ps,
            prev_ps,
            ath_ps,
            prev_investor_cap,
        })
    }
}

pub struct CohortState<R: RealizedOps> {
    pub supply: SupplyState,
    pub realized: R,
    pub sent: Sats,
    pub satblocks_destroyed: Sats,
    pub satdays_destroyed: Sats,
    cost_basis_data: CostBasisData,
}

impl<R: RealizedOps> CohortState<R> {
    pub(crate) fn new(path: &Path, name: &str) -> Self {
        Self {
            supply: SupplyState::default(),
            realized: R::default(),
            sent: Sats::ZERO,
            satblocks_destroyed: Sats::ZERO,
            satdays_destroyed: Sats::ZERO,
            cost_basis_data: CostBasisData::create(path, name),
        }
    }

    /// Enable price rounding for cost basis data.
    pub(crate) fn with_price_rounding(mut self, digits: i32) -> Self {
        self.cost_basis_data = self.cost_basis_data.with_price_rounding(digits);
        self
    }

    pub(crate) fn import_at_or_before(&mut self, height: Height) -> Result<Height> {
        self.cost_basis_data.import_at_or_before(height)
    }

    /// Restore realized cap from cost_basis_data after import.
    pub(crate) fn restore_realized_cap(&mut self) {
        self.realized.set_cap_raw(self.cost_basis_data.cap_raw());
        self.realized
            .set_investor_cap_raw(self.cost_basis_data.investor_cap_raw());
    }

    pub(crate) fn reset_cost_basis_data_if_needed(&mut self) -> Result<()> {
        self.cost_basis_data.clean()?;
        self.cost_basis_data.init();
        Ok(())
    }

    pub(crate) fn apply_pending(&mut self) {
        self.cost_basis_data.apply_pending();
    }

    pub(crate) fn cost_basis_data_first_key_value(&self) -> Option<(Cents, &Sats)> {
        self.cost_basis_data
            .first_key_value()
            .map(|(k, v)| (k.into(), v))
    }

    pub(crate) fn cost_basis_data_last_key_value(&self) -> Option<(Cents, &Sats)> {
        self.cost_basis_data
            .last_key_value()
            .map(|(k, v)| (k.into(), v))
    }

    pub(crate) fn reset_single_iteration_values(&mut self) {
        self.sent = Sats::ZERO;
        self.satdays_destroyed = Sats::ZERO;
        self.satblocks_destroyed = Sats::ZERO;
        self.realized.reset_single_iteration_values();
    }

    pub(crate) fn increment_snapshot(&mut self, s: &CostBasisSnapshot) {
        self.supply += &s.supply_state;

        if s.supply_state.value > Sats::ZERO {
            self.realized
                .increment_snapshot(s.price_sats, s.investor_cap);
            self.cost_basis_data.increment(
                s.realized_price,
                s.supply_state.value,
                s.price_sats,
                s.investor_cap,
            );
        }
    }

    pub(crate) fn decrement_snapshot(&mut self, s: &CostBasisSnapshot) {
        self.supply -= &s.supply_state;

        if s.supply_state.value > Sats::ZERO {
            self.realized
                .decrement_snapshot(s.price_sats, s.investor_cap);
            self.cost_basis_data.decrement(
                s.realized_price,
                s.supply_state.value,
                s.price_sats,
                s.investor_cap,
            );
        }
    }

    pub(crate) fn receive_utxo(&mut self, supply: &SupplyState, price: Cents) {
        self.receive_utxo_snapshot(supply, &CostBasisSnapshot::from_utxo(price, supply));
    }

    /// Like receive_utxo but takes a pre-computed snapshot to avoid redundant multiplication
    /// when the same supply/price is used across multiple cohorts.
    pub(crate) fn receive_utxo_snapshot(
        &mut self,
        supply: &SupplyState,
        snapshot: &CostBasisSnapshot,
    ) {
        self.supply += supply;

        if supply.value > Sats::ZERO {
            self.realized.receive(snapshot.realized_price, supply.value);

            self.cost_basis_data.increment(
                snapshot.realized_price,
                supply.value,
                snapshot.price_sats,
                snapshot.investor_cap,
            );
        }
    }

    pub(crate) fn receive_address(
        &mut self,
        supply: &SupplyState,
        price: Cents,
        current: &CostBasisSnapshot,
        prev: &CostBasisSnapshot,
    ) {
        self.supply += supply;

        if supply.value > Sats::ZERO {
            self.realized.receive(price, supply.value);

            if current.supply_state.value.is_not_zero() {
                self.cost_basis_data.increment(
                    current.realized_price,
                    current.supply_state.value,
                    current.price_sats,
                    current.investor_cap,
                );
            }

            if prev.supply_state.value.is_not_zero() {
                self.cost_basis_data.decrement(
                    prev.realized_price,
                    prev.supply_state.value,
                    prev.price_sats,
                    prev.investor_cap,
                );
            }
        }
    }

    pub(crate) fn send_utxo_precomputed(
        &mut self,
        supply: &SupplyState,
        pre: &SendPrecomputed,
    ) {
        self.supply -= supply;
        self.sent += pre.sats;
        self.satblocks_destroyed += pre.age.satblocks_destroyed(pre.sats);
        self.satdays_destroyed += pre.age.satdays_destroyed(pre.sats);

        self.realized
            .send(pre.sats, pre.current_ps, pre.prev_ps, pre.ath_ps, pre.prev_investor_cap);

        self.cost_basis_data
            .decrement(pre.prev_price, pre.sats, pre.prev_ps, pre.prev_investor_cap);
    }

    pub(crate) fn send_utxo(
        &mut self,
        supply: &SupplyState,
        current_price: Cents,
        prev_price: Cents,
        ath: Cents,
        age: Age,
    ) {
        if let Some(pre) = SendPrecomputed::new(supply, current_price, prev_price, ath, age) {
            self.send_utxo_precomputed(supply, &pre);
        } else if supply.utxo_count > 0 {
            self.supply -= supply;
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn send_address(
        &mut self,
        supply: &SupplyState,
        current_price: Cents,
        prev_price: Cents,
        ath: Cents,
        age: Age,
        current: &CostBasisSnapshot,
        prev: &CostBasisSnapshot,
    ) {
        if supply.utxo_count == 0 {
            return;
        }

        self.supply -= supply;

        if supply.value > Sats::ZERO {
            self.sent += supply.value;
            self.satblocks_destroyed += age.satblocks_destroyed(supply.value);
            self.satdays_destroyed += age.satdays_destroyed(supply.value);

            let sats = supply.value;

            // Compute once for realized.send using typed values
            let current_ps = CentsSats::from_price_sats(current_price, sats);
            let prev_ps = CentsSats::from_price_sats(prev_price, sats);
            let ath_ps = CentsSats::from_price_sats(ath, sats);
            let prev_investor_cap = prev_ps.to_investor_cap(prev_price);

            self.realized
                .send(sats, current_ps, prev_ps, ath_ps, prev_investor_cap);

            if current.supply_state.value.is_not_zero() {
                self.cost_basis_data.increment(
                    current.realized_price,
                    current.supply_state.value,
                    current.price_sats,
                    current.investor_cap,
                );
            }

            if prev.supply_state.value.is_not_zero() {
                self.cost_basis_data.decrement(
                    prev.realized_price,
                    prev.supply_state.value,
                    prev.price_sats,
                    prev.investor_cap,
                );
            }
        }
    }

    pub(crate) fn compute_percentiles(&mut self) -> Option<Percentiles> {
        self.cost_basis_data.compute_percentiles()
    }

    pub(crate) fn cached_percentiles(&self) -> Option<Percentiles> {
        self.cost_basis_data.cached_percentiles()
    }

    pub(crate) fn compute_unrealized_state(&mut self, height_price: Cents) -> UnrealizedState {
        self.cost_basis_data.compute_unrealized_state(height_price)
    }

    pub(crate) fn write(&mut self, height: Height, cleanup: bool) -> Result<()> {
        self.cost_basis_data.write(height, cleanup)
    }

    pub(crate) fn cost_basis_map(&self) -> &BTreeMap<CentsCompact, Sats> {
        self.cost_basis_data.map()
    }
}
