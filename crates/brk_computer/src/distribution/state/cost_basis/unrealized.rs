use std::ops::Bound;

use brk_types::{Cents, CentsCompact, Sats};

use super::CostBasisMap;

#[derive(Debug, Default, Clone)]
pub struct UnrealizedState {
    pub supply_in_profit: Sats,
    pub supply_in_loss: Sats,
    pub unrealized_profit: Cents,
    pub unrealized_loss: Cents,
    pub invested_capital_in_profit: Cents,
    pub invested_capital_in_loss: Cents,
    /// Raw Σ(price² × sats) for UTXOs in profit. Used for aggregation.
    pub investor_cap_in_profit_raw: u128,
    /// Raw Σ(price² × sats) for UTXOs in loss. Used for aggregation.
    pub investor_cap_in_loss_raw: u128,
    /// Raw Σ(price × sats) for UTXOs in profit. Used for aggregation.
    pub invested_capital_in_profit_raw: u128,
    /// Raw Σ(price × sats) for UTXOs in loss. Used for aggregation.
    pub invested_capital_in_loss_raw: u128,
}

impl UnrealizedState {
    pub const ZERO: Self = Self {
        supply_in_profit: Sats::ZERO,
        supply_in_loss: Sats::ZERO,
        unrealized_profit: Cents::ZERO,
        unrealized_loss: Cents::ZERO,
        invested_capital_in_profit: Cents::ZERO,
        invested_capital_in_loss: Cents::ZERO,
        investor_cap_in_profit_raw: 0,
        investor_cap_in_loss_raw: 0,
        invested_capital_in_profit_raw: 0,
        invested_capital_in_loss_raw: 0,
    };
}

/// Core cache state: supply + unrealized profit/loss only (64 bytes, 1 cache line).
#[derive(Debug, Default, Clone)]
pub struct WithoutCapital {
    pub(crate) supply_in_profit: Sats,
    pub(crate) supply_in_loss: Sats,
    pub(crate) unrealized_profit: u128,
    pub(crate) unrealized_loss: u128,
}

/// Full cache state: core + invested capital + investor cap (128 bytes, 2 cache lines).
#[derive(Debug, Default, Clone)]
pub struct WithCapital {
    core: WithoutCapital,
    invested_capital_in_profit: u128,
    invested_capital_in_loss: u128,
    investor_cap_in_profit: u128,
    investor_cap_in_loss: u128,
}

#[inline(always)]
fn div_btc(raw: u128) -> Cents {
    if raw == 0 {
        Cents::ZERO
    } else {
        Cents::new((raw / Sats::ONE_BTC_U128) as u64)
    }
}

/// Trait for accumulating profit/loss across BTreeMap entries.
/// `WithoutCapital` skips capital tracking; `WithCapital` tracks all fields.
pub trait Accumulate: Default + Clone + Send + Sync + 'static {
    fn to_output(&self) -> UnrealizedState;
    fn core(&self) -> &WithoutCapital;
    fn core_mut(&mut self) -> &mut WithoutCapital;

    fn supply_in_profit(&self) -> Sats { self.core().supply_in_profit }
    fn supply_in_loss(&self) -> Sats { self.core().supply_in_loss }
    fn unrealized_profit(&mut self) -> &mut u128 { &mut self.core_mut().unrealized_profit }
    fn unrealized_loss(&mut self) -> &mut u128 { &mut self.core_mut().unrealized_loss }

    fn accumulate_profit(&mut self, price_u128: u128, sats: Sats);
    fn accumulate_loss(&mut self, price_u128: u128, sats: Sats);
    fn deaccumulate_profit(&mut self, price_u128: u128, sats: Sats);
    fn deaccumulate_loss(&mut self, price_u128: u128, sats: Sats);
}

impl Accumulate for WithoutCapital {
    fn to_output(&self) -> UnrealizedState {
        UnrealizedState {
            supply_in_profit: self.supply_in_profit,
            supply_in_loss: self.supply_in_loss,
            unrealized_profit: div_btc(self.unrealized_profit),
            unrealized_loss: div_btc(self.unrealized_loss),
            ..UnrealizedState::ZERO
        }
    }

