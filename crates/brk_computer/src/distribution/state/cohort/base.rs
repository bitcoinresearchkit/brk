use std::path::Path;

use brk_error::Result;
use brk_types::{Age, CentsSats, CentsUnsigned, CostBasisSnapshot, Height, Sats, SupplyState};

use super::super::cost_basis::{
    CachedUnrealizedState, Percentiles, CostBasisData, RealizedState, UnrealizedState,
};

#[derive(Clone)]
pub struct CohortState {
    pub supply: SupplyState,
    pub realized: Option<RealizedState>,
    pub sent: Sats,
    pub satblocks_destroyed: Sats,
    pub satdays_destroyed: Sats,
    cost_basis_data: Option<CostBasisData>,
    cached_unrealized: Option<CachedUnrealizedState>,
}

impl CohortState {
    pub fn new(path: &Path, name: &str, compute_dollars: bool) -> Self {
        Self {
            supply: SupplyState::default(),
            realized: compute_dollars.then_some(RealizedState::default()),
            sent: Sats::ZERO,
            satblocks_destroyed: Sats::ZERO,
            satdays_destroyed: Sats::ZERO,
            cost_basis_data: compute_dollars.then_some(CostBasisData::create(path, name)),
            cached_unrealized: None,
        }
    }

    pub fn import_at_or_before(&mut self, height: Height) -> Result<Height> {
        self.cached_unrealized = None;
        match self.cost_basis_data.as_mut() {
            Some(p) => p.import_at_or_before(height),
            None => Ok(height),
        }
    }

    /// Restore realized cap from cost_basis_data after import.
    /// Uses the exact persisted values instead of recomputing from the map.
    pub fn restore_realized_cap(&mut self) {
        if let Some(cost_basis_data) = self.cost_basis_data.as_ref()
            && let Some(realized) = self.realized.as_mut()
        {
            realized.set_cap_raw(cost_basis_data.cap_raw());
            realized.set_investor_cap_raw(cost_basis_data.investor_cap_raw());
        }
    }

    pub fn reset_cost_basis_data_if_needed(&mut self) -> Result<()> {
        if let Some(p) = self.cost_basis_data.as_mut() {
            p.clean()?;
            p.init();
        }
        self.cached_unrealized = None;
        Ok(())
    }

    pub fn apply_pending(&mut self) {
        if let Some(p) = self.cost_basis_data.as_mut() {
            p.apply_pending();
        }
    }

    pub fn cost_basis_data_first_key_value(&self) -> Option<(CentsUnsigned, &Sats)> {
        self.cost_basis_data.as_ref()?.first_key_value().map(|(k, v)| (k.into(), v))
    }

    pub fn cost_basis_data_last_key_value(&self) -> Option<(CentsUnsigned, &Sats)> {
        self.cost_basis_data.as_ref()?.last_key_value().map(|(k, v)| (k.into(), v))
    }

    pub fn reset_single_iteration_values(&mut self) {
        self.sent = Sats::ZERO;
        self.satdays_destroyed = Sats::ZERO;
        self.satblocks_destroyed = Sats::ZERO;
        if let Some(realized) = self.realized.as_mut() {
            realized.reset_single_iteration_values();
        }
    }

    pub fn increment(&mut self, supply: &SupplyState, price: Option<CentsUnsigned>) {
        match price {
            Some(p) => self.increment_snapshot(&CostBasisSnapshot::from_utxo(p, supply)),
            None => self.supply += supply,
        }
    }

    pub fn increment_snapshot(&mut self, s: &CostBasisSnapshot) {
        self.supply += &s.supply_state;

        if s.supply_state.value > Sats::ZERO
            && let Some(realized) = self.realized.as_mut()
        {
            realized.increment_snapshot(s.price_sats, s.investor_cap);
            self.cost_basis_data.as_mut().unwrap().increment(
                s.realized_price,
                s.supply_state.value,
                s.price_sats,
                s.investor_cap,
            );

            if let Some(cache) = self.cached_unrealized.as_mut() {
                cache.on_receive(s.realized_price, s.supply_state.value);
            }
        }
    }

    pub fn decrement(&mut self, supply: &SupplyState, price: Option<CentsUnsigned>) {
        match price {
            Some(p) => self.decrement_snapshot(&CostBasisSnapshot::from_utxo(p, supply)),
            None => self.supply -= supply,
        }
    }

    pub fn decrement_snapshot(&mut self, s: &CostBasisSnapshot) {
        self.supply -= &s.supply_state;

        if s.supply_state.value > Sats::ZERO
            && let Some(realized) = self.realized.as_mut()
        {
            realized.decrement_snapshot(s.price_sats, s.investor_cap);
            self.cost_basis_data.as_mut().unwrap().decrement(
                s.realized_price,
                s.supply_state.value,
                s.price_sats,
                s.investor_cap,
            );

            if let Some(cache) = self.cached_unrealized.as_mut() {
                cache.on_send(s.realized_price, s.supply_state.value);
            }
        }
    }

    pub fn receive_utxo(&mut self, supply: &SupplyState, price: Option<CentsUnsigned>) {
        self.supply += supply;

        if supply.value > Sats::ZERO
            && let Some(realized) = self.realized.as_mut()
        {
            let price = price.unwrap();
            let sats = supply.value;

            // Compute once using typed values
            let price_sats = CentsSats::from_price_sats(price, sats);
            let investor_cap = price_sats.to_investor_cap(price);

            realized.receive(price, sats);

            self.cost_basis_data.as_mut().unwrap().increment(
                price,
                sats,
                price_sats,
                investor_cap,
            );

            if let Some(cache) = self.cached_unrealized.as_mut() {
                cache.on_receive(price, sats);
            }
        }
    }

