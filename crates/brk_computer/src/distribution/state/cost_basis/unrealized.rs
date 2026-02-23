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

/// Internal cache state using u128 for raw cent*sat values.
/// This avoids rounding errors from premature division by ONE_BTC.
/// Division happens only when converting to UnrealizedState output.
#[derive(Debug, Default, Clone)]
struct CachedStateRaw {
    supply_in_profit: Sats,
    supply_in_loss: Sats,
    /// Raw value: sum of (price_cents * sats) for UTXOs in profit
    unrealized_profit: u128,
    /// Raw value: sum of (price_cents * sats) for UTXOs in loss
    unrealized_loss: u128,
    /// Raw value: sum of (price_cents * sats) for UTXOs in profit
    invested_capital_in_profit: u128,
    /// Raw value: sum of (price_cents * sats) for UTXOs in loss
    invested_capital_in_loss: u128,
    /// Raw value: sum of (price_cents² * sats) for UTXOs in profit
    investor_cap_in_profit: u128,
    /// Raw value: sum of (price_cents² * sats) for UTXOs in loss
    investor_cap_in_loss: u128,
}

impl CachedStateRaw {
    /// Convert raw values to final output by dividing by ONE_BTC.
    fn to_output(&self) -> UnrealizedState {
        UnrealizedState {
            supply_in_profit: self.supply_in_profit,
            supply_in_loss: self.supply_in_loss,
            unrealized_profit: Cents::new((self.unrealized_profit / Sats::ONE_BTC_U128) as u64),
            unrealized_loss: Cents::new((self.unrealized_loss / Sats::ONE_BTC_U128) as u64),
            invested_capital_in_profit: Cents::new(
                (self.invested_capital_in_profit / Sats::ONE_BTC_U128) as u64,
            ),
            invested_capital_in_loss: Cents::new(
                (self.invested_capital_in_loss / Sats::ONE_BTC_U128) as u64,
            ),
            investor_cap_in_profit_raw: self.investor_cap_in_profit,
            investor_cap_in_loss_raw: self.investor_cap_in_loss,
            invested_capital_in_profit_raw: self.invested_capital_in_profit,
            invested_capital_in_loss_raw: self.invested_capital_in_loss,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CachedUnrealizedState {
    state: CachedStateRaw,
    at_price: CentsCompact,
}

impl CachedUnrealizedState {
    pub(crate) fn compute_fresh(price: Cents, map: &CostBasisMap) -> Self {
        let price: CentsCompact = price.into();
        let state = Self::compute_raw(price, map);
        Self {
            state,
            at_price: price,
        }
    }

    /// Get the current cached state as output (without price update).
    pub(crate) fn current_state(&self) -> UnrealizedState {
        self.state.to_output()
    }

    pub(crate) fn get_at_price(&mut self, new_price: Cents, map: &CostBasisMap) -> UnrealizedState {
        let new_price: CentsCompact = new_price.into();
        if new_price != self.at_price {
            self.update_for_price_change(new_price, map);
        }
        self.state.to_output()
    }

    pub(crate) fn on_receive(&mut self, price: Cents, sats: Sats) {
        let price: CentsCompact = price.into();
        let sats_u128 = sats.as_u128();
        let price_u128 = price.as_u128();
        let invested_capital = price_u128 * sats_u128;
        let investor_cap = price_u128 * invested_capital;

        if price <= self.at_price {
            self.state.supply_in_profit += sats;
            self.state.invested_capital_in_profit += invested_capital;
            self.state.investor_cap_in_profit += investor_cap;
            if price < self.at_price {
                let diff = (self.at_price - price).as_u128();
                self.state.unrealized_profit += diff * sats_u128;
            }
        } else {
            self.state.supply_in_loss += sats;
            self.state.invested_capital_in_loss += invested_capital;
            self.state.investor_cap_in_loss += investor_cap;
            let diff = (price - self.at_price).as_u128();
            self.state.unrealized_loss += diff * sats_u128;
        }
    }

    pub(crate) fn on_send(&mut self, price: Cents, sats: Sats) {
        let price: CentsCompact = price.into();
        let sats_u128 = sats.as_u128();
        let price_u128 = price.as_u128();
        let invested_capital = price_u128 * sats_u128;
        let investor_cap = price_u128 * invested_capital;

        if price <= self.at_price {
            self.state.supply_in_profit -= sats;
            self.state.invested_capital_in_profit -= invested_capital;
            self.state.investor_cap_in_profit -= investor_cap;
            if price < self.at_price {
                let diff = (self.at_price - price).as_u128();
                self.state.unrealized_profit -= diff * sats_u128;
            }
        } else {
            self.state.supply_in_loss -= sats;
            self.state.invested_capital_in_loss -= invested_capital;
            self.state.investor_cap_in_loss -= investor_cap;
            let diff = (price - self.at_price).as_u128();
            self.state.unrealized_loss -= diff * sats_u128;
        }
    }

    fn update_for_price_change(&mut self, new_price: CentsCompact, map: &CostBasisMap) {
        let old_price = self.at_price;

        if new_price > old_price {
            let delta = (new_price - old_price).as_u128();

            // Save original supply for delta calculation (before crossing UTXOs move)
            let original_supply_in_profit = self.state.supply_in_profit.as_u128();

            // First, process UTXOs crossing from loss to profit
            // Range (old_price, new_price] means: old_price < price <= new_price
            for (&price, &sats) in
                map.range((Bound::Excluded(old_price), Bound::Included(new_price)))
            {
                let sats_u128 = sats.as_u128();
                let price_u128 = price.as_u128();
                let invested_capital = price_u128 * sats_u128;
                let investor_cap = price_u128 * invested_capital;

                // Move between buckets
                self.state.supply_in_loss -= sats;
                self.state.supply_in_profit += sats;
                self.state.invested_capital_in_loss -= invested_capital;
                self.state.invested_capital_in_profit += invested_capital;
                self.state.investor_cap_in_loss -= investor_cap;
                self.state.investor_cap_in_profit += investor_cap;

                // Remove their original contribution to unrealized_loss
                // (price > old_price is always true due to Bound::Excluded)
                let original_loss = (price - old_price).as_u128();
                self.state.unrealized_loss -= original_loss * sats_u128;

                // Add their new contribution to unrealized_profit (if not at boundary)
                if price < new_price {
                    let new_profit = (new_price - price).as_u128();
                    self.state.unrealized_profit += new_profit * sats_u128;
                }
            }

            // Apply delta to non-crossing UTXOs only
            // Non-crossing profit UTXOs: their profit increases by delta
            self.state.unrealized_profit += delta * original_supply_in_profit;
            // Non-crossing loss UTXOs: their loss decreases by delta
            let non_crossing_loss_sats = self.state.supply_in_loss.as_u128(); // Already excludes crossing
            self.state.unrealized_loss -= delta * non_crossing_loss_sats;
        } else if new_price < old_price {
            let delta = (old_price - new_price).as_u128();

            // Save original supply for delta calculation (before crossing UTXOs move)
            let original_supply_in_loss = self.state.supply_in_loss.as_u128();

            // First, process UTXOs crossing from profit to loss
            // Range (new_price, old_price] means: new_price < price <= old_price
            for (&price, &sats) in
                map.range((Bound::Excluded(new_price), Bound::Included(old_price)))
            {
                let sats_u128 = sats.as_u128();
                let price_u128 = price.as_u128();
                let invested_capital = price_u128 * sats_u128;
                let investor_cap = price_u128 * invested_capital;

                // Move between buckets
                self.state.supply_in_profit -= sats;
                self.state.supply_in_loss += sats;
                self.state.invested_capital_in_profit -= invested_capital;
                self.state.invested_capital_in_loss += invested_capital;
                self.state.investor_cap_in_profit -= investor_cap;
                self.state.investor_cap_in_loss += investor_cap;

                // Remove their original contribution to unrealized_profit (if not at boundary)
                if price < old_price {
                    let original_profit = (old_price - price).as_u128();
                    self.state.unrealized_profit -= original_profit * sats_u128;
                }

                // Add their new contribution to unrealized_loss
                // (price > new_price is always true due to Bound::Excluded)
                let new_loss = (price - new_price).as_u128();
                self.state.unrealized_loss += new_loss * sats_u128;
            }

            // Apply delta to non-crossing UTXOs only
            // Non-crossing loss UTXOs: their loss increases by delta
            self.state.unrealized_loss += delta * original_supply_in_loss;
            // Non-crossing profit UTXOs: their profit decreases by delta
            let non_crossing_profit_sats = self.state.supply_in_profit.as_u128(); // Already excludes crossing
            self.state.unrealized_profit -= delta * non_crossing_profit_sats;
        }

        self.at_price = new_price;
    }

    /// Compute raw cached state from the map.
    fn compute_raw(current_price: CentsCompact, map: &CostBasisMap) -> CachedStateRaw {
        let mut state = CachedStateRaw::default();

        for (&price, &sats) in map.iter() {
            let sats_u128 = sats.as_u128();
            let price_u128 = price.as_u128();
            let invested_capital = price_u128 * sats_u128;
            let investor_cap = price_u128 * invested_capital;

            if price <= current_price {
                state.supply_in_profit += sats;
                state.invested_capital_in_profit += invested_capital;
                state.investor_cap_in_profit += investor_cap;
                if price < current_price {
                    let diff = (current_price - price).as_u128();
                    state.unrealized_profit += diff * sats_u128;
                }
            } else {
                state.supply_in_loss += sats;
                state.invested_capital_in_loss += invested_capital;
                state.investor_cap_in_loss += investor_cap;
                let diff = (price - current_price).as_u128();
                state.unrealized_loss += diff * sats_u128;
            }
        }

        state
    }

    /// Compute final UnrealizedState directly (not cached).
    /// Used for date_state which doesn't use the cache.
    pub(crate) fn compute_full_standalone(
        current_price: CentsCompact,
        map: &CostBasisMap,
    ) -> UnrealizedState {
        Self::compute_raw(current_price, map).to_output()
    }
}