    fn core(&self) -> &WithoutCapital { self }
    fn core_mut(&mut self) -> &mut WithoutCapital { self }

    #[inline(always)]
    fn accumulate_profit(&mut self, _price_u128: u128, sats: Sats) {
        self.supply_in_profit += sats;
    }
    #[inline(always)]
    fn accumulate_loss(&mut self, _price_u128: u128, sats: Sats) {
        self.supply_in_loss += sats;
    }
    #[inline(always)]
    fn deaccumulate_profit(&mut self, _price_u128: u128, sats: Sats) {
        self.supply_in_profit -= sats;
    }
    #[inline(always)]
    fn deaccumulate_loss(&mut self, _price_u128: u128, sats: Sats) {
        self.supply_in_loss -= sats;
    }
}

impl Accumulate for WithCapital {
    fn to_output(&self) -> UnrealizedState {
        UnrealizedState {
            invested_capital_in_profit: div_btc(self.invested_capital_in_profit),
            invested_capital_in_loss: div_btc(self.invested_capital_in_loss),
            investor_cap_in_profit_raw: self.investor_cap_in_profit,
            investor_cap_in_loss_raw: self.investor_cap_in_loss,
            invested_capital_in_profit_raw: self.invested_capital_in_profit,
            invested_capital_in_loss_raw: self.invested_capital_in_loss,
            ..Accumulate::to_output(&self.core)
        }
    }

    fn core(&self) -> &WithoutCapital { &self.core }
    fn core_mut(&mut self) -> &mut WithoutCapital { &mut self.core }

