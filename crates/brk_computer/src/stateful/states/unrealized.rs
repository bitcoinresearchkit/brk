use std::ops::Bound;

use brk_types::{Dollars, Sats};
use vecdb::CheckedSub;

use super::PriceToAmount;

#[derive(Debug, Default, Clone)]
pub struct UnrealizedState {
    pub supply_in_profit: Sats,
    pub supply_in_loss: Sats,
    pub unrealized_profit: Dollars,
    pub unrealized_loss: Dollars,
}

impl UnrealizedState {
    pub const NAN: Self = Self {
        supply_in_profit: Sats::ZERO,
        supply_in_loss: Sats::ZERO,
        unrealized_profit: Dollars::NAN,
        unrealized_loss: Dollars::NAN,
    };

    pub const ZERO: Self = Self {
        supply_in_profit: Sats::ZERO,
        supply_in_loss: Sats::ZERO,
        unrealized_profit: Dollars::ZERO,
        unrealized_loss: Dollars::ZERO,
    };
}

/// Cached unrealized state for O(k) incremental updates.
/// k = number of entries in price flip range (typically tiny).
#[derive(Debug, Clone)]
pub struct CachedUnrealizedState {
    pub state: UnrealizedState,
    at_price: Dollars,
}

impl CachedUnrealizedState {
    /// Create new cache by computing from scratch. O(n).
    pub fn compute_fresh(price: Dollars, price_to_amount: &PriceToAmount) -> Self {
        let state = Self::compute_full_standalone(price, price_to_amount);
        Self {
            state,
            at_price: price,
        }
    }

    /// Get unrealized state at new_price. O(k) where k = flip range size.
    pub fn get_at_price(
        &mut self,
        new_price: Dollars,
        price_to_amount: &PriceToAmount,
    ) -> &UnrealizedState {
        if new_price != self.at_price {
            self.update_for_price_change(new_price, price_to_amount);
        }
        &self.state
    }

    /// Update cached state when a receive happens.
    /// Determines profit/loss classification relative to cached price.
    pub fn on_receive(&mut self, purchase_price: Dollars, sats: Sats) {
        if purchase_price <= self.at_price {
            self.state.supply_in_profit += sats;
            if purchase_price < self.at_price {
                let diff = self.at_price.checked_sub(purchase_price).unwrap();
                self.state.unrealized_profit += diff * sats;
            }
        } else {
            self.state.supply_in_loss += sats;
            let diff = purchase_price.checked_sub(self.at_price).unwrap();
            self.state.unrealized_loss += diff * sats;
        }
    }

    /// Update cached state when a send happens from historical price.
    pub fn on_send(&mut self, historical_price: Dollars, sats: Sats) {
        if historical_price <= self.at_price {
            // Was in profit
            self.state.supply_in_profit -= sats;
            if historical_price < self.at_price {
                let diff = self.at_price.checked_sub(historical_price).unwrap();
                let profit_removed = diff * sats;
                self.state.unrealized_profit = self
                    .state
                    .unrealized_profit
                    .checked_sub(profit_removed)
                    .unwrap_or(Dollars::ZERO);
            }
        } else {
            // Was in loss
            self.state.supply_in_loss -= sats;
            let diff = historical_price.checked_sub(self.at_price).unwrap();
            let loss_removed = diff * sats;
            self.state.unrealized_loss = self
                .state
                .unrealized_loss
                .checked_sub(loss_removed)
                .unwrap_or(Dollars::ZERO);
        }
    }

