use std::cmp::Ordering;

use brk_types::{CentsSats, CentsSquaredSats, Cents, Sats};

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
    /// Raw realized peak regret: Σ((peak - sell_price) × sats)
    peak_regret_raw: u128,
    /// Sats sent in profit
    sent_in_profit: Sats,
    /// Sats sent in loss
    sent_in_loss: Sats,
}

impl RealizedState {
    /// Get realized cap as CentsUnsigned (divides by ONE_BTC).
    #[inline]
    pub(crate) fn cap(&self) -> Cents {
        Cents::new((self.cap_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Set cap_raw directly from persisted value.
    #[inline]
    pub(crate) fn set_cap_raw(&mut self, cap_raw: CentsSats) {
        self.cap_raw = cap_raw.inner();
    }

    /// Set investor_cap_raw directly from persisted value.
    #[inline]
    pub(crate) fn set_investor_cap_raw(&mut self, investor_cap_raw: CentsSquaredSats) {
        self.investor_cap_raw = investor_cap_raw;
    }

    /// Get investor price as CentsUnsigned.
    /// investor_price = Σ(price² × sats) / Σ(price × sats)
    /// This is the dollar-weighted average acquisition price.
    #[inline]
    pub(crate) fn investor_price(&self) -> Cents {
        if self.cap_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.investor_cap_raw / self.cap_raw) as u64)
    }

    /// Get raw realized cap for aggregation.
    #[inline]
    pub(crate) fn cap_raw(&self) -> CentsSats {
        CentsSats::new(self.cap_raw)
    }

    /// Get raw investor cap for aggregation.
    #[inline]
    pub(crate) fn investor_cap_raw(&self) -> CentsSquaredSats {
        self.investor_cap_raw
    }

    /// Get realized profit as CentsUnsigned.
    #[inline]
    pub(crate) fn profit(&self) -> Cents {
        Cents::new((self.profit_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get realized loss as CentsUnsigned.
    #[inline]
    pub(crate) fn loss(&self) -> Cents {
        Cents::new((self.loss_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get profit value created as CentsUnsigned (sell_price × sats for profit cases).
    #[inline]
    pub(crate) fn profit_value_created(&self) -> Cents {
        Cents::new((self.profit_value_created_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get profit value destroyed as CentsUnsigned (cost_basis × sats for profit cases).
    /// This is also known as profit_flow.
    #[inline]
    pub(crate) fn profit_value_destroyed(&self) -> Cents {
        Cents::new((self.profit_value_destroyed_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get loss value created as CentsUnsigned (sell_price × sats for loss cases).
    #[inline]
    pub(crate) fn loss_value_created(&self) -> Cents {
        Cents::new((self.loss_value_created_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get loss value destroyed as CentsUnsigned (cost_basis × sats for loss cases).
    /// This is also known as capitulation_flow.
    #[inline]
    pub(crate) fn loss_value_destroyed(&self) -> Cents {
        Cents::new((self.loss_value_destroyed_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get realized peak regret as CentsUnsigned.
    /// This is Σ((peak - sell_price) × sats) - how much more could have been made
    /// by selling at peak instead of when actually sold.
    #[inline]
    pub(crate) fn peak_regret(&self) -> Cents {
        Cents::new((self.peak_regret_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get sats sent in profit.
    #[inline]
    pub(crate) fn sent_in_profit(&self) -> Sats {
        self.sent_in_profit
    }

    /// Get sats sent in loss.
    #[inline]
    pub(crate) fn sent_in_loss(&self) -> Sats {
        self.sent_in_loss
    }

    pub(crate) fn reset_single_iteration_values(&mut self) {
        self.profit_raw = 0;
        self.loss_raw = 0;
        self.profit_value_created_raw = 0;
        self.profit_value_destroyed_raw = 0;
        self.loss_value_created_raw = 0;
        self.loss_value_destroyed_raw = 0;
        self.peak_regret_raw = 0;
        self.sent_in_profit = Sats::ZERO;
        self.sent_in_loss = Sats::ZERO;
    }

    /// Increment using pre-computed values (for UTXO path)
    #[inline]
    pub(crate) fn increment(&mut self, price: Cents, sats: Sats) {
        if sats.is_zero() {
            return;
        }
        let price_sats = CentsSats::from_price_sats(price, sats);
        self.cap_raw += price_sats.as_u128();
        self.investor_cap_raw += price_sats.to_investor_cap(price);
    }

    /// Increment using pre-computed snapshot values (for address path)
    #[inline]
    pub(crate) fn increment_snapshot(&mut self, price_sats: CentsSats, investor_cap: CentsSquaredSats) {
        self.cap_raw += price_sats.as_u128();
        self.investor_cap_raw += investor_cap;
    }

    /// Decrement using pre-computed snapshot values (for address path)
    #[inline]
    pub(crate) fn decrement_snapshot(&mut self, price_sats: CentsSats, investor_cap: CentsSquaredSats) {
        self.cap_raw -= price_sats.as_u128();
        self.investor_cap_raw -= investor_cap;
    }

    #[inline]
    pub(crate) fn receive(&mut self, price: Cents, sats: Sats) {
        self.increment(price, sats);
    }

    /// Send with pre-computed typed values. Inlines decrement to avoid recomputation.
    #[inline]
    pub(crate) fn send(
        &mut self,
        sats: Sats,
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
                self.sent_in_profit += sats;
            }
            Ordering::Less => {
                self.loss_raw += (prev_ps - current_ps).as_u128();
                self.loss_value_created_raw += current_ps.as_u128();
                self.loss_value_destroyed_raw += prev_ps.as_u128();
                self.sent_in_loss += sats;
            }
            Ordering::Equal => {
                // Break-even: count as profit side (arbitrary but consistent)
                self.profit_value_created_raw += current_ps.as_u128();
                self.profit_value_destroyed_raw += prev_ps.as_u128();
                self.sent_in_profit += sats;
            }
        }

        // Track peak regret: (peak - sell_price) × sats
        self.peak_regret_raw += (ath_ps - current_ps).as_u128();

        // Inline decrement to avoid recomputation
        self.cap_raw -= prev_ps.as_u128();
        self.investor_cap_raw -= prev_investor_cap;
    }
}