    #[inline(always)]
    fn accumulate_profit(&mut self, price_u128: u128, sats: Sats) {
        self.core.supply_in_profit += sats;
        let invested_capital = price_u128 * sats.as_u128();
        self.invested_capital_in_profit += invested_capital;
        self.investor_cap_in_profit += price_u128 * invested_capital;
    }
    #[inline(always)]
    fn accumulate_loss(&mut self, price_u128: u128, sats: Sats) {
        self.core.supply_in_loss += sats;
        let invested_capital = price_u128 * sats.as_u128();
        self.invested_capital_in_loss += invested_capital;
        self.investor_cap_in_loss += price_u128 * invested_capital;
    }
    #[inline(always)]
    fn deaccumulate_profit(&mut self, price_u128: u128, sats: Sats) {
        self.core.supply_in_profit -= sats;
        let invested_capital = price_u128 * sats.as_u128();
        self.invested_capital_in_profit -= invested_capital;
        self.investor_cap_in_profit -= price_u128 * invested_capital;
    }
    #[inline(always)]
    fn deaccumulate_loss(&mut self, price_u128: u128, sats: Sats) {
        self.core.supply_in_loss -= sats;
        let invested_capital = price_u128 * sats.as_u128();
        self.invested_capital_in_loss -= invested_capital;
        self.investor_cap_in_loss -= price_u128 * invested_capital;
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CachedUnrealizedState<S: Accumulate> {
    state: S,
    at_price: CentsCompact,
    cached_output: Option<UnrealizedState>,
}

impl<S: Accumulate> CachedUnrealizedState<S> {
    pub(crate) fn compute_fresh(price: Cents, map: &CostBasisMap) -> Self {
        let price: CentsCompact = price.into();
        let state = Self::compute_raw(price, map);
        Self {
            state,
            at_price: price,
            cached_output: None,
        }
    }

    pub(crate) fn current_state(&self) -> UnrealizedState {
        self.state.to_output()
    }

    pub(crate) fn get_at_price(&mut self, new_price: Cents, map: &CostBasisMap) -> UnrealizedState {
        let new_price: CentsCompact = new_price.into();
        if new_price != self.at_price {
            self.update_for_price_change(new_price, map);
            self.cached_output = None;
        }
        if let Some(ref output) = self.cached_output {
            return output.clone();
        }
        self.cached_output.insert(self.state.to_output()).clone()
    }

    pub(crate) fn on_receive(&mut self, price: Cents, sats: Sats) {
        self.cached_output = None;
        let price: CentsCompact = price.into();
        let sats_u128 = sats.as_u128();
        let price_u128 = price.as_u128();

        if price <= self.at_price {
            self.state.accumulate_profit(price_u128, sats);
            if price < self.at_price {
                let diff = (self.at_price - price).as_u128();
                *self.state.unrealized_profit() += diff * sats_u128;
            }
        } else {
            self.state.accumulate_loss(price_u128, sats);
            let diff = (price - self.at_price).as_u128();
            *self.state.unrealized_loss() += diff * sats_u128;
        }
    }

    pub(crate) fn on_send(&mut self, price: Cents, sats: Sats) {
        self.cached_output = None;
        let price: CentsCompact = price.into();
        let sats_u128 = sats.as_u128();
        let price_u128 = price.as_u128();

        if price <= self.at_price {
            self.state.deaccumulate_profit(price_u128, sats);
            if price < self.at_price {
                let diff = (self.at_price - price).as_u128();
                *self.state.unrealized_profit() -= diff * sats_u128;
            }
        } else {
            self.state.deaccumulate_loss(price_u128, sats);
            let diff = (price - self.at_price).as_u128();
            *self.state.unrealized_loss() -= diff * sats_u128;
        }
    }

    fn update_for_price_change(&mut self, new_price: CentsCompact, map: &CostBasisMap) {
        let old_price = self.at_price;

        if new_price > old_price {
            let delta = (new_price - old_price).as_u128();
            let original_supply_in_profit = self.state.supply_in_profit().as_u128();

            for (&price, &sats) in
                map.range((Bound::Excluded(old_price), Bound::Included(new_price)))
            {
                let sats_u128 = sats.as_u128();
                let price_u128 = price.as_u128();

                self.state.deaccumulate_loss(price_u128, sats);
                self.state.accumulate_profit(price_u128, sats);

                let original_loss = (price - old_price).as_u128();
                *self.state.unrealized_loss() -= original_loss * sats_u128;

                if price < new_price {
                    let new_profit = (new_price - price).as_u128();
                    *self.state.unrealized_profit() += new_profit * sats_u128;
                }
            }

            *self.state.unrealized_profit() += delta * original_supply_in_profit;
            let non_crossing_loss_sats = self.state.supply_in_loss().as_u128();
            *self.state.unrealized_loss() -= delta * non_crossing_loss_sats;
        } else if new_price < old_price {
            let delta = (old_price - new_price).as_u128();
            let original_supply_in_loss = self.state.supply_in_loss().as_u128();

            for (&price, &sats) in
                map.range((Bound::Excluded(new_price), Bound::Included(old_price)))
            {
                let sats_u128 = sats.as_u128();
                let price_u128 = price.as_u128();

                self.state.deaccumulate_profit(price_u128, sats);
                self.state.accumulate_loss(price_u128, sats);

                if price < old_price {
                    let original_profit = (old_price - price).as_u128();
                    *self.state.unrealized_profit() -= original_profit * sats_u128;
                }

                let new_loss = (price - new_price).as_u128();
                *self.state.unrealized_loss() += new_loss * sats_u128;
            }

            *self.state.unrealized_loss() += delta * original_supply_in_loss;
            let non_crossing_profit_sats = self.state.supply_in_profit().as_u128();
            *self.state.unrealized_profit() -= delta * non_crossing_profit_sats;
        }

        self.at_price = new_price;
    }

    fn compute_raw(current_price: CentsCompact, map: &CostBasisMap) -> S {
        let mut state = S::default();

        for (&price, &sats) in map.iter() {
            let sats_u128 = sats.as_u128();
            let price_u128 = price.as_u128();

            if price <= current_price {
                state.accumulate_profit(price_u128, sats);
                if price < current_price {
                    let diff = (current_price - price).as_u128();
                    *state.unrealized_profit() += diff * sats_u128;
                }
            } else {
                state.accumulate_loss(price_u128, sats);
                let diff = (price - current_price).as_u128();
                *state.unrealized_loss() += diff * sats_u128;
            }
        }

        state
    }
}
