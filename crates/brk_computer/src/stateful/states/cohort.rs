use std::path::Path;

use brk_error::Result;
use brk_types::{Dollars, Height, Sats};

use crate::grouped::PERCENTILES_LEN;

use super::{CachedUnrealizedState, PriceToAmount, RealizedState, SupplyState, UnrealizedState};

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

    /// Cached unrealized state for O(k) incremental updates.
    cached_unrealized: Option<CachedUnrealizedState>,
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
            cached_unrealized: None,
        }
    }

    /// Import state from checkpoint.
    pub fn import_at_or_before(&mut self, height: Height) -> Result<Height> {
        // Invalidate cache when importing new data
        self.cached_unrealized = None;

        match self.price_to_amount.as_mut() {
            Some(p) => p.import_at_or_before(height),
            None => Ok(height),
        }
    }

    /// Reset price_to_amount if needed (for starting fresh).
    pub fn reset_price_to_amount_if_needed(&mut self) -> Result<()> {
        if let Some(p) = self.price_to_amount.as_mut() {
            p.clean()?;
            p.init();
        }
        // Invalidate cache when data is reset
        self.cached_unrealized = None;
        Ok(())
    }

    /// Apply pending price_to_amount updates. Must be called before reads.
    pub fn apply_pending(&mut self) {
        if let Some(p) = self.price_to_amount.as_mut() {
            p.apply_pending();
        }
    }

    /// Get first (lowest) price entry in distribution.
    pub fn price_to_amount_first_key_value(&self) -> Option<(Dollars, &Sats)> {
        self.price_to_amount.as_ref()?.first_key_value()
    }

    /// Get last (highest) price entry in distribution.
    pub fn price_to_amount_last_key_value(&self) -> Option<(Dollars, &Sats)> {
        self.price_to_amount.as_ref()?.last_key_value()
    }

    /// Reset per-block values before processing next block.
    pub fn reset_single_iteration_values(&mut self) {
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
            && let Some(realized) = self.realized.as_mut()
        {
            let price = price.unwrap();
            realized.increment(supply, price);
            self.price_to_amount
                .as_mut()
                .unwrap()
                .increment(price, supply);

            // Update cache for added supply
            if let Some(cache) = self.cached_unrealized.as_mut() {
                cache.on_receive(price, supply.value);
            }
        }
    }

    /// Add supply with pre-computed realized cap (for address cohorts).
    pub fn increment_(
        &mut self,
        supply: &SupplyState,
        realized_cap: Dollars,
        realized_price: Dollars,
    ) {
        self.supply += supply;

        if supply.value > Sats::ZERO
            && let Some(realized) = self.realized.as_mut()
        {
            realized.increment_(realized_cap);
            self.price_to_amount
                .as_mut()
                .unwrap()
                .increment(realized_price, supply);

            // Update cache for added supply
            if let Some(cache) = self.cached_unrealized.as_mut() {
                cache.on_receive(realized_price, supply.value);
            }
        }
    }

    /// Remove supply from this cohort (e.g., when UTXO ages out of cohort).
    pub fn decrement(&mut self, supply: &SupplyState, price: Option<Dollars>) {
        self.supply -= supply;

        if supply.value > Sats::ZERO
            && let Some(realized) = self.realized.as_mut()
        {
            let price = price.unwrap();
            realized.decrement(supply, price);
            self.price_to_amount
                .as_mut()
                .unwrap()
                .decrement(price, supply);

            // Update cache for removed supply
            if let Some(cache) = self.cached_unrealized.as_mut() {
                cache.on_send(price, supply.value);
            }
        }
    }

    /// Remove supply with pre-computed realized cap (for address cohorts).
    pub fn decrement_(
        &mut self,
        supply: &SupplyState,
        realized_cap: Dollars,
        realized_price: Dollars,
    ) {
        self.supply -= supply;

        if supply.value > Sats::ZERO
            && let Some(realized) = self.realized.as_mut()
        {
            realized.decrement_(realized_cap);
            self.price_to_amount
                .as_mut()
                .unwrap()
                .decrement(realized_price, supply);

            // Update cache for removed supply
            if let Some(cache) = self.cached_unrealized.as_mut() {
                cache.on_send(realized_price, supply.value);
            }
        }
    }

    /// Process received output (new UTXO in cohort).
    pub fn receive(&mut self, supply: &SupplyState, price: Option<Dollars>) {
        self.receive_(supply, price, price.map(|price| (price, supply)), None);
    }

    /// Process received output with custom price_to_amount updates (for address cohorts).
    pub fn receive_(
        &mut self,
        supply: &SupplyState,
        price: Option<Dollars>,
        price_to_amount_increment: Option<(Dollars, &SupplyState)>,
        price_to_amount_decrement: Option<(Dollars, &SupplyState)>,
    ) {
        self.supply += supply;

        if supply.value > Sats::ZERO
            && let Some(realized) = self.realized.as_mut()
        {
            let price = price.unwrap();
            realized.receive(supply, price);

            if let Some((price, supply)) = price_to_amount_increment
                && supply.value.is_not_zero()
            {
                self.price_to_amount
                    .as_mut()
                    .unwrap()
                    .increment(price, supply);

                // Update cache for added supply
                if let Some(cache) = self.cached_unrealized.as_mut() {
                    cache.on_receive(price, supply.value);
                }
            }

            if let Some((price, supply)) = price_to_amount_decrement
                && supply.value.is_not_zero()
            {
                self.price_to_amount
                    .as_mut()
                    .unwrap()
                    .decrement(price, supply);

                // Update cache for removed supply
                if let Some(cache) = self.cached_unrealized.as_mut() {
                    cache.on_send(price, supply.value);
                }
            }
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
        self.send_(
            supply,
            current_price,
            prev_price,
            blocks_old,
            days_old,
            older_than_hour,
            None,
            prev_price.map(|prev_price| (prev_price, supply)),
        );
    }

    /// Process spent input with custom price_to_amount updates (for address cohorts).
    #[allow(clippy::too_many_arguments)]
    pub fn send_(
        &mut self,
        supply: &SupplyState,
        current_price: Option<Dollars>,
        prev_price: Option<Dollars>,
        blocks_old: usize,
        days_old: f64,
        older_than_hour: bool,
        price_to_amount_increment: Option<(Dollars, &SupplyState)>,
        price_to_amount_decrement: Option<(Dollars, &SupplyState)>,
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

                if let Some((price, supply)) = price_to_amount_increment
                    && supply.value.is_not_zero()
                {
                    self.price_to_amount
                        .as_mut()
                        .unwrap()
                        .increment(price, supply);

                    // Update cache for added supply
                    if let Some(cache) = self.cached_unrealized.as_mut() {
                        cache.on_receive(price, supply.value);
                    }
                }

                if let Some((price, supply)) = price_to_amount_decrement
                    && supply.value.is_not_zero()
                {
                    self.price_to_amount
                        .as_mut()
                        .unwrap()
                        .decrement(price, supply);

                    // Update cache for removed supply
                    if let Some(cache) = self.cached_unrealized.as_mut() {
                        cache.on_send(price, supply.value);
                    }
                }
            }
        }
    }

    /// Compute prices at percentile thresholds.
    pub fn compute_percentile_prices(&self) -> [Dollars; PERCENTILES_LEN] {
        match self.price_to_amount.as_ref() {
            Some(p) if !p.is_empty() => p.compute_percentiles(),
            _ => [Dollars::NAN; PERCENTILES_LEN],
        }
    }

    /// Compute unrealized profit/loss at current price.
    /// Uses O(k) incremental updates for height_price where k = flip range size.
    pub fn compute_unrealized_states(
        &mut self,
        height_price: Dollars,
        date_price: Option<Dollars>,
    ) -> (UnrealizedState, Option<UnrealizedState>) {
        let price_to_amount = match self.price_to_amount.as_ref() {
            Some(p) if !p.is_empty() => p,
            _ => {
                return (
                    UnrealizedState::NAN,
                    date_price.map(|_| UnrealizedState::NAN),
                );
            }
        };

        // Date unrealized: compute from scratch (only at date boundaries, ~144x less frequent)
        let date_state = date_price.map(|date_price| {
            CachedUnrealizedState::compute_full_standalone(date_price, price_to_amount)
        });

        // Height unrealized: use incremental cache (O(k) where k = flip range)
        let height_state = if let Some(cache) = self.cached_unrealized.as_mut() {
            cache.get_at_price(height_price, price_to_amount).clone()
        } else {
            let cache = CachedUnrealizedState::compute_fresh(height_price, price_to_amount);
            let state = cache.state.clone();
            self.cached_unrealized = Some(cache);
            state
        };

        (height_state, date_state)
    }

    /// Flush state to disk at checkpoint.
    pub fn write(&mut self, height: Height, cleanup: bool) -> Result<()> {
        if let Some(p) = self.price_to_amount.as_mut() {
            p.write(height, cleanup)?;
        }
        Ok(())
    }

    /// Get first (lowest) price in distribution.
    pub fn min_price(&self) -> Option<Dollars> {
        self.price_to_amount
            .as_ref()?
            .first_key_value()
            .map(|(k, _)| k)
    }

    /// Get last (highest) price in distribution.
    pub fn max_price(&self) -> Option<Dollars> {
        self.price_to_amount
            .as_ref()?
            .last_key_value()
            .map(|(k, _)| k)
    }

    /// Get iterator over price_to_amount for merged percentile computation.
    /// Returns None if price data is not tracked for this cohort.
    pub fn price_to_amount_iter(&self) -> Option<impl Iterator<Item = (Dollars, &Sats)>> {
        self.price_to_amount.as_ref().map(|p| p.iter())
    }
}