    pub fn receive_address(
        &mut self,
        supply: &SupplyState,
        price: CentsUnsigned,
        current: &CostBasisSnapshot,
        prev: &CostBasisSnapshot,
    ) {
        self.supply += supply;

        if supply.value > Sats::ZERO
            && let Some(realized) = self.realized.as_mut()
        {
            realized.receive(price, supply.value);

            if current.supply_state.value.is_not_zero() {
                self.cost_basis_data.as_mut().unwrap().increment(
                    current.realized_price,
                    current.supply_state.value,
                    current.price_sats,
                    current.investor_cap,
                );

                if let Some(cache) = self.cached_unrealized.as_mut() {
                    cache.on_receive(current.realized_price, current.supply_state.value);
                }
            }

            if prev.supply_state.value.is_not_zero() {
                self.cost_basis_data.as_mut().unwrap().decrement(
                    prev.realized_price,
                    prev.supply_state.value,
                    prev.price_sats,
                    prev.investor_cap,
                );

                if let Some(cache) = self.cached_unrealized.as_mut() {
                    cache.on_send(prev.realized_price, prev.supply_state.value);
                }
            }
        }
    }

    pub fn send_utxo(
        &mut self,
        supply: &SupplyState,
        current_price: Option<CentsUnsigned>,
        prev_price: Option<CentsUnsigned>,
        ath: Option<CentsUnsigned>,
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

            if let Some(realized) = self.realized.as_mut() {
                let cp = current_price.unwrap();
                let pp = prev_price.unwrap();
                let ath_price = ath.unwrap();
                let sats = supply.value;

                // Compute ONCE using typed values
                let current_ps = CentsSats::from_price_sats(cp, sats);
                let prev_ps = CentsSats::from_price_sats(pp, sats);
                let ath_ps = CentsSats::from_price_sats(ath_price, sats);
                let prev_investor_cap = prev_ps.to_investor_cap(pp);

                realized.send(current_ps, prev_ps, ath_ps, prev_investor_cap);

                self.cost_basis_data.as_mut().unwrap().decrement(
                    pp,
                    sats,
                    prev_ps,
                    prev_investor_cap,
                );

                if let Some(cache) = self.cached_unrealized.as_mut() {
                    cache.on_send(pp, sats);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn send_address(
        &mut self,
        supply: &SupplyState,
        current_price: CentsUnsigned,
        prev_price: CentsUnsigned,
        ath: CentsUnsigned,
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

            if let Some(realized) = self.realized.as_mut() {
                let sats = supply.value;

                // Compute once for realized.send using typed values
                let current_ps = CentsSats::from_price_sats(current_price, sats);
                let prev_ps = CentsSats::from_price_sats(prev_price, sats);
                let ath_ps = CentsSats::from_price_sats(ath, sats);
                let prev_investor_cap = prev_ps.to_investor_cap(prev_price);

                realized.send(current_ps, prev_ps, ath_ps, prev_investor_cap);

                if current.supply_state.value.is_not_zero() {
                    self.cost_basis_data.as_mut().unwrap().increment(
                        current.realized_price,
                        current.supply_state.value,
                        current.price_sats,
                        current.investor_cap,
                    );

                    if let Some(cache) = self.cached_unrealized.as_mut() {
                        cache.on_receive(current.realized_price, current.supply_state.value);
                    }
                }

                if prev.supply_state.value.is_not_zero() {
                    self.cost_basis_data.as_mut().unwrap().decrement(
                        prev.realized_price,
                        prev.supply_state.value,
                        prev.price_sats,
                        prev.investor_cap,
                    );

                    if let Some(cache) = self.cached_unrealized.as_mut() {
                        cache.on_send(prev.realized_price, prev.supply_state.value);
                    }
                }
            }
        }
    }

    pub fn compute_percentiles(&self) -> Option<Percentiles> {
        self.cost_basis_data.as_ref()?.compute_percentiles()
    }

    pub fn compute_unrealized_states(
        &mut self,
        height_price: CentsUnsigned,
        date_price: Option<CentsUnsigned>,
    ) -> (UnrealizedState, Option<UnrealizedState>) {
        let cost_basis_data = match self.cost_basis_data.as_ref() {
            Some(p) if !p.is_empty() => p,
            _ => return (UnrealizedState::ZERO, date_price.map(|_| UnrealizedState::ZERO)),
        };

        let date_state = date_price.map(|date_price| {
            CachedUnrealizedState::compute_full_standalone(date_price.into(), cost_basis_data)
        });

        let height_state = if let Some(cache) = self.cached_unrealized.as_mut() {
            cache.get_at_price(height_price, cost_basis_data)
        } else {
            let cache = CachedUnrealizedState::compute_fresh(height_price, cost_basis_data);
            let state = cache.current_state();
            self.cached_unrealized = Some(cache);
            state
        };

        (height_state, date_state)
    }

    pub fn write(&mut self, height: Height, cleanup: bool) -> Result<()> {
        if let Some(p) = self.cost_basis_data.as_mut() {
            p.write(height, cleanup)?;
        }
        Ok(())
    }

    pub fn min_price(&self) -> Option<CentsUnsigned> {
        self.cost_basis_data.as_ref()?.first_key_value().map(|(k, _)| k.into())
    }

    pub fn max_price(&self) -> Option<CentsUnsigned> {
        self.cost_basis_data.as_ref()?.last_key_value().map(|(k, _)| k.into())
    }

    pub fn cost_basis_data_iter(
        &self,
    ) -> Option<impl Iterator<Item = (CentsUnsigned, &Sats)>> {
        self.cost_basis_data.as_ref().map(|p| p.iter().map(|(k, v)| (k.into(), v)))
    }
}
