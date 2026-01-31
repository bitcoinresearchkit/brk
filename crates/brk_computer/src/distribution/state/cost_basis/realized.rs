use std::cmp::Ordering;

use brk_types::{CentsSats, CentsSquaredSats, CentsUnsigned, Sats};

/// Realized state using u128 for raw cent*sat values internally.
/// This avoids overflow and defers division to output time for efficiency.
#[derive(Debug, Default, Clone)]
pub struct RealizedState {
    /// Raw realized cap: Σ(price × sats)
    cap_raw: u128,
    /// Raw investor cap: Σ(price² × sats)
    /// investor_price = investor_cap_raw / cap_raw (gives cents directly)
    investor_cap_raw: CentsSquaredSats,
    /// Raw realized profit (cents * sats)
    profit_raw: u128,
    /// Raw realized loss (cents * sats)
    loss_raw: u128,
    /// sell_price × sats for profit cases
    profit_value_created_raw: u128,
    /// cost_basis × sats for profit cases
    profit_value_destroyed_raw: u128,
    /// sell_price × sats for loss cases
    loss_value_created_raw: u128,
    /// cost_basis × sats for loss cases (= capitulation_flow)
    loss_value_destroyed_raw: u128,
    /// Raw realized ATH regret: Σ((ath - sell_price) × sats)
    ath_regret_raw: u128,
}

