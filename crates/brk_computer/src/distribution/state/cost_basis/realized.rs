use std::cmp::Ordering;

use brk_types::{Cents, CentsSats, CentsSquaredSats, Sats};

/// Trait for realized state operations, implemented by both Core and Full variants.
/// Core skips extra fields (value_created/destroyed, peak_regret, sent_in_profit/loss, investor_cap).
pub trait RealizedOps: Default + Clone + Send + Sync + 'static {
    fn cap(&self) -> Cents;
    fn profit(&self) -> Cents;
    fn loss(&self) -> Cents;
    fn set_cap_raw(&mut self, cap_raw: CentsSats);
    fn set_investor_cap_raw(&mut self, investor_cap_raw: CentsSquaredSats);
    fn reset_single_iteration_values(&mut self);
    fn increment(&mut self, price: Cents, sats: Sats);
    fn increment_snapshot(&mut self, price_sats: CentsSats, investor_cap: CentsSquaredSats);
    fn decrement_snapshot(&mut self, price_sats: CentsSats, investor_cap: CentsSquaredSats);
    fn receive(&mut self, price: Cents, sats: Sats) {
        self.increment(price, sats);
    }
    fn send(
        &mut self,
        sats: Sats,
        current_ps: CentsSats,
        prev_ps: CentsSats,
        ath_ps: CentsSats,
        prev_investor_cap: CentsSquaredSats,
    );
}

/// Core realized state: only cap, profit, loss (48 bytes).
/// Used by CoreCohortMetrics and MinimalCohortMetrics cohorts
/// (epoch, class, amount_range, type_ — ~50 separate cohorts).
#[derive(Debug, Default, Clone)]
pub struct CoreRealizedState {
    cap_raw: u128,
    profit_raw: u128,
    loss_raw: u128,
}

impl RealizedOps for CoreRealizedState {
    #[inline]
    fn cap(&self) -> Cents {
        if self.cap_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.cap_raw / Sats::ONE_BTC_U128) as u64)
    }

    #[inline]
    fn profit(&self) -> Cents {
        if self.profit_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.profit_raw / Sats::ONE_BTC_U128) as u64)
    }

    #[inline]
    fn loss(&self) -> Cents {
        if self.loss_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.loss_raw / Sats::ONE_BTC_U128) as u64)
    }

    #[inline]
    fn set_cap_raw(&mut self, cap_raw: CentsSats) {
        self.cap_raw = cap_raw.inner();
    }

    #[inline]
    fn set_investor_cap_raw(&mut self, _investor_cap_raw: CentsSquaredSats) {
        // no-op for Core
    }

    #[inline]
    fn reset_single_iteration_values(&mut self) {
        self.profit_raw = 0;
        self.loss_raw = 0;
    }

    #[inline]
    fn increment(&mut self, price: Cents, sats: Sats) {
        if sats.is_zero() {
            return;
        }
        let price_sats = CentsSats::from_price_sats(price, sats);
        self.cap_raw += price_sats.as_u128();
    }

    #[inline]
    fn increment_snapshot(&mut self, price_sats: CentsSats, _investor_cap: CentsSquaredSats) {
        self.cap_raw += price_sats.as_u128();
    }

    #[inline]
    fn decrement_snapshot(&mut self, price_sats: CentsSats, _investor_cap: CentsSquaredSats) {
        self.cap_raw -= price_sats.as_u128();
    }

    #[inline]
    fn send(
        &mut self,
        _sats: Sats,
        current_ps: CentsSats,
        prev_ps: CentsSats,
        _ath_ps: CentsSats,
        _prev_investor_cap: CentsSquaredSats,
    ) {
        match current_ps.cmp(&prev_ps) {
            Ordering::Greater => {
                self.profit_raw += (current_ps - prev_ps).as_u128();
            }
            Ordering::Less => {
                self.loss_raw += (prev_ps - current_ps).as_u128();
            }
            Ordering::Equal => {}
        }
        self.cap_raw -= prev_ps.as_u128();
    }
}

/// Full realized state (~160 bytes).
/// Used by BasicCohortMetrics and CompleteCohortMetrics cohorts
/// (age_range — 21 separate cohorts).
#[derive(Debug, Default, Clone)]
pub struct RealizedState {
    core: CoreRealizedState,
    /// Raw investor cap: Σ(price² × sats)
    investor_cap_raw: CentsSquaredSats,
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

impl RealizedOps for RealizedState {
    #[inline]
    fn cap(&self) -> Cents {
        self.core.cap()
    }

