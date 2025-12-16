//! Cohort state tracking during computation.
//!
//! This state is maintained in memory during block processing and periodically flushed.

use std::cmp::Ordering;
use std::path::Path;

use brk_error::Result;
use brk_types::{CheckedSub, Dollars, Height, Sats};

use crate::{
    PriceToAmount, RealizedState, SupplyState, UnrealizedState,
    grouped::{PERCENTILES, PERCENTILES_LEN},
    utils::OptionExt,
};

/// State tracked for each cohort during computation.
#[derive(Clone)]
pub struct CohortState {
    /// Current supply in this cohort
    pub supply: SupplyState,

    /// Realized cap and profit/loss (requires price data)
    pub realized: Option<RealizedState>,

    /// Amount sent in current block
    pub sent: Sats,

    /// Satoshi-blocks destroyed (supply * blocks_old when spent)
    pub satblocks_destroyed: Sats,

    /// Satoshi-days destroyed (supply * days_old when spent)
    pub satdays_destroyed: Sats,

    /// Price distribution for percentile calculations (requires price data)
    price_to_amount: Option<PriceToAmount>,
}

impl CohortState {
    /// Create new cohort state.
    pub fn new(path: &Path, name: &str, compute_dollars: bool) -> Self {
        Self {
            supply: SupplyState::default(),
            realized: compute_dollars.then_some(RealizedState::NAN),
            sent: Sats::ZERO,
            satblocks_destroyed: Sats::ZERO,
            satdays_destroyed: Sats::ZERO,
            price_to_amount: compute_dollars.then_some(PriceToAmount::create(path, name)),
        }
    }

    /// Import state from checkpoint.
    pub fn import_at_or_before(&mut self, height: Height) -> Result<Height> {
        match self.price_to_amount.as_mut() {
            Some(p) => p.import_at_or_before(height),
            None => Ok(height),
        }
    }

    /// Reset price_to_amount if needed (for starting fresh).
    pub fn reset_price_to_amount(&mut self) -> Result<()> {
        if let Some(p) = self.price_to_amount.as_mut() {
            p.clean()?;
            p.init();
        }
        Ok(())
    }

    /// Reset per-block values before processing next block.
    pub fn reset_block_values(&mut self) {
        self.sent = Sats::ZERO;
        self.satdays_destroyed = Sats::ZERO;
        self.satblocks_destroyed = Sats::ZERO;
        if let Some(realized) = self.realized.as_mut() {
            realized.reset_single_iteration_values();
        }
    }

    /// Add supply to this cohort (e.g., when UTXO ages into cohort).
    pub fn increment(&mut self, supply: &SupplyState, price: Option<Dollars>) {
        self.supply += supply;

        if supply.value > Sats::ZERO
            && let Some(realized) = self.realized.as_mut() {
                let price = price.unwrap();
                realized.increment(supply, price);
                self.price_to_amount.as_mut().unwrap().increment(price, supply);
            }
    }

    /// Remove supply from this cohort (e.g., when UTXO ages out of cohort).
    pub fn decrement(&mut self, supply: &SupplyState, price: Option<Dollars>) {
        self.supply -= supply;

        if supply.value > Sats::ZERO
            && let Some(realized) = self.realized.as_mut() {
                let price = price.unwrap();
                realized.decrement(supply, price);
                self.price_to_amount.as_mut().unwrap().decrement(price, supply);
            }
    }

    /// Process received output (new UTXO in cohort).
    pub fn receive(&mut self, supply: &SupplyState, price: Option<Dollars>) {
        self.supply += supply;

        if supply.value > Sats::ZERO
            && let Some(realized) = self.realized.as_mut() {
                let price = price.unwrap();
                realized.receive(supply, price);
                self.price_to_amount.as_mut().unwrap().increment(price, supply);
            }
    }

