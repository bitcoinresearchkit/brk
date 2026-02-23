use std::path::Path;

use brk_error::Result;
use brk_types::{Age, CentsSats, Cents, CostBasisSnapshot, Height, Sats, SupplyState};

use super::super::cost_basis::{CostBasisData, Percentiles, RealizedState, UnrealizedState};

pub struct CohortState {
    pub supply: SupplyState,
    pub realized: RealizedState,
    pub sent: Sats,
    pub satblocks_destroyed: Sats,
    pub satdays_destroyed: Sats,
    cost_basis_data: CostBasisData,
}

impl CohortState {
    pub(crate) fn new(path: &Path, name: &str) -> Self {
        Self {
            supply: SupplyState::default(),
            realized: RealizedState::default(),
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
    /// Uses the exact persisted values instead of recomputing from the map.
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

    pub(crate) fn increment(&mut self, supply: &SupplyState, price: Cents) {
        self.increment_snapshot(&CostBasisSnapshot::from_utxo(price, supply));
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

    pub(crate) fn decrement(&mut self, supply: &SupplyState, price: Cents) {
        self.decrement_snapshot(&CostBasisSnapshot::from_utxo(price, supply));
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
        self.supply += supply;

        if supply.value > Sats::ZERO {
            let sats = supply.value;

            // Compute once using typed values
            let price_sats = CentsSats::from_price_sats(price, sats);
            let investor_cap = price_sats.to_investor_cap(price);

            self.realized.receive(price, sats);

            self.cost_basis_data
                .increment(price, sats, price_sats, investor_cap);
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

    pub(crate) fn send_utxo(
        &mut self,
        supply: &SupplyState,
        current_price: Cents,
        prev_price: Cents,
        ath: Cents,
        age: Age,
    ) {
        if supply.utxo_count == 0 {
            return;
        }

        self.supply -= supply;

        if supply.value > Sats::ZERO {
            self.sent += supply.value;
            self.satblocks_destroyed += age.satblocks_destroyed(supply.value);
            self.satdays_destroyed += age.satdays_destroyed(supply.value);

            let cp = current_price;
            let pp = prev_price;
            let ath_price = ath;
            let sats = supply.value;

            // Compute ONCE using typed values
            let current_ps = CentsSats::from_price_sats(cp, sats);
            let prev_ps = CentsSats::from_price_sats(pp, sats);
            let ath_ps = CentsSats::from_price_sats(ath_price, sats);
            let prev_investor_cap = prev_ps.to_investor_cap(pp);

            self.realized
                .send(sats, current_ps, prev_ps, ath_ps, prev_investor_cap);

            self.cost_basis_data
                .decrement(pp, sats, prev_ps, prev_investor_cap);
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

    pub(crate) fn compute_unrealized_states(
        &mut self,
        height_price: Cents,
        date_price: Option<Cents>,
    ) -> (UnrealizedState, Option<UnrealizedState>) {
        self.cost_basis_data
            .compute_unrealized_states(height_price, date_price)
    }

    pub(crate) fn write(&mut self, height: Height, cleanup: bool) -> Result<()> {
        self.cost_basis_data.write(height, cleanup)
    }

    pub(crate) fn cost_basis_data_iter(&self) -> impl Iterator<Item = (Cents, &Sats)> {
        self.cost_basis_data.iter().map(|(k, v)| (k.into(), v))
    }
}