    /// Incremental update for price change. O(k) where k = entries in flip range.
    fn update_for_price_change(&mut self, new_price: Dollars, price_to_amount: &PriceToAmount) {
        let old_price = self.at_price;
        let delta_f64 = f64::from(new_price) - f64::from(old_price);

        // Update profit/loss for entries that DON'T flip
        // Profit changes by delta * supply_in_profit
        // Loss changes by -delta * supply_in_loss
        if delta_f64 > 0.0 {
            // Price went up: profits increase, losses decrease
            self.state.unrealized_profit += Dollars::from(delta_f64) * self.state.supply_in_profit;
            let loss_decrease = Dollars::from(delta_f64) * self.state.supply_in_loss;
            self.state.unrealized_loss = self
                .state
                .unrealized_loss
                .checked_sub(loss_decrease)
                .unwrap_or(Dollars::ZERO);
        } else if delta_f64 < 0.0 {
            // Price went down: profits decrease, losses increase
            let profit_decrease = Dollars::from(-delta_f64) * self.state.supply_in_profit;
            self.state.unrealized_profit = self
                .state
                .unrealized_profit
                .checked_sub(profit_decrease)
                .unwrap_or(Dollars::ZERO);
            self.state.unrealized_loss += Dollars::from(-delta_f64) * self.state.supply_in_loss;
        }

        // Handle flipped entries (only iterate the small range between prices)
        if new_price > old_price {
            // Price went up: entries where old < price <= new flip from loss to profit
            for (&price, &sats) in
                price_to_amount.range((Bound::Excluded(old_price), Bound::Included(new_price)))
            {
                // Move from loss to profit
                self.state.supply_in_loss -= sats;
                self.state.supply_in_profit += sats;

                // Undo the loss adjustment applied above for this entry
                // We decreased loss by delta * sats, but this entry should be removed entirely
                // Original loss: (price - old_price) * sats
                // After global adjustment: original - delta * sats (negative, wrong)
                // Correct: 0 (removed from loss)
                // Correction: add back delta * sats, then add original loss
                let delta_adj = Dollars::from(delta_f64) * sats;
                self.state.unrealized_loss += delta_adj;
                if price > old_price {
                    let original_loss = price.checked_sub(old_price).unwrap() * sats;
                    self.state.unrealized_loss += original_loss;
                }

                // Undo the profit adjustment applied above for this entry
                // We increased profit by delta * sats, but this entry was not in profit before
                // Correct profit: (new_price - price) * sats
                // Correction: subtract delta * sats, add correct profit
                let profit_adj = Dollars::from(delta_f64) * sats;
                self.state.unrealized_profit = self
                    .state
                    .unrealized_profit
                    .checked_sub(profit_adj)
                    .unwrap_or(Dollars::ZERO);
                if new_price > price {
                    let correct_profit = new_price.checked_sub(price).unwrap() * sats;
                    self.state.unrealized_profit += correct_profit;
                }
            }
        } else if new_price < old_price {
            // Price went down: entries where new < price <= old flip from profit to loss
            for (&price, &sats) in
                price_to_amount.range((Bound::Excluded(new_price), Bound::Included(old_price)))
            {
                // Move from profit to loss
                self.state.supply_in_profit -= sats;
                self.state.supply_in_loss += sats;

                // Undo the profit adjustment applied above for this entry
                let delta_adj = Dollars::from(-delta_f64) * sats;
                self.state.unrealized_profit += delta_adj;
                if old_price > price {
                    let original_profit = old_price.checked_sub(price).unwrap() * sats;
                    self.state.unrealized_profit += original_profit;
                }

                // Undo the loss adjustment applied above for this entry
                let loss_adj = Dollars::from(-delta_f64) * sats;
                self.state.unrealized_loss = self
                    .state
                    .unrealized_loss
                    .checked_sub(loss_adj)
                    .unwrap_or(Dollars::ZERO);
                if price > new_price {
                    let correct_loss = price.checked_sub(new_price).unwrap() * sats;
                    self.state.unrealized_loss += correct_loss;
                }
            }
        }

        self.at_price = new_price;
    }

    /// Full computation from scratch (no cache). O(n).
    pub fn compute_full_standalone(
        current_price: Dollars,
        price_to_amount: &PriceToAmount,
    ) -> UnrealizedState {
        let mut state = UnrealizedState::ZERO;

        for (&price, &sats) in price_to_amount.iter() {
            if price <= current_price {
                state.supply_in_profit += sats;
                if price < current_price {
                    let diff = current_price.checked_sub(price).unwrap();
                    state.unrealized_profit += diff * sats;
                }
            } else {
                state.supply_in_loss += sats;
                let diff = price.checked_sub(current_price).unwrap();
                state.unrealized_loss += diff * sats;
            }
        }

        state
    }
}