    /// Process spent input (UTXO leaving cohort).
    pub fn send(
        &mut self,
        supply: &SupplyState,
        current_price: Option<Dollars>,
        prev_price: Option<Dollars>,
        blocks_old: usize,
        days_old: f64,
        older_than_hour: bool,
    ) {
        if supply.utxo_count == 0 {
            return;
        }

        self.supply -= supply;

        if supply.value > Sats::ZERO {
            self.sent += supply.value;
            self.satblocks_destroyed += supply.value * blocks_old;
            self.satdays_destroyed +=
                Sats::from((u64::from(supply.value) as f64 * days_old).floor() as u64);

            if let Some(realized) = self.realized.as_mut() {
                let current_price = current_price.unwrap();
                let prev_price = prev_price.unwrap();
                realized.send(supply, current_price, prev_price, older_than_hour);
                self.price_to_amount.as_mut().unwrap().decrement(prev_price, supply);
            }
        }
    }

    /// Compute prices at percentile thresholds.
    pub fn compute_percentile_prices(&self) -> [Dollars; PERCENTILES_LEN] {
        let mut result = [Dollars::NAN; PERCENTILES_LEN];

        let price_to_amount = match self.price_to_amount.as_ref() {
            Some(p) => p,
            None => return result,
        };

        if price_to_amount.is_empty() || self.supply.value == Sats::ZERO {
            return result;
        }

        let total = u64::from(self.supply.value);
        let targets = PERCENTILES.map(|p| total * u64::from(p) / 100);

        let mut accumulated = 0u64;
        let mut pct_idx = 0;

        for (&price, &sats) in price_to_amount.iter() {
            accumulated += u64::from(sats);

            while pct_idx < PERCENTILES_LEN && accumulated >= targets[pct_idx] {
                result[pct_idx] = price;
                pct_idx += 1;
            }

            if pct_idx >= PERCENTILES_LEN {
                break;
            }
        }

        result
    }

    /// Compute unrealized profit/loss at current price.
    pub fn compute_unrealized(
        &self,
        height_price: Dollars,
        date_price: Option<Dollars>,
    ) -> (UnrealizedState, Option<UnrealizedState>) {
        let price_to_amount = match self.price_to_amount.as_ref() {
            Some(p) if !p.is_empty() => p,
            _ => return (UnrealizedState::NAN, date_price.map(|_| UnrealizedState::NAN)),
        };

        let mut height_state = UnrealizedState::ZERO;
        let mut date_state = date_price.map(|_| UnrealizedState::ZERO);

        for (&price, &sats) in price_to_amount.iter() {
            Self::update_unrealized(price, height_price, sats, &mut height_state);

            if let Some(date_price) = date_price {
                Self::update_unrealized(price, date_price, sats, date_state.um());
            }
        }

        (height_state, date_state)
    }

    fn update_unrealized(price: Dollars, current: Dollars, sats: Sats, state: &mut UnrealizedState) {
        match price.cmp(&current) {
            Ordering::Less | Ordering::Equal => {
                state.supply_in_profit += sats;
                if price < current && price > Dollars::ZERO && current > Dollars::ZERO {
                    state.unrealized_profit += current.checked_sub(price).unwrap() * sats;
                }
            }
            Ordering::Greater => {
                state.supply_in_loss += sats;
                if price > Dollars::ZERO && current > Dollars::ZERO {
                    state.unrealized_loss += price.checked_sub(current).unwrap() * sats;
                }
            }
        }
    }

    /// Flush state to disk at checkpoint.
    pub fn commit(&mut self, height: Height) -> Result<()> {
        if let Some(p) = self.price_to_amount.as_mut() {
            p.flush(height)?;
        }
        Ok(())
    }

    /// Get first (lowest) price in distribution.
    pub fn min_price(&self) -> Option<&Dollars> {
        self.price_to_amount.as_ref()?.first_key_value().map(|(k, _)| k)
    }

    /// Get last (highest) price in distribution.
    pub fn max_price(&self) -> Option<&Dollars> {
        self.price_to_amount.as_ref()?.last_key_value().map(|(k, _)| k)
    }
}