    #[inline]
    fn profit(&self) -> Cents {
        self.core.profit()
    }

    #[inline]
    fn loss(&self) -> Cents {
        self.core.loss()
    }

    #[inline]
    fn set_cap_raw(&mut self, cap_raw: CentsSats) {
        self.core.set_cap_raw(cap_raw);
    }

    #[inline]
    fn set_investor_cap_raw(&mut self, investor_cap_raw: CentsSquaredSats) {
        self.investor_cap_raw = investor_cap_raw;
    }

    #[inline]
    fn reset_single_iteration_values(&mut self) {
        self.core.reset_single_iteration_values();
        self.profit_value_created_raw = 0;
        self.profit_value_destroyed_raw = 0;
        self.loss_value_created_raw = 0;
        self.loss_value_destroyed_raw = 0;
        self.peak_regret_raw = 0;
        self.sent_in_profit = Sats::ZERO;
        self.sent_in_loss = Sats::ZERO;
    }

    #[inline]
    fn increment(&mut self, price: Cents, sats: Sats) {
        if sats.is_zero() {
            return;
        }
        let price_sats = CentsSats::from_price_sats(price, sats);
        self.core.cap_raw += price_sats.as_u128();
        self.investor_cap_raw += price_sats.to_investor_cap(price);
    }

    #[inline]
    fn increment_snapshot(&mut self, price_sats: CentsSats, investor_cap: CentsSquaredSats) {
        self.core.cap_raw += price_sats.as_u128();
        self.investor_cap_raw += investor_cap;
    }

    #[inline]
    fn decrement_snapshot(&mut self, price_sats: CentsSats, investor_cap: CentsSquaredSats) {
        self.core.cap_raw -= price_sats.as_u128();
        self.investor_cap_raw -= investor_cap;
    }

    #[inline]
    fn send(
        &mut self,
        sats: Sats,
        current_ps: CentsSats,
        prev_ps: CentsSats,
        ath_ps: CentsSats,
        prev_investor_cap: CentsSquaredSats,
    ) {
        match current_ps.cmp(&prev_ps) {
            Ordering::Greater => {
                self.core.profit_raw += (current_ps - prev_ps).as_u128();
                self.profit_value_created_raw += current_ps.as_u128();
                self.profit_value_destroyed_raw += prev_ps.as_u128();
                self.sent_in_profit += sats;
            }
            Ordering::Less => {
                self.core.loss_raw += (prev_ps - current_ps).as_u128();
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
        self.core.cap_raw -= prev_ps.as_u128();
        self.investor_cap_raw -= prev_investor_cap;
    }
}

impl RealizedState {
    /// Get investor price as CentsUnsigned.
    /// investor_price = Σ(price² × sats) / Σ(price × sats)
    #[inline]
    pub(crate) fn investor_price(&self) -> Cents {
        if self.core.cap_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.investor_cap_raw / self.core.cap_raw) as u64)
    }

    /// Get raw realized cap for aggregation.
    #[inline]
    pub(crate) fn cap_raw(&self) -> CentsSats {
        CentsSats::new(self.core.cap_raw)
    }

    /// Get raw investor cap for aggregation.
    #[inline]
    pub(crate) fn investor_cap_raw(&self) -> CentsSquaredSats {
        self.investor_cap_raw
    }

    /// Get profit value created as CentsUnsigned (sell_price × sats for profit cases).
    #[inline]
    pub(crate) fn profit_value_created(&self) -> Cents {
        if self.profit_value_created_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.profit_value_created_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get profit value destroyed as CentsUnsigned (cost_basis × sats for profit cases).
    #[inline]
    pub(crate) fn profit_value_destroyed(&self) -> Cents {
        if self.profit_value_destroyed_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.profit_value_destroyed_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get loss value created as CentsUnsigned (sell_price × sats for loss cases).
    #[inline]
    pub(crate) fn loss_value_created(&self) -> Cents {
        if self.loss_value_created_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.loss_value_created_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get loss value destroyed as CentsUnsigned (cost_basis × sats for loss cases).
    #[inline]
    pub(crate) fn loss_value_destroyed(&self) -> Cents {
        if self.loss_value_destroyed_raw == 0 {
            return Cents::ZERO;
        }
        Cents::new((self.loss_value_destroyed_raw / Sats::ONE_BTC_U128) as u64)
    }

    /// Get realized peak regret as CentsUnsigned.
    #[inline]
    pub(crate) fn peak_regret(&self) -> Cents {
        if self.peak_regret_raw == 0 {
            return Cents::ZERO;
        }
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
}