impl RealizedState {
    /// Get realized cap as CentsUnsigned (divides by ONE_BTC).
    #[inline]
    pub fn cap(&self) -> CentsUnsigned {
        CentsUnsigned::new((self.cap_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Set cap_raw directly from persisted value.
    #[inline]
    pub fn set_cap_raw(&mut self, cap_raw: CentsSats) {
        self.cap_raw = cap_raw.inner();
    }

    /// Set investor_cap_raw directly from persisted value.
    #[inline]
    pub fn set_investor_cap_raw(&mut self, investor_cap_raw: CentsSquaredSats) {
        self.investor_cap_raw = investor_cap_raw;
    }

    /// Get investor price as CentsUnsigned.
    /// investor_price = Σ(price² × sats) / Σ(price × sats)
    /// This is the dollar-weighted average acquisition price.
    #[inline]
    pub fn investor_price(&self) -> CentsUnsigned {
        if self.cap_raw == 0 {
            return CentsUnsigned::ZERO;
        }
        CentsUnsigned::new((self.investor_cap_raw / self.cap_raw) as u64)
    }

    /// Get raw realized cap for aggregation.
    #[inline]
    pub fn cap_raw(&self) -> CentsSats {
        CentsSats::new(self.cap_raw)
    }

    /// Get raw investor cap for aggregation.
    #[inline]
    pub fn investor_cap_raw(&self) -> CentsSquaredSats {
        self.investor_cap_raw
    }

    /// Get realized profit as CentsUnsigned.
    #[inline]
    pub fn profit(&self) -> CentsUnsigned {
        CentsUnsigned::new((self.profit_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get realized loss as CentsUnsigned.
    #[inline]
    pub fn loss(&self) -> CentsUnsigned {
        CentsUnsigned::new((self.loss_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get value created as CentsUnsigned (derived from profit + loss splits).
    #[inline]
    pub fn value_created(&self) -> CentsUnsigned {
        let raw = self.profit_value_created_raw + self.loss_value_created_raw;
        CentsUnsigned::new((raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get value destroyed as CentsUnsigned (derived from profit + loss splits).
    #[inline]
    pub fn value_destroyed(&self) -> CentsUnsigned {
        let raw = self.profit_value_destroyed_raw + self.loss_value_destroyed_raw;
        CentsUnsigned::new((raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get profit value created as CentsUnsigned (sell_price × sats for profit cases).
    #[inline]
    pub fn profit_value_created(&self) -> CentsUnsigned {
        CentsUnsigned::new((self.profit_value_created_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get profit value destroyed as CentsUnsigned (cost_basis × sats for profit cases).
    /// This is also known as profit_flow.
    #[inline]
    pub fn profit_value_destroyed(&self) -> CentsUnsigned {
        CentsUnsigned::new((self.profit_value_destroyed_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get loss value created as CentsUnsigned (sell_price × sats for loss cases).
    #[inline]
    pub fn loss_value_created(&self) -> CentsUnsigned {
        CentsUnsigned::new((self.loss_value_created_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get loss value destroyed as CentsUnsigned (cost_basis × sats for loss cases).
    /// This is also known as capitulation_flow.
    #[inline]
    pub fn loss_value_destroyed(&self) -> CentsUnsigned {
        CentsUnsigned::new((self.loss_value_destroyed_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get capitulation flow as CentsUnsigned.
    /// This is the invested capital (cost_basis × sats) sold at a loss.
    /// Alias for loss_value_destroyed.
    #[inline]
    pub fn capitulation_flow(&self) -> CentsUnsigned {
        self.loss_value_destroyed()
    }

    /// Get profit flow as CentsUnsigned.
    /// This is the invested capital (cost_basis × sats) sold at a profit.
    /// Alias for profit_value_destroyed.
    #[inline]
    pub fn profit_flow(&self) -> CentsUnsigned {
        self.profit_value_destroyed()
    }

    /// Get realized ATH regret as CentsUnsigned.
    /// This is Σ((ath - sell_price) × sats) - how much more could have been made
    /// by selling at ATH instead of when actually sold.
    #[inline]
    pub fn ath_regret(&self) -> CentsUnsigned {
        CentsUnsigned::new((self.ath_regret_raw / Sats::ONE_BTC_U128) as u64)
    }

    pub fn reset_single_iteration_values(&mut self) {
        self.profit_raw = 0;
        self.loss_raw = 0;
        self.profit_value_created_raw = 0;
        self.profit_value_destroyed_raw = 0;
        self.loss_value_created_raw = 0;
        self.loss_value_destroyed_raw = 0;
        self.ath_regret_raw = 0;
    }

    /// Increment using pre-computed values (for UTXO path)
    #[inline]
    pub fn increment(&mut self, price: CentsUnsigned, sats: Sats) {
        if sats.is_zero() {
            return;
        }
        let price_sats = CentsSats::from_price_sats(price, sats);
        self.cap_raw += price_sats.as_u128();
        self.investor_cap_raw += price_sats.to_investor_cap(price);
    }

    /// Increment using pre-computed snapshot values (for address path)
    #[inline]
    pub fn increment_snapshot(&mut self, price_sats: CentsSats, investor_cap: CentsSquaredSats) {
        self.cap_raw += price_sats.as_u128();
        self.investor_cap_raw += investor_cap;
    }

    /// Decrement using pre-computed snapshot values (for address path)
    #[inline]
    pub fn decrement_snapshot(&mut self, price_sats: CentsSats, investor_cap: CentsSquaredSats) {
        self.cap_raw -= price_sats.as_u128();
        self.investor_cap_raw -= investor_cap;
    }

    #[inline]
    pub fn receive(&mut self, price: CentsUnsigned, sats: Sats) {
        self.increment(price, sats);
    }

    /// Send with pre-computed typed values. Inlines decrement to avoid recomputation.
    #[inline]
    pub fn send(
        &mut self,
        current_ps: CentsSats,
        prev_ps: CentsSats,
        ath_ps: CentsSats,
        prev_investor_cap: CentsSquaredSats,
    ) {
        match current_ps.cmp(&prev_ps) {
            Ordering::Greater => {
                self.profit_raw += (current_ps - prev_ps).as_u128();
                self.profit_value_created_raw += current_ps.as_u128();
                self.profit_value_destroyed_raw += prev_ps.as_u128();
            }
            Ordering::Less => {
                self.loss_raw += (prev_ps - current_ps).as_u128();
                self.loss_value_created_raw += current_ps.as_u128();
                self.loss_value_destroyed_raw += prev_ps.as_u128();
            }
            Ordering::Equal => {
                // Break-even: count as profit side (arbitrary but consistent)
                self.profit_value_created_raw += current_ps.as_u128();
                self.profit_value_destroyed_raw += prev_ps.as_u128();
            }
        }

        // Track ATH regret: (ath - sell_price) × sats
        self.ath_regret_raw += (ath_ps - current_ps).as_u128();

        // Inline decrement to avoid recomputation
        self.cap_raw -= prev_ps.as_u128();
        self.investor_cap_raw -= prev_investor_cap;
    }
}
