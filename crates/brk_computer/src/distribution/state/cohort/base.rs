use std::path::Path;

use brk_error::Result;
use brk_types::{Age, Cents, CentsCompact, CentsSats, CentsSquaredSats, CostBasisSnapshot, Height, Sats, SupplyState};

use super::super::{cost_basis::{Accumulate, CostBasisData, CostBasisOps, RealizedOps, UnrealizedState}, pending::PendingDelta};

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

pub struct CohortState<R: RealizedOps, C: CostBasisOps> {
    pub supply: SupplyState,
    pub realized: R,
    pub sent: Sats,
    pub satdays_destroyed: Sats,
    cost_basis: C,
}

impl<R: RealizedOps, C: CostBasisOps> CohortState<R, C> {
    pub(crate) fn new(path: &Path, name: &str) -> Self {
        Self {
            supply: SupplyState::default(),
            realized: R::default(),
            sent: Sats::ZERO,
            satdays_destroyed: Sats::ZERO,
            cost_basis: C::create(path, name),
        }
    }

    /// Enable price rounding for cost basis data.
    pub(crate) fn with_price_rounding(mut self, digits: i32) -> Self {
        self.cost_basis = self.cost_basis.with_price_rounding(digits);
        self
    }

    pub(crate) fn import_at_or_before(&mut self, height: Height) -> Result<Height> {
        self.cost_basis.import_at_or_before(height)
    }

    /// Restore realized cap from cost_basis after import.
    pub(crate) fn restore_realized_cap(&mut self) {
        self.realized.set_cap_raw(self.cost_basis.cap_raw());
        self.realized
            .set_investor_cap_raw(self.cost_basis.investor_cap_raw());
    }

    pub(crate) fn reset_cost_basis_data_if_needed(&mut self) -> Result<()> {
        self.cost_basis.clean()?;
        self.cost_basis.init();
        Ok(())
    }

    pub(crate) fn apply_pending(&mut self) {
        self.cost_basis.apply_pending();
    }

    pub(crate) fn reset_single_iteration_values(&mut self) {
        self.sent = Sats::ZERO;
        if R::TRACK_ACTIVITY {
            self.satdays_destroyed = Sats::ZERO;
        }
        self.realized.reset_single_iteration_values();
    }

    pub(crate) fn increment_snapshot(&mut self, s: &CostBasisSnapshot) {
        self.supply += &s.supply_state;

        if s.supply_state.value > Sats::ZERO {
            self.realized
                .increment_snapshot(s.price_sats, s.investor_cap);
            self.cost_basis.increment(
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
            self.cost_basis.decrement(
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

            self.cost_basis.increment(
                snapshot.realized_price,
                supply.value,
                snapshot.price_sats,
                snapshot.investor_cap,
            );
        }
    }

    pub(crate) fn receive_addr(
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
                self.cost_basis.increment(
                    current.realized_price,
                    current.supply_state.value,
                    current.price_sats,
                    current.investor_cap,
                );
            }

            if prev.supply_state.value.is_not_zero() {
                self.cost_basis.decrement(
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
        if R::TRACK_ACTIVITY {
            self.satdays_destroyed += pre.age.satdays_destroyed(pre.sats);
        }

        self.realized
            .send(pre.sats, pre.current_ps, pre.prev_ps, pre.ath_ps, pre.prev_investor_cap);

        self.cost_basis
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
    pub(crate) fn send_addr(
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
            if R::TRACK_ACTIVITY {
                self.satdays_destroyed += age.satdays_destroyed(supply.value);
            }

            let sats = supply.value;

            // Compute once for realized.send using typed values
            let current_ps = CentsSats::from_price_sats(current_price, sats);
            let prev_ps = CentsSats::from_price_sats(prev_price, sats);
            let ath_ps = CentsSats::from_price_sats(ath, sats);
            let prev_investor_cap = prev_ps.to_investor_cap(prev_price);

            self.realized
                .send(sats, current_ps, prev_ps, ath_ps, prev_investor_cap);

            if current.supply_state.value.is_not_zero() {
                self.cost_basis.increment(
                    current.realized_price,
                    current.supply_state.value,
                    current.price_sats,
                    current.investor_cap,
                );
            }

            if prev.supply_state.value.is_not_zero() {
                self.cost_basis.decrement(
                    prev.realized_price,
                    prev.supply_state.value,
                    prev.price_sats,
                    prev.investor_cap,
                );
            }
        }
    }

    pub(crate) fn write(&mut self, height: Height, cleanup: bool) -> Result<()> {
        self.cost_basis.write(height, cleanup)
    }
}

/// Methods only available with CostBasisData (map + unrealized).
impl<R: RealizedOps, S: Accumulate> CohortState<R, CostBasisData<S>> {
    pub(crate) fn compute_unrealized_state(&mut self, height_price: Cents) -> UnrealizedState {
        self.cost_basis.compute_unrealized_state(height_price)
    }

    pub(crate) fn for_each_cost_basis_pending(&self, f: impl FnMut(&CentsCompact, &PendingDelta)) {
        self.cost_basis.for_each_pending(f);
    }

    pub(crate) fn cost_basis_map(&self) -> &std::collections::BTreeMap<CentsCompact, Sats> {
        self.cost_basis.map()
    }
}
